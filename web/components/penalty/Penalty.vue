<script setup lang="ts">
import { PenItem, stateString } from "@/composables/penalty";
import { formatDate, parseDate } from "@/composables/utils";
import { useBlockViewMode } from "@/composables/viewMode";
import api from "@composables/axios";
import { useMediaQuery } from "@vueuse/core";
import { onMounted, ref } from "vue";
import { VaDateInput, VaIcon, VaInput, VaSelect } from "vuestic-ui";
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

// 卡片（3 欄）或表格（單欄單列）檢視，選擇會記在 localStorage。
const viewMode = useBlockViewMode("penalty");

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
            <div class="mb-4 flex flex-wrap items-center gap-x-6 gap-y-3">
                <h1 class="text-2xl font-bold text-neutral-100">直播懲罰</h1>
                <Rule />
                <div
                    role="tablist"
                    aria-label="直播懲罰分頁"
                    class="flex items-center gap-1 rounded-full border border-white/10 bg-white/5 p-1 shadow-[0_2px_12px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.08)] backdrop-blur-md"
                >
                    <button
                        v-for="tab in tabs"
                        :id="`penalty-tab-${tab.name}`"
                        :key="tab.name"
                        type="button"
                        role="tab"
                        :aria-selected="activeTab === tab.name"
                        :aria-controls="`penalty-panel-${tab.name}`"
                        class="rounded-full px-4 py-1.5 text-sm font-medium text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                        :class="
                            activeTab === tab.name
                                ? 'bg-gradient-to-b from-white/20 to-white/10 text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.2),0_1px_3px_rgba(0,0,0,0.25)]'
                                : ''
                        "
                        @click="activeTab = tab.name"
                    >
                        {{ tab.label }}
                    </button>
                </div>
            </div>

            <section
                v-if="activeTab === 'list'"
                id="penalty-panel-list"
                role="tabpanel"
                aria-labelledby="penalty-tab-list"
                class="flex flex-col"
            >
                <LatestPenalty :penalties="penalties" class="mb-3" />

                <section
                    class="penalty-filter-bar grid gap-3 items-center mb-3 rounded-3xl"
                    aria-label="懲罰篩選"
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
                    <VaSelect
                        v-model="filterStatus"
                        :options="finishOptions"
                        placeholder="完成狀態"
                        :text-by="textByFinish"
                        :value-by="valueByFinish"
                        clearable
                        :clear-value="null"
                        class="penalty-status"
                    />
                </section>

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
                                @updateState="
                                    (state) => {
                                        filterStatus == null
                                            ? (filterStatus = state)
                                            : (filterStatus = null);
                                    }
                                "
                                @addPenalty="addPenaltyRef?.open()"
                                @editPenalty="setPenaltyRef?.open($event)"
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

.penalty-filter-bar {
    grid-template-columns: minmax(13rem, 0.9fr) minmax(16rem, 1.8fr) minmax(
            11rem,
            0.8fr
        );
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

/* The penalty date filter intentionally has no calendar trigger. */
.penalty-date .va-input-wrapper__field > .va-icon {
    display: none;
}

/* 表格在自己的捲動區內捲動，捲到底不會帶動整個頁面。 */
.penalty-viewport .va-scroll-container {
    overscroll-behavior: contain;
}

@media (max-width: 900px) {
    .penalty-filter-bar {
        grid-template-columns: repeat(2, minmax(0, 1fr));
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

    .penalty-filter-bar {
        grid-template-columns: 1fr;
        padding: 0.7rem;
    }
}
</style>
