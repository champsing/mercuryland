<script setup lang="ts">
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { copyToClipboard } from "@/composables/utils";
import { computed } from "vue";
import { VaButton, VaIcon } from "vuestic-ui";
import StatisticsChart from "./StatisticsChart.vue";

interface StatGroup {
    key: string;
    title: string;
    eyebrow: string;
    color: string;
    items: PenItem[];
    states: { state: number; count: number }[];
}

const props = defineProps<{ penalties: PenItem[] }>();

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
        // Keep the API order so the copied list matches the old modal.
        const items = props.penalties.filter((p) => states.includes(p.state));

        return {
            ...group,
            items,
            states: states.map((state) => ({
                state,
                count: items.filter((p) => p.state === state).length,
            })),
        };
    }),
);

function copyGroup(group: StatGroup) {
    copyToClipboard(group.items.map((x) => x.name).join("\n"));
}
</script>

<template>
    <div class="flex flex-col gap-4 pb-4">
        <div class="flex flex-wrap items-baseline gap-x-2">
            <h2 class="text-xl font-bold text-neutral-100">統計</h2>
            <span
                class="text-[0.72rem] font-extrabold uppercase text-[#45d483]"
            >
                Statistics
            </span>
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
                        :disabled="group.items.length === 0"
                        @click="copyGroup(group)"
                    >
                        <VaIcon name="content_copy" size="small" class="mr-1" />
                        複製全部
                    </VaButton>
                </header>

                <dl class="grid grid-cols-2 gap-2">
                    <div
                        v-for="stat in group.states"
                        :key="stat.state"
                        class="rounded-xl border border-white/[0.06] bg-white/[0.03] px-3 py-2"
                    >
                        <dt
                            class="text-xs font-semibold"
                            :class="stateColor(stat.state, 'text')"
                        >
                            {{ stateString(stat.state) }}
                        </dt>
                        <dd
                            class="mt-0.5 text-2xl font-black leading-none text-[#f7f7f8] tabular-nums"
                        >
                            {{ stat.count }}
                        </dd>
                    </div>
                </dl>

                <div
                    class="overflow-hidden rounded-xl border border-white/[0.06] bg-black/20"
                >
                    <div
                        class="border-b border-white/[0.06] px-3 py-1.5 text-xs text-zinc-500"
                    >
                        共 {{ group.items.length }} 筆
                    </div>
                    <ul
                        v-if="group.items.length"
                        class="max-h-56 overflow-y-auto overscroll-contain px-1 py-1"
                    >
                        <li
                            v-for="item in group.items"
                            :key="item.id"
                            class="flex items-center gap-2 rounded-lg px-2 py-1.5 text-sm"
                        >
                            <span
                                class="shrink-0 text-xs text-zinc-500 tabular-nums"
                            >
                                #{{ item.id }}
                            </span>
                            <span
                                class="min-w-0 flex-1 truncate text-zinc-200"
                                :title="item.name"
                            >
                                {{ item.name }}
                            </span>
                            <span
                                class="shrink-0 text-xs text-zinc-500 tabular-nums"
                            >
                                {{ item.date }}
                            </span>
                        </li>
                    </ul>
                    <p
                        v-else
                        class="px-3 py-6 text-center text-sm text-zinc-500"
                    >
                        暫無{{ group.title }}
                    </p>
                </div>
            </section>
        </div>

        <StatisticsChart :penalties="penalties" />
    </div>
</template>
