<script setup lang="ts">
import { useAuthState } from "@/composables/authState";
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { openLinks } from "@/composables/utils";
import { VodItem } from "@/composables/vod";
import api from "@composables/axios";
import { Play24Filled } from "@vicons/fluent";
import { computed, onMounted, ref } from "vue";
import { VaButton, VaChip, VaIcon, VaScrollContainer } from "vuestic-ui";
import SortMenu from "../SortMenu.vue";
import Detail from "./Detail.vue";

const props = defineProps<{
    penalties: PenItem[];
    dateRange: { start: Date; end: Date };
    state?: number | null;
    search: string;
}>();

const emit = defineEmits<{
    (e: "updateState", state: number): void;
    (e: "addPenalty"): void;
    (e: "editPenalty", penalty: PenItem): void;
}>();

const vodData = ref<VodItem[]>([]);

async function loadVodData() {
    try {
        const response = await api.get<VodItem[]>(`/api/video/list`);
        vodData.value = response.data;
    } catch (error) {
        console.error("Failed to load VOD data", error);
    }
}

onMounted(loadVodData);

const authState = useAuthState();
const showActions = computed(() => authState.isAuthenticated);

const YOUTUBE_LIVE = "https://youtube.com/live/";

const modal = ref(null as number | null);

const sortField = ref<string>("date");
const sortDirection = ref<"asc" | "desc">("desc");

const sortOptions = [
    { value: "date", label: "日期", defaultDirection: "desc" as const },
    { value: "name", label: "名稱", defaultDirection: "asc" as const },
    { value: "state", label: "狀態", defaultDirection: "asc" as const },
];

function comparePenalty(lhs: PenItem, rhs: PenItem): number {
    let result = 0;
    if (sortField.value === "name") {
        result = lhs.name.localeCompare(rhs.name, "zh-Hant");
    } else if (sortField.value === "state") {
        result = lhs.state - rhs.state;
    } else {
        result = lhs.date.localeCompare(rhs.date);
    }
    return sortDirection.value === "asc" ? result : -result;
}

const items = computed(() =>
    filterPenaltyData(
        props.penalties,
        props.dateRange,
        props.state,
        props.search,
    ).sort(comparePenalty),
);

const vodLinksByDate = computed(() => {
    const map = new Map<string, string[]>();

    for (const vod of vodData.value) {
        const links = map.get(vod.date) ?? [];
        links.push(`${YOUTUBE_LIVE}${vod.link}`);
        map.set(vod.date, links);
    }

    return map;
});

function vodLinksOfDate(date: string): string[] {
    return vodLinksByDate.value.get(date) ?? [];
}

function openVodOfDate(date: string) {
    openLinks(vodLinksOfDate(date));
}

function handleStateKeydown(event: KeyboardEvent, state: number) {
    if (event.key !== "Enter" && event.key !== " ") return;

    event.preventDefault();
    emit("updateState", state);
}

function filterPenaltyData(
    data: PenItem[],
    dateRange: { start: Date; end: Date },
    state: number | null,
    search: string,
): PenItem[] {
    return data
        .filter(
            (v) =>
                v.date >= dateRange.start.toISOString().slice(0, 10) &&
                v.date <=
                    new Date(dateRange.end.getTime() + 28800000)
                        .toISOString()
                        .slice(0, 10),
        )
        .filter((v) => state == null || state == v.state)
        .filter(
            (v) =>
                search == "" ||
                v.name.toLowerCase().includes(search.toLowerCase()),
        );
}
</script>

<template>
    <div class="flex h-full min-h-0 flex-col">
        <div class="mb-3 grid grid-cols-[1fr_auto_1fr] items-center gap-3 px-1">
            <span aria-hidden="true" />
            <p
                class="flex items-center justify-center gap-1 text-sm text-zinc-400"
            >
                <span>共</span>
                <span
                    class="text-base font-bold leading-none text-zinc-100 tabular-nums"
                >
                    {{ items.length }}
                </span>
                <span>筆懲罰</span>
            </p>
            <div class="flex items-center justify-end gap-2">
                <SortMenu
                    v-model:field="sortField"
                    v-model:direction="sortDirection"
                    :options="sortOptions"
                />
                <VaButton
                    v-if="showActions"
                    class="rounded-lg font-extrabold [&_.va-button__content]:gap-[0.35rem]"
                    color="#5865f2"
                    aria-label="新增懲罰"
                    @click="$emit('addPenalty')"
                >
                    <VaIcon name="add" />
                    <span class="max-md:hidden">新增懲罰</span>
                </VaButton>
            </div>
        </div>

        <VaScrollContainer
            vertical
            color="#8ea2ff"
            size="medium"
            class="flex-1 min-h-0"
        >
            <div
                v-if="items.length"
                class="grid w-full grid-cols-1 gap-3 pb-4 pr-1 md:grid-cols-2 lg:grid-cols-3"
            >
                <article
                    v-for="penalty in items"
                    :key="penalty.id"
                    class="flex flex-col gap-3 rounded-2xl border border-zinc-700/40 bg-zinc-800/40 p-8 backdrop-blur-sm transition-all duration-200 hover:-translate-y-0.5 hover:border-[#8ea2ff]/40 hover:bg-zinc-700/40 hover:shadow-[0_8px_32px_rgba(0,0,0,0.35)]"
                >
                    <header
                        class="flex min-w-0 items-start justify-between gap-3"
                    >
                        <div class="flex min-w-0 flex-col gap-1.5">
                            <span
                                class="flex items-center gap-1.5 text-base font-semibold tracking-wide text-[#8ea2ff] tabular-nums"
                            >
                                <VaIcon name="event" size="small" />
                                {{ penalty.date }}
                            </span>
                            <button
                                type="button"
                                class="line-clamp-2 cursor-pointer break-words border-0 bg-transparent p-0 text-left text-[22px] font-bold leading-snug text-white transition-colors hover:text-[#8ea2ff]"
                                :title="penalty.name"
                                @click="modal = penalty.id"
                            >
                                {{ penalty.name }}
                            </button>
                        </div>

                        <VaChip
                            outline
                            size="small"
                            role="button"
                            tabindex="0"
                            :color="stateColor(penalty.state, 'raw')"
                            class="shrink-0 cursor-pointer"
                            :title="`點擊篩選：${stateString(penalty.state)}`"
                            @click="emit('updateState', penalty.state)"
                            @keydown="handleStateKeydown($event, penalty.state)"
                        >
                            ● {{ stateString(penalty.state) }}
                        </VaChip>
                    </header>

                    <footer
                        class="mt-auto flex items-center justify-end gap-2 pt-1"
                    >
                        <button
                            v-if="showActions"
                            type="button"
                            class="inline-flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-[#a855f7] text-white shadow-md shadow-black/20 transition-colors hover:bg-[#9333ea] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#a855f7]"
                            aria-label="編輯懲罰"
                            title="編輯懲罰"
                            @click="emit('editPenalty', penalty)"
                        >
                            <VaIcon name="edit" size="small" />
                        </button>
                        <button
                            type="button"
                            :disabled="!vodLinksOfDate(penalty.date).length"
                            :aria-label="`觀看 ${penalty.date} 的直播`"
                            :title="
                                vodLinksOfDate(penalty.date).length
                                    ? `觀看 ${penalty.date} 的直播`
                                    : '此日期尚無直播影片'
                            "
                            class="inline-flex h-9 shrink-0 items-center justify-center gap-1.5 rounded-full bg-[#ff0000] px-4 text-sm font-semibold text-white shadow-md shadow-black/20 transition-colors hover:bg-[#e60000] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#ff0000] disabled:cursor-not-allowed disabled:bg-zinc-600 disabled:opacity-50 disabled:hover:bg-zinc-600"
                            @click="openVodOfDate(penalty.date)"
                        >
                            <VaIcon color="#ffffff" size="1.25rem">
                                <Play24Filled />
                            </VaIcon>
                            <span>播放</span>
                        </button>
                    </footer>
                </article>
            </div>

            <div
                v-else
                class="flex flex-col items-center justify-center py-20 text-zinc-500"
            >
                <VaIcon name="gavel" size="large" class="mb-3 opacity-60" />
                <p class="text-lg">沒有符合條件的懲罰</p>
                <p class="mt-1 text-sm">試著調整日期、狀態或搜尋關鍵字</p>
            </div>
        </VaScrollContainer>

        <Detail v-model="modal" />
    </div>
</template>
