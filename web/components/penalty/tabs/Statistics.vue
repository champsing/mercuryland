<script setup lang="ts">
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { copyToClipboard } from "@/composables/utils";
import { computed, ref } from "vue";
import { VaButton, VaIcon, VaTextarea } from "vuestic-ui";
import StatisticsChart from "./StatisticsChart.vue";

interface StatState {
    state: number;
    items: PenItem[];
    count: number;
}

interface StatGroup {
    key: string;
    title: string;
    eyebrow: string;
    color: string;
    states: StatState[];
    count: number;
}

const props = defineProps<{ penalties: PenItem[] }>();

const kbdClass =
    "inline-block whitespace-nowrap rounded-[3px] border border-[#b4b4b4] bg-[#e3d0d0] px-1 py-0.5 text-[0.85em] font-bold leading-none text-[#0b17c3d4] shadow-[0_1px_1px_rgba(0,0,0,0.2),0_2px_0_0_rgba(255,255,255,0.7)_inset]";

// Each group's count tiles act as a state filter for its merged list.
const selectedStates = ref<Record<string, number | null>>({});

const groups = computed<StatGroup[]>(() =>
    [
        {
            key: "existing",
            title: "現存懲罰",
            eyebrow: "Existing",
            color: "#b73813",
            states: [1, 2],
        },
        {
            key: "done",
            title: "完成懲罰",
            eyebrow: "Done",
            color: "#297a33",
            states: [3, 4],
        },
    ].map(({ states, ...group }) => {
        const statStates = states.map((state) => {
            const items = props.penalties.filter((p) => p.state === state);
            return { state, items, count: items.length };
        });

        return {
            ...group,
            states: statStates,
            count: statStates.reduce((sum, stat) => sum + stat.count, 0),
        };
    }),
);

function activeStateOf(group: StatGroup): number | null {
    const selected = selectedStates.value[group.key] ?? null;
    return group.states.some((stat) => stat.state === selected)
        ? selected
        : null;
}

function toggleState(group: StatGroup, state: number) {
    selectedStates.value = {
        ...selectedStates.value,
        [group.key]: activeStateOf(group) === state ? null : state,
    };
}

function namesOf(items: PenItem[]): string {
    return items.map((item) => item.name).join("\n");
}

function visibleItems(group: StatGroup): PenItem[] {
    const state = activeStateOf(group);
    if (state == null) {
        return group.states.flatMap((stat) => stat.items);
    }
    return group.states.find((stat) => stat.state === state)?.items ?? [];
}

function visibleCount(group: StatGroup): number {
    const state = activeStateOf(group);
    if (state == null) return group.count;
    return group.states.find((stat) => stat.state === state)?.count ?? 0;
}

function listLabel(group: StatGroup): string {
    const state = activeStateOf(group);
    return state == null ? "合併顯示" : stateString(state);
}

function listLabelClass(group: StatGroup): string {
    const state = activeStateOf(group);
    return state == null ? "text-zinc-500" : stateColor(state, "text");
}

function copyLabel(group: StatGroup): string {
    const state = activeStateOf(group);
    return state == null ? "複製全部" : `複製${stateString(state)}`;
}

function copyGroup(group: StatGroup) {
    copyToClipboard(namesOf(visibleItems(group)));
}
</script>

<template>
    <div class="flex flex-col gap-4 pb-4">
        <div class="flex flex-col gap-4">
            <div class="flex flex-wrap items-baseline gap-x-2">
                <h2 class="text-xl font-bold text-neutral-100">統計</h2>
                <span
                    class="text-[0.72rem] font-extrabold uppercase text-[#45d483]"
                >
                    Statistics
                </span>
            </div>
            <div class="flex flex-wrap items-center justify-between gap-2">
                <h3 class="text-base font-extrabold text-[#f7f7f8]">數量</h3>
                <p
                    class="flex flex-wrap items-center gap-1.5 text-sm text-zinc-400"
                >
                    <span class="inline-flex items-center gap-1">
                        <kbd :class="kbdClass">Ctrl</kbd>
                        <kbd :class="kbdClass">A</kbd>
                    </span>
                    <span>可快速選取清單的全部項目</span>
                </p>
            </div>
        </div>

        <div class="grid gap-4 md:grid-cols-2">
            <section
                v-for="group in groups"
                :key="group.key"
                :aria-label="group.title"
                class="flex min-w-0 flex-col gap-3 rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.6)] p-4"
            >
                <header
                    class="flex flex-wrap items-center justify-between gap-2"
                >
                    <div class="flex flex-wrap items-baseline gap-x-2">
                        <h3 class="text-base font-extrabold text-[#f7f7f8]">
                            {{ group.title }}
                        </h3>
                        <span
                            class="text-[0.72rem] font-extrabold uppercase"
                            :style="{ color: group.color }"
                        >
                            {{ group.eyebrow }}
                        </span>
                    </div>
                    <VaButton
                        size="small"
                        class="rounded-lg"
                        :color="group.color"
                        gradient
                        :disabled="visibleCount(group) === 0"
                        @click="copyGroup(group)"
                    >
                        <VaIcon name="content_copy" size="small" class="mr-1" />
                        {{ copyLabel(group) }}
                    </VaButton>
                </header>

                <div
                    role="group"
                    :aria-label="`${group.title}狀態篩選`"
                    class="grid grid-cols-2 gap-2"
                >
                    <button
                        v-for="stat in group.states"
                        :key="stat.state"
                        type="button"
                        :aria-pressed="activeStateOf(group) === stat.state"
                        class="cursor-pointer rounded-xl border px-3 py-2 text-left transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                        :class="
                            activeStateOf(group) === stat.state
                                ? `${stateColor(stat.state, 'bg')} border-transparent`
                                : 'border-white/[0.06] bg-white/[0.03] hover:border-white/15 hover:bg-white/[0.06]'
                        "
                        @click="toggleState(group, stat.state)"
                    >
                        <span
                            class="block text-xs font-semibold"
                            :class="
                                activeStateOf(group) === stat.state
                                    ? 'text-white'
                                    : stateColor(stat.state, 'text')
                            "
                        >
                            {{ stateString(stat.state) }}
                        </span>
                        <span
                            class="mt-0.5 block text-2xl font-black leading-none text-[#f7f7f8] tabular-nums"
                        >
                            {{ stat.count }}
                        </span>
                    </button>
                </div>

                <div class="flex min-w-0 flex-col gap-1">
                    <div class="flex items-baseline justify-between gap-2">
                        <span
                            class="text-xs font-semibold"
                            :class="listLabelClass(group)"
                        >
                            {{ listLabel(group) }}
                        </span>
                        <span class="text-[0.7rem] text-zinc-500 tabular-nums">
                            共 {{ visibleCount(group) }} 筆
                        </span>
                    </div>
                    <VaTextarea
                        :input-aria-label="`${group.title}清單`"
                        :model-value="namesOf(visibleItems(group))"
                        :min-rows="8"
                        :max-rows="10"
                        :resize="false"
                        readonly
                        placeholder="暫無懲罰"
                    />
                </div>
            </section>
        </div>

        <StatisticsChart :penalties="penalties" />
    </div>
</template>
