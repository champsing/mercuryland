<script setup lang="ts">
import vodSchedule from "@assets/data/schedule.json";
import { useAuthState } from "@/composables/authState";
import { notifyConnectionFailure } from "@/composables/connectionNotice";
import { sampleVods, withDevSamples } from "@/composables/sampleData";
import { nextTabIndex } from "@/composables/tabList";
import { formatDate, parseDate } from "@/composables/utils";
import { useBlockViewMode } from "@/composables/viewMode";
import {
    calculateVodTime,
    summarizeCalculation,
    VodItem,
} from "@/composables/vod";
import api from "@composables/axios";
import { useMediaQuery } from "@vueuse/core";
import { computed, nextTick, onMounted, ref, Ref } from "vue";
import { VaChip, VaDateInput, VaIcon, VaSelect, VaSwitch } from "vuestic-ui";
import SortMenu from "../SortMenu.vue";
import ViewModeToggle from "../ViewModeToggle.vue";
import ViewportHeight from "../ViewportHeight.vue";
import AddVod from "./AddVod.vue";
import Blocks from "./Blocks.vue";
import Rule from "./Rule.vue";
import SetVod from "./SetVod.vue";
import Calculation from "./time/Calculation.vue";
import Statistics from "./time/Statistics.vue";
import Summary from "./time/Summary.vue";

document.title = "直播隨選 - 水星人的夢幻樂園";

type VodTab = "list" | "hours" | "statistics";

const tabs: { name: VodTab; label: string }[] = [
    { name: "list", label: "列表" },
    { name: "hours", label: "時數" },
    { name: "statistics", label: "統計" },
];

const activeTab = ref<VodTab>("list");

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

function openHours() {
    activeTab.value = "hours";
    // The summary button unmounts with the list panel, so move focus to the
    // tab that took over the content instead of dropping it on the body.
    nextTick(() => document.getElementById("vod-tab-hours")?.focus());
}

const dateRange = defineModel<{ start: Date; end: Date }>("dateRange", {
    //1519833600 = 2018 03 01 12:00 AM Taipei ST; 8 hours = 28800 seconds
    default: () => ({
        start: new Date(1519833600000),
        end: new Date(Date.now() + 28800000),
    }),
});

const strictFiltering = ref(false);
const selectedTags: Ref<string[]> = ref([]);
const vodData = ref<VodItem[]>([]);

// 卡片（3 欄）或表格（單欄單列）檢視，選擇會記在 localStorage。
const viewMode = useBlockViewMode("vod");

const authState = useAuthState();
const showActions = computed(() => authState.isAuthenticated);

const sortField = ref<string>("date");
const sortDirection = ref<"asc" | "desc">("desc");
const sortOptions = [
    { value: "date", label: "日期", defaultDirection: "desc" as const },
    { value: "title", label: "名稱", defaultDirection: "asc" as const },
];

const tagList = computed(() =>
    [...new Set(vodData.value.flatMap((x) => x.tags))].sort(),
);

// ViewportHeight subtracts the page's bottom padding so the list ends exactly
// above the footer on desktop. On phones it stops at the copyright line and
// leaves the footer actions below the fold.
const pageRef = ref<HTMLElement | null>(null);
const filteredCount = ref(0);
const setVodRef = ref<{ open: (vod: VodItem) => void } | null>(null);
const addVodRef = ref<{ open: () => void } | null>(null);

// The ledger and the floating balance share one calculation.
const calculationEntries = computed(() =>
    calculateVodTime(vodData.value, vodSchedule),
);
const calculationTotals = computed(() =>
    summarizeCalculation(calculationEntries.value),
);
const computedTime = computed(() => calculationTotals.value.remaining);

// Phones collapse the filter panel and let the hours tab page-scroll; the
// ledger caps its own height there.
const isMobile = useMediaQuery("(max-width: 767px)");

// 手機上篩選選項預設收合，讓清單有更多空間。
const filterOpen = ref(false);

function applyVodData(items: VodItem[]) {
    vodData.value = items
        .map((item) => ({
            ...item,
            tags: item.tags ?? [],
        }))
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));
}

async function loadVodData() {
    try {
        const response = await api.get<VodItem[]>(`/api/video/list`);
        applyVodData(withDevSamples(response.data, sampleVods()));
    } catch (error) {
        console.error("Failed to load VOD data", error);
        notifyConnectionFailure();
        applyVodData(withDevSamples(vodData.value, sampleVods()));
    }
}

onMounted(loadVodData);

function updateTag(tag: string) {
    if (selectedTags.value.includes(tag)) {
        selectedTags.value = selectedTags.value.filter((x) => x !== tag);
    } else {
        selectedTags.value = [...selectedTags.value, tag];
    }
}

const handleEditVod = (vod: VodItem) => {
    setVodRef.value?.open(vod);
};
</script>

<template>
    <main ref="pageRef" class="vod-page bg-page-surface">
        <section class="w-full max-w-[1680px] mx-auto">
            <div
                class="mx-4 md:mx-20 mb-3 flex flex-wrap items-center gap-x-3 gap-y-3 md:gap-x-6"
            >
                <h1
                    class="shrink-0 whitespace-nowrap text-2xl font-bold text-neutral-100"
                >
                    直播隨選
                </h1>
                <p
                    v-if="activeTab === 'list'"
                    class="flex shrink-0 items-center gap-1 whitespace-nowrap text-sm text-zinc-400"
                >
                    <span>共</span>
                    <span
                        class="text-base font-bold leading-none text-zinc-100 tabular-nums"
                    >
                        {{ filteredCount }}
                    </span>
                    <span>筆直播</span>
                </p>
                <div
                    role="tablist"
                    aria-label="直播隨選分頁"
                    class="order-last flex w-full items-center gap-1 rounded-full border border-white/10 bg-white/5 p-1 shadow-[0_2px_12px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.08)] backdrop-blur-md lg:order-none lg:w-auto"
                >
                    <button
                        v-for="tab in tabs"
                        :id="`vod-tab-${tab.name}`"
                        :key="tab.name"
                        type="button"
                        role="tab"
                        :aria-selected="activeTab === tab.name"
                        :aria-controls="`vod-panel-${tab.name}`"
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
                    class="inline-flex h-8 shrink-0 cursor-pointer items-center justify-center gap-1.5 rounded-full border border-white/10 bg-[#5bc6a1] px-3 text-sm font-medium text-zinc-900 transition-colors hover:bg-[#3db088] focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60 lg:ml-auto lg:px-4"
                    aria-label="新增直播"
                    title="新增直播"
                    @click="addVodRef?.open()"
                >
                    <VaIcon name="add" size="small" aria-hidden="true" />
                    <span class="hidden lg:inline">新增直播</span>
                </button>
            </div>

            <section
                v-if="activeTab === 'list'"
                id="vod-panel-list"
                role="tabpanel"
                aria-labelledby="vod-tab-list"
                class="flex flex-col"
            >
                <div
                    class="mx-4 md:mx-20 mb-3 flex flex-col gap-3 md:flex-row md:items-stretch"
                >
                    <section
                        class="vod-filter-panel min-w-0 flex-1 rounded-3xl"
                        aria-label="直播隨選篩選"
                    >
                        <button
                            v-if="isMobile"
                            type="button"
                            class="flex w-full cursor-pointer items-center gap-2 rounded-2xl px-2 py-1.5 text-sm font-medium text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                            :aria-expanded="filterOpen"
                            aria-controls="vod-filter-bar"
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
                            id="vod-filter-bar"
                            class="vod-filter-bar"
                        >
                            <VaDateInput
                                v-model="dateRange"
                                :format-date="formatDate"
                                :parse-date="parseDate"
                                manual-input
                                mode="auto"
                                class="vod-date"
                            />
                            <VaSelect
                                v-model="selectedTags"
                                :options="tagList"
                                multiple
                                clearable
                                searchable
                                placeholder="請選擇直播的TAG"
                                dropdownIcon="va-plus"
                                class="vod-tags"
                            >
                                <template #content>
                                    <VaChip
                                        v-for="chip in selectedTags"
                                        :key="chip"
                                        color="#90dc52"
                                        outline
                                        size="small"
                                        class="mr-1 my-1"
                                        closeable
                                        @update:model-value="updateTag(chip)"
                                    >
                                        {{ chip }}
                                    </VaChip>
                                </template>
                            </VaSelect>
                            <div
                                class="flex w-full flex-wrap items-center justify-center gap-3 px-4"
                            >
                                <div class="vod-switch">
                                    <VaSwitch
                                        v-model="strictFiltering"
                                        off-color="#1ccba2"
                                        color="#3444a2"
                                        style="
                                            --va-switch-checker-background-color: #252723;
                                        "
                                        false-inner-label="符合一項"
                                        true-inner-label="符合全部"
                                    />
                                </div>
                                <div class="flex items-center gap-2">
                                    <div class="md:hidden">
                                        <ViewModeToggle v-model="viewMode" />
                                    </div>
                                    <SortMenu
                                        v-model:field="sortField"
                                        v-model:direction="sortDirection"
                                        :options="sortOptions"
                                    />
                                </div>
                            </div>
                        </div>
                    </section>
                    <div class="hidden items-center gap-2 md:flex">
                        <ViewModeToggle v-model="viewMode" />
                    </div>
                </div>

                <ViewportHeight
                    :scroll-container="pageRef"
                    footer-copyright-only
                    :min-height="isMobile ? 160 : undefined"
                >
                    <section
                        class="relative flex flex-col lg:flex-row gap-2 h-full px-4 md:px-20"
                    >
                        <div class="w-full h-full">
                            <Blocks
                                :dateRange="dateRange"
                                :selectedTags="selectedTags"
                                :strictFiltering="strictFiltering"
                                :vodData="vodData"
                                v-model:viewMode="viewMode"
                                v-model:sort-field="sortField"
                                v-model:sort-direction="sortDirection"
                                @updateTag="(tag: string) => updateTag(tag)"
                                @editVod="handleEditVod"
                                @countChange="filteredCount = $event"
                            />
                        </div>

                        <div
                            class="pointer-events-none absolute bottom-0 left-0 right-0 z-1 pl-4 pr-5 [&>*]:pointer-events-auto md:bottom-4 md:left-1/2 md:right-auto md:w-auto md:-translate-x-1/2 md:px-0"
                        >
                            <Summary
                                :t="computedTime"
                                @openCalculation="openHours"
                            />
                        </div>
                    </section>
                </ViewportHeight>
            </section>

            <section
                v-else-if="activeTab === 'hours'"
                id="vod-panel-hours"
                role="tabpanel"
                aria-labelledby="vod-tab-hours"
            >
                <div class="mx-4 md:mx-20">
                    <ViewportHeight
                        :scroll-container="pageRef"
                        :auto-height="isMobile"
                        :min-height="520"
                    >
                        <Calculation :entries="calculationEntries" />
                    </ViewportHeight>
                </div>
            </section>

            <section
                v-else
                id="vod-panel-statistics"
                role="tabpanel"
                aria-labelledby="vod-tab-statistics"
            >
                <div class="mx-4 md:mx-20">
                    <Statistics :vodData="vodData" />
                </div>
            </section>
        </section>
        <SetVod
            ref="setVodRef"
            :tag-list="tagList"
            @updated="loadVodData"
            @deleted="loadVodData"
        />
        <AddVod ref="addVodRef" :tag-list="tagList" @saved="loadVodData" />
    </main>
</template>

<style>
.vod-page {
    min-height: calc(100vh - var(--app-footer-height, 48px));
    min-height: calc(100dvh - var(--app-footer-height, 48px));
    color: #f7f7f8;
    padding: 1.25rem 1rem 1.125rem;
}

/* The app shell reserves 3.5rem above the page for the fixed desktop header.
   Without subtracting it the page overflows the viewport by exactly that
   amount, leaving dead space between the list and the footer. */
@media (min-width: 1024px) {
    .vod-page {
        min-height: calc(100vh - var(--app-footer-height, 48px) - 3.5rem);
        min-height: calc(100dvh - var(--app-footer-height, 48px) - 3.5rem);
    }
}

.vod-filter-panel {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: linear-gradient(
        135deg,
        rgba(28, 65, 84, 0.94) 0%,
        rgba(20, 70, 100, 0.92) 50%,
        rgba(10, 60, 90, 0.95) 100%
    );
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.3);
    padding: 0.8rem;
    backdrop-filter: blur(16px);
}

.vod-filter-bar {
    display: grid;
    grid-template-columns: minmax(13rem, 1fr) minmax(18rem, 2fr) auto;
    gap: 0.75rem;
    align-items: center;
}

.n-base-suffix__arrow {
    --n-arrow-size: 20px;
}

@media (max-width: 1180px) {
    .vod-filter-bar {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }
}

@media (max-width: 767px) {
    .vod-page {
        padding: 3.5rem 0.75rem 0.875rem;
    }

    .vod-filter-panel {
        padding: 0.7rem;
    }

    .vod-filter-bar {
        grid-template-columns: 1fr;
    }
}
</style>
