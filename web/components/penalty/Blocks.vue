<script setup lang="ts">
import { useAuthState } from "@/composables/authState";
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { openLinks } from "@/composables/utils";
import { BlockViewMode } from "@/composables/viewMode";
import { VodItem } from "@/composables/vod";
import api from "@composables/axios";
import { useMediaQuery } from "@vueuse/core";
import { Play24Filled } from "@vicons/fluent";
import { computed, onMounted, ref } from "vue";
import { VaButton, VaIcon, VaScrollContainer } from "vuestic-ui";
import SortMenu from "../SortMenu.vue";
import ViewModeToggle from "../ViewModeToggle.vue";
import Detail from "./Detail.vue";

const props = defineProps<{
    penalties: PenItem[];
    dateRange: { start: Date; end: Date };
    state?: number | null;
    search: string;
}>();

const viewMode = defineModel<BlockViewMode>("viewMode", { default: "grid" });

// 手機寬度一律用列表檢視：卡片與列表在窄螢幕上幾乎相同，列表更省捲動。
// 這裡只影響顯示，不會覆寫使用者在桌面上儲存的偏好。
const isDesktop = useMediaQuery("(min-width: 768px)");
const effectiveViewMode = computed<BlockViewMode>(() =>
    isDesktop.value ? viewMode.value : "list",
);

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
        <div
            class="mb-3 flex w-full flex-wrap items-center justify-between gap-2 px-1 md:grid md:grid-cols-[1fr_auto_1fr] md:items-center md:gap-3"
        >
            <span aria-hidden="true" class="hidden md:block" />
            <p
                class="flex items-center gap-1 text-sm text-zinc-400 md:justify-center"
            >
                <span>共</span>
                <span
                    class="text-base font-bold leading-none text-zinc-100 tabular-nums"
                >
                    {{ items.length }}
                </span>
                <span>筆懲罰</span>
            </p>
            <div class="ml-auto flex flex-wrap items-center justify-end gap-2">
                <ViewModeToggle v-model="viewMode" class="max-md:hidden" />
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
                v-if="items.length && effectiveViewMode === 'grid'"
                class="grid w-full grid-cols-1 gap-3 pb-4 pr-1 md:grid-cols-2 lg:grid-cols-3"
            >
                <article
                    v-for="penalty in items"
                    :key="penalty.id"
                    tabindex="0"
                    class="flex cursor-pointer flex-col gap-3 rounded-2xl border border-zinc-700/40 bg-zinc-800/40 p-8 backdrop-blur-sm transition-all duration-200 hover:-translate-y-0.5 hover:border-[#8ea2ff]/40 hover:bg-zinc-700/40 hover:shadow-[0_8px_32px_rgba(0,0,0,0.35)]"
                    @click="modal = penalty.id"
                    @keydown.enter.self="modal = penalty.id"
                    @keydown.space.self.prevent="modal = penalty.id"
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
                            <div
                                class="line-clamp-2 break-words text-[22px] font-bold leading-snug text-white"
                                :title="penalty.name"
                            >
                                {{ penalty.name }}
                            </div>
                        </div>

                        <button
                            type="button"
                            :class="stateColor(penalty.state, 'bg')"
                            class="shrink-0 cursor-pointer rounded-lg border-0 px-2.5 py-1 text-sm font-bold leading-none text-white shadow-sm shadow-black/25 transition-opacity hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#8ea2ff]"
                            :title="`點擊篩選：${stateString(penalty.state)}`"
                            @click.stop="emit('updateState', penalty.state)"
                        >
                            {{ stateString(penalty.state) }}
                        </button>
                    </header>

                    <footer
                        class="mt-auto flex items-center justify-end gap-2 pt-1"
                    >
                        <button
                            v-if="showActions"
                            type="button"
                            class="inline-flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-[#7e22ce] text-white shadow-md shadow-black/20 transition-colors hover:bg-[#6b21a8] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#a855f7]"
                            aria-label="編輯懲罰"
                            title="編輯懲罰"
                            @click.stop="emit('editPenalty', penalty)"
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
                            class="inline-flex h-9 shrink-0 items-center justify-center gap-1.5 rounded-full bg-[#d90000] px-4 text-sm font-semibold text-white shadow-md shadow-black/20 transition-colors hover:bg-[#b30000] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#ff0000] disabled:cursor-not-allowed disabled:bg-zinc-600 disabled:opacity-50 disabled:hover:bg-zinc-600"
                            @click.stop="openVodOfDate(penalty.date)"
                        >
                            <VaIcon color="#ffffff" size="1.25rem">
                                <Play24Filled />
                            </VaIcon>
                            <span>播放</span>
                        </button>
                    </footer>
                </article>
            </div>

            <div v-else-if="items.length" class="w-full pb-4 pr-1">
                <div
                    class="mb-1 hidden grid-cols-[10rem_minmax(0,1fr)_6rem_2.25rem] items-center gap-3 px-4 text-xs font-semibold tracking-wide text-zinc-500 md:grid"
                    aria-hidden="true"
                >
                    <span>日期</span>
                    <span>懲罰</span>
                    <span class="text-center">狀態</span>
                    <span />
                </div>

                <div class="flex w-full flex-col gap-1.5">
                    <article
                        v-for="penalty in items"
                        :key="penalty.id"
                        tabindex="0"
                        class="grid cursor-pointer grid-cols-[minmax(0,1fr)_auto_auto] items-center gap-x-3 gap-y-1.5 rounded-xl border border-zinc-700/40 bg-zinc-800/40 px-4 py-2.5 backdrop-blur-sm transition-colors hover:border-[#8ea2ff]/40 hover:bg-zinc-700/40 md:grid-cols-[10rem_minmax(0,1fr)_6rem_2.25rem] md:gap-3"
                        @click="modal = penalty.id"
                        @keydown.enter.self="modal = penalty.id"
                        @keydown.space.self.prevent="modal = penalty.id"
                    >
                        <div
                            class="col-start-1 row-start-1 flex min-w-0 items-center gap-2"
                        >
                            <span
                                class="flex items-center gap-1.5 text-sm font-semibold tracking-wide text-[#8ea2ff] tabular-nums"
                            >
                                <VaIcon name="event" size="small" />
                                {{ penalty.date }}
                            </span>
                            <button
                                type="button"
                                :disabled="!vodLinksOfDate(penalty.date).length"
                                :aria-label="`觀看 ${penalty.date} 的直播`"
                                :title="
                                    vodLinksOfDate(penalty.date).length
                                        ? `觀看 ${penalty.date} 的直播`
                                        : '此日期尚無直播影片'
                                "
                                class="inline-flex h-7 w-7 shrink-0 cursor-pointer items-center justify-center rounded-full bg-[#d90000] text-white shadow-sm shadow-black/25 transition-colors hover:bg-[#b30000] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#ff0000] disabled:cursor-not-allowed disabled:bg-zinc-600 disabled:opacity-50 disabled:hover:bg-zinc-600"
                                @click.stop="openVodOfDate(penalty.date)"
                            >
                                <VaIcon color="#ffffff" size="0.9rem">
                                    <Play24Filled />
                                </VaIcon>
                            </button>
                        </div>

                        <div
                            class="col-span-3 row-start-2 truncate text-base font-bold text-white md:col-span-1 md:col-start-2 md:row-start-1"
                            :title="penalty.name"
                        >
                            {{ penalty.name }}
                        </div>

                        <button
                            type="button"
                            :class="stateColor(penalty.state, 'bg')"
                            class="col-start-2 row-start-1 shrink-0 cursor-pointer justify-self-end rounded-lg border-0 px-2.5 py-1 text-xs font-bold leading-none text-white shadow-sm shadow-black/25 transition-opacity hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#8ea2ff] md:col-start-3 md:row-start-1"
                            :title="`點擊篩選：${stateString(penalty.state)}`"
                            @click.stop="emit('updateState', penalty.state)"
                        >
                            {{ stateString(penalty.state) }}
                        </button>

                        <button
                            v-if="showActions"
                            type="button"
                            class="col-start-3 row-start-1 inline-flex h-9 w-9 shrink-0 cursor-pointer items-center justify-center justify-self-end rounded-full bg-[#7e22ce] text-white shadow-md shadow-black/20 transition-colors hover:bg-[#6b21a8] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#a855f7] md:col-start-4 md:row-start-1"
                            aria-label="編輯懲罰"
                            title="編輯懲罰"
                            @click.stop="emit('editPenalty', penalty)"
                        >
                            <VaIcon name="edit" size="small" />
                        </button>
                    </article>
                </div>
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
