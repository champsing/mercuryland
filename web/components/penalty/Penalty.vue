<script setup lang="ts">
import { PenItem, stateString } from "@/composables/penalty";
import { useCollapsed } from "@/composables/collapsible";
import { formatDate, parseDate } from "@/composables/utils";
import { useBlockViewMode } from "@/composables/viewMode";
import api from "@composables/axios";
import { useElementSize, useMediaQuery } from "@vueuse/core";
import { onMounted, ref } from "vue";
import { VaDateInput, VaIcon, VaInput, VaSelect } from "vuestic-ui";
import ViewportHeight from "../ViewportHeight.vue";
import AddPenalty from "./AddPenalty.vue";
import Blocks from "./Blocks.vue";
import Rule from "./Rule.vue";
import EditPenalty from "./edit/EditPenalty.vue";
import News from "./header/News.vue";
import Statistics from "./header/Statistics.vue";
import Syntax from "./header/Syntax.vue";

document.title = "直播懲罰 - 水星人的夢幻樂園";

let filterDate = defineModel<{ start: Date; end: Date }>("filterDate", {
    default: () => ({
        start: new Date(1672502400000),
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

// 手機上懲罰頁自己是捲動容器；量測快速面板高度，讓表格高度不隨面板開合改變。
const pageRef = ref<HTMLElement | null>(null);
const panelsRef = ref<HTMLElement | null>(null);
const { height: panelsHeight } = useElementSize(
    panelsRef,
    { width: 0, height: 0 },
    { box: "border-box" },
);

// 短螢幕手機上允許清單縮小，讓頁面仍能剛好貼合螢幕高度。
const isMobile = useMediaQuery("(max-width: 720px)");

// 三個快速面板（最新懲罰 / 統計 / 語法）可整組收合，狀態同樣會記住。
const panelsCollapsed = useCollapsed("penalty-quick-panels");

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
    <main ref="pageRef" class="penalty-page">
        <section class="penalty-shell w-full max-w-[1680px] mx-auto">
            <div class="mb-2 flex justify-end px-3 md:px-6">
                <button
                    type="button"
                    class="inline-flex cursor-pointer items-center gap-1.5 rounded-lg border border-white/10 bg-zinc-900/60 px-3 py-1 text-xs font-semibold text-zinc-300 transition-colors hover:border-[#8ea2ff]/40 hover:bg-zinc-800/70 hover:text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#8ea2ff]"
                    :aria-expanded="!panelsCollapsed"
                    aria-controls="penalty-quick-panels"
                    @click="panelsCollapsed = !panelsCollapsed"
                >
                    <VaIcon
                        :name="panelsCollapsed ? 'unfold_more' : 'unfold_less'"
                        size="small"
                    />
                    <span>{{
                        panelsCollapsed ? "展開快速面板" : "收合快速面板"
                    }}</span>
                </button>
            </div>

            <Transition name="quick-panels">
                <section
                    v-show="!panelsCollapsed"
                    ref="panelsRef"
                    id="penalty-quick-panels"
                    class="penalty-quick-panels grid gap-4 pb-4 px-3 md:px-6"
                    aria-label="懲罰快速面板"
                >
                    <News :penalties="penalties" />
                    <Statistics :penalties="penalties" />
                    <Syntax />
                </section>
            </Transition>

            <section
                class="penalty-filter-bar grid gap-3 items-center mb-4 mx-3 md:mx-10 rounded-3xl"
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
                <div class="penalty-rule flex justify-end">
                    <Rule />
                </div>
            </section>

            <div class="penalty-viewport">
                <ViewportHeight
                    :scroll-container="pageRef"
                    :extra-height="panelsHeight"
                    :min-height="isMobile ? 160 : undefined"
                >
                    <section
                        class="penalty-main flex h-full min-h-0 w-full flex-col px-3 md:px-6"
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
    background:
        radial-gradient(
            circle at 15% 0%,
            rgba(88, 166, 255, 0.2),
            transparent 32rem
        ),
        linear-gradient(135deg, #0f1117 0%, #16191f 48%, #101318 100%);
    color: #f7f7f8;
    padding: 2.5rem 1rem 1.25rem;
}

.penalty-quick-panels {
    grid-template-columns: minmax(18rem, 1.15fr) repeat(2, minmax(16rem, 1fr));
}

.quick-panels-enter-active,
.quick-panels-leave-active {
    transition:
        opacity 0.18s ease,
        transform 0.18s ease;
}

.quick-panels-enter-from,
.quick-panels-leave-to {
    opacity: 0;
    transform: translateY(-0.4rem);
}

.penalty-quick-panels .latest-card__content,
.penalty-quick-panels .side-card__content {
    gap: 0.4rem;
    padding: 0.85rem !important;
}

.penalty-quick-panels .latest-release__title {
    -webkit-line-clamp: 2;
    font-size: 1rem;
}

.penalty-filter-bar {
    grid-template-columns:
        minmax(13rem, 0.9fr) minmax(16rem, 1.8fr) minmax(11rem, 0.8fr)
        auto;
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

/* 表格在自己的捲動區內捲動，捲到底不會帶動整個頁面。 */
.penalty-viewport .va-scroll-container {
    overscroll-behavior: contain;
}

.overall-button {
    margin-top: 0.65rem;
    --va-button-group-font-size: 1.15rem;
    --va-button-group-border-radius: 2px;
    --va-button-group-button-padding: 0.3rem;
    --va-button-group-button-width: 90px;
}

kbd {
    background-color: #e3d0d0;
    border-radius: 3px;
    border: 1px solid #b4b4b4;
    box-shadow:
        0 1px 1px rgba(0, 0, 0, 0.2),
        0 2px 0 0 rgba(255, 255, 255, 0.7) inset;
    color: #0b17c3d4;
    display: inline-block;
    font-size: 0.85em;
    font-weight: 700;
    line-height: 1;
    padding: 2px 4px;
    white-space: nowrap;
}

@media (max-width: 1180px) {
    .penalty-quick-panels {
        grid-template-columns: repeat(3, minmax(0, 1fr));
    }
}

@media (max-width: 900px) {
    .penalty-filter-bar {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .penalty-rule {
        justify-content: stretch;
    }

    .penalty-quick-panels {
        grid-template-columns: 1fr;
    }
}

@media (max-width: 720px) {
    .penalty-page {
        padding: 4rem 0.75rem 1rem;
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
