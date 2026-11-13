<script setup lang="ts">
import { useAuthState } from "@/composables/authState";
import { PenItem, stateString } from "@/composables/penalty";
import { nextTabIndex } from "@/composables/tabList";
import { formatDate, parseDate } from "@/composables/utils";
import { useBlockViewMode } from "@/composables/viewMode";
import api from "@composables/axios";
import { useMediaQuery } from "@vueuse/core";
import { computed, onMounted, ref } from "vue";
import { VaDateInput, VaIcon, VaInput, VaSelect } from "vuestic-ui";
import SortMenu from "../SortMenu.vue";
import ViewModeToggle from "../ViewModeToggle.vue";
import ViewportHeight from "../ViewportHeight.vue";
import AddPenalty from "./AddPenalty.vue";
import Blocks from "./Blocks.vue";
import Rule from "./Rule.vue";
import EditPenalty from "./edit/EditPenalty.vue";
import LatestPenalty from "./tabs/LatestPenalty.vue";
import Legend from "./tabs/Legend.vue";
import Statistics from "./tabs/Statistics.vue";

document.title = "直播懲罰 - 水星人的夢幻樂園";

type PenaltyTab = "list" | "statistics" | "legend";

const tabs: { name: PenaltyTab; label: string }[] = [
    { name: "list", label: "列表" },
    { name: "statistics", label: "統計" },
    { name: "legend", label: "圖例" },
];

const activeTab = ref<PenaltyTab>("list");

// 手機上篩選選項預設收合，讓清單有更多空間。
const filterOpen = ref(false);

let filterDate = defineModel<{ start: Date; end: Date }>("filterDate", {
    default: () => ({
        // 2018-03-01 00:00 Taipei, matching the VOD filter default.
        start: new Date(1519833600000),
        end: new Date(Date.now() + 28800000),
    }),
});

let filterStatus = defineModel("filterStatus", {
    default: null as number | null,
});

let filterSearch = defineModel("filterSearch", {
    default: "" as string,
});

let finishOptions = [
    { valueBy: 0, textBy: stateString(0) },
    { valueBy: 1, textBy: stateString(1) },
    { valueBy: 2, textBy: stateString(2) },
    { valueBy: 3, textBy: stateString(3) },
    { valueBy: 4, textBy: stateString(4) },
];

const textByFinish = (option: { textBy: string }) => option.textBy;
const valueByFinish = (option: { valueBy: number }) => option.valueBy;

const penalties = ref<PenItem[]>([]);
const listCount = ref(0);

// 卡片（3 欄）或表格（單欄單列）檢視，選擇會記在 localStorage。
const viewMode = useBlockViewMode("penalty");

const authState = useAuthState();
const showActions = computed(() => authState.isAuthenticated);

function onTabKeydown(
    event: KeyboardEvent,
    current: number,
    count: number,
    select: (index: number) => void,
) {
    const next = nextTabIndex(event.key, current, count);
    if (next === null) return;

    event.preventDefault();
    select(next);

    const tablist = (event.currentTarget as HTMLElement).parentElement;
    tablist?.querySelectorAll<HTMLElement>('[role="tab"]')[next]?.focus();
}

// Demo: list presentation controls live in the filter bar on desktop.
const sortField = ref<string>("date");
const sortDirection = ref<"asc" | "desc">("desc");
const sortOptions = [
    { value: "date", label: "日期", defaultDirection: "desc" as const },
    { value: "name", label: "名稱", defaultDirection: "asc" as const },
    { value: "state", label: "狀態", defaultDirection: "asc" as const },
];

// 手機上懲罰頁自己是捲動容器，ViewportHeight 需要量測它。
const pageRef = ref<HTMLElement | null>(null);

// 短螢幕手機上允許清單縮小，讓頁面仍能剛好貼合螢幕高度。
const isMobile = useMediaQuery("(max-width: 720px)");

const addPenaltyRef = ref<{ open: () => void } | null>(null);
const setPenaltyRef = ref<{ open: (penalty: PenItem) => void } | null>(null);

async function loadPenData() {
    try {
        const response = await api.get<PenItem[]>(`/api/penalty/list`);
        penalties.value = response.data;
        console.log("Penalty data loaded:", penalties.value);
    } catch (error) {
        console.error("Failed to load penalty data", error);
    }
}

onMounted(loadPenData);
</script>

<template>
    <main ref="pageRef" class="penalty-page bg-page-surface">
        <section class="penalty-shell page-content">
            <div
                class="mb-4 flex flex-wrap items-center gap-x-4 gap-y-3 sm:gap-x-6"
            >
                <div class="flex min-w-0 items-center gap-3">
                    <h1 class="shrink-0 text-2xl font-bold text-neutral-100">
                        直播懲罰
                    </h1>
                    <p
                        v-if="activeTab !== 'legend'"
                        class="flex items-center gap-1 whitespace-nowrap text-sm text-zinc-400"
                    >
                        <span>共</span>
                        <span
                            class="text-base font-bold leading-none text-zinc-100 tabular-nums"
                        >
                            {{ listCount }}
                        </span>
                        <span class="hidden sm:inline">筆懲罰</span>
                        <span class="sm:hidden">筆</span>
                    </p>
                </div>
                <div
                    role="tablist"
                    aria-label="直播懲罰分頁"
                    class="order-last flex w-full items-center gap-1 rounded-full border border-white/10 bg-white/5 p-1 shadow-[0_2px_12px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.08)] backdrop-blur-md lg:order-none lg:w-auto"
                >
                    <button
                        v-for="tab in tabs"
                        :id="`penalty-tab-${tab.name}`"
                        :key="tab.name"
                        type="button"
                        role="tab"
                        :aria-selected="activeTab === tab.name"
                        :aria-controls="`penalty-panel-${tab.name}`"
                        :tabindex="activeTab === tab.name ? 0 : -1"
                        class="flex-1 rounded-full px-4 py-1.5 text-sm font-medium text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60 lg:flex-none"
                        :class="
                            activeTab === tab.name
                                ? 'bg-gradient-to-b from-white/20 to-white/10 text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.2),0_1px_3px_rgba(0,0,0,0.25)]'
                                : ''
                        "
                        @click="activeTab = tab.name"
                        @keydown="
                            onTabKeydown(
                                $event,
                                tabs.findIndex(
                                    (item) => item.name === activeTab,
                                ),
                                tabs.length,
                                (index) => (activeTab = tabs[index].name),
                            )
                        "
                    >
                        {{ tab.label }}
                    </button>
                </div>
                <Rule class="max-lg:ml-auto" />
                <button
                    v-if="showActions"
                    type="button"
                    class="inline-flex h-8 shrink-0 cursor-pointer items-center justify-center gap-1.5 rounded-full border border-white/10 bg-[#5865f2] px-3 text-sm font-medium text-white transition-colors hover:bg-[#4752c4] focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60 lg:ml-auto lg:px-4"
                    aria-label="新增懲罰"
                    title="新增懲罰"
                    @click="addPenaltyRef?.open()"
                >
                    <VaIcon name="add" size="small" aria-hidden="true" />
                    <span class="hidden lg:inline">新增懲罰</span>
                </button>
            </div>

            <section
                v-if="activeTab === 'list'"
                id="penalty-panel-list"
                role="tabpanel"
                aria-labelledby="penalty-tab-list"
                class="flex flex-col"
            >
                <LatestPenalty :penalties="penalties" class="mb-3" />

                <div
                    class="mb-3 flex flex-col gap-3 md:flex-row md:items-stretch"
                >
                    <section
                        class="penalty-filter-panel flex min-w-0 flex-1 flex-col gap-2 rounded-3xl"
                        aria-label="懲罰篩選"
                    >
                        <button
                            v-if="isMobile"
                            type="button"
                            class="flex w-full cursor-pointer items-center gap-2 rounded-2xl px-2 py-1.5 text-sm font-medium text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                            :aria-expanded="filterOpen"
                            aria-controls="penalty-filter-bar"
                            @click="filterOpen = !filterOpen"
                        >
                            <VaIcon
                                name="filter_alt"
                                size="small"
                                aria-hidden="true"
                            />
                            <span>篩選選項</span>
                            <VaIcon
                                :name="
                                    filterOpen ? 'expand_less' : 'expand_more'
                                "
                                size="small"
                                class="ml-auto"
                                aria-hidden="true"
                            />
                        </button>
                        <div
                            v-show="!isMobile || filterOpen"
                            id="penalty-filter-bar"
                            class="penalty-filter-bar grid gap-3 items-center"
                        >
                            <VaDateInput
                                v-model="filterDate"
                                :format-date="formatDate"
                                :parse-date="parseDate"
                                manual-input
                                mode="auto"
                                class="penalty-date"
                            />
                            <VaInput
                                v-model="filterSearch"
                                class="penalty-search"
                                placeholder="搜尋懲罰內容"
                            >
                                <template #prependInner>
                                    <VaIcon name="search" size="small" />
                                </template>
                            </VaInput>
                            <div
                                class="penalty-status-row flex items-center gap-2"
                            >
                                <VaSelect
                                    v-model="filterStatus"
                                    :options="finishOptions"
                                    placeholder="完成狀態"
                                    :text-by="textByFinish"
                                    :value-by="valueByFinish"
                                    clearable
                                    :clear-value="null"
                                    class="penalty-status min-w-0 flex-1"
                                />
                                <SortMenu
                                    v-model:field="sortField"
                                    v-model:direction="sortDirection"
                                    :options="sortOptions"
                                />
                            </div>
                        </div>
                    </section>
                    <div class="hidden items-center gap-2 md:flex">
                        <ViewModeToggle v-model="viewMode" />
                    </div>
                </div>

                <div class="penalty-viewport">
                    <ViewportHeight
                        :scroll-container="pageRef"
                        :min-height="isMobile ? 160 : undefined"
                    >
                        <section
                            class="penalty-main flex h-full min-h-0 w-full flex-col"
                        >
                            <Blocks
                                :penalties="penalties"
                                :dateRange="filterDate"
                                :state="filterStatus"
                                :search="filterSearch"
                                v-model:viewMode="viewMode"
                                v-model:sort-field="sortField"
                                v-model:sort-direction="sortDirection"
                                @updateState="
                                    (state) => {
                                        filterStatus == null
                                            ? (filterStatus = state)
                                            : (filterStatus = null);
                                    }
                                "
                                @editPenalty="setPenaltyRef?.open($event)"
                                @countChange="listCount = $event"
                            />
                        </section>
                    </ViewportHeight>
                </div>
            </section>

            <section
                v-else-if="activeTab === 'statistics'"
                id="penalty-panel-statistics"
                role="tabpanel"
                aria-labelledby="penalty-tab-statistics"
            >
                <Statistics :penalties="penalties" />
            </section>

            <section
                v-else
                id="penalty-panel-legend"
                role="tabpanel"
                aria-labelledby="penalty-tab-legend"
            >
                <Legend />
            </section>
        </section>
        <AddPenalty ref="addPenaltyRef" @saved="loadPenData" />
        <EditPenalty
            ref="setPenaltyRef"
            @updated="loadPenData"
            @deleted="loadPenData"
        />
    </main>
</template>

<style>
.penalty-page {
    min-height: calc(100vh - var(--app-footer-height, 48px));
    color: #f7f7f8;
    padding: 1.25rem 1rem 1.25rem;
}

.penalty-filter-panel {
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: linear-gradient(
        135deg,
        rgba(33, 70, 76, 0.94) 0%,
        rgba(24, 77, 96, 0.92) 50%,
        rgba(8, 72, 78, 0.95) 100%
    );
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.3);
    padding: 0.8rem;
    backdrop-filter: blur(16px);
}

.penalty-filter-bar {
    grid-template-columns: minmax(13rem, 0.9fr) minmax(16rem, 1.8fr) minmax(
            13rem,
            0.9fr
        );
}

/* 表格在自己的捲動區內捲動，捲到底不會帶動整個頁面。 */
.penalty-viewport .va-scroll-container {
    overscroll-behavior: contain;
}

@media (max-width: 900px) {
    .penalty-filter-bar {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    /* The date range needs the full row at tablet widths; splitting it with
       the search field truncates the second date. */
    .penalty-filter-bar .penalty-date {
        grid-column: 1 / -1;
    }
}

@media (max-width: 720px) {
    .penalty-page {
        padding: 3.5rem 0.75rem 1rem;
        height: calc(100vh - var(--app-footer-height, 48px));
        height: calc(100dvh - var(--app-footer-height, 48px));
        min-height: 0;
        overflow-y: auto;
        overscroll-behavior: contain;
    }

    .penalty-filter-panel {
        padding: 0.7rem;
    }

    .penalty-filter-bar {
        grid-template-columns: 1fr;
    }
}
</style>
