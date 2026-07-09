<script setup lang="ts">
import { PenItem, stateString } from "@/composables/penalty";
import { formatDate, parseDate } from "@/composables/utils";
import api from "@composables/axios";
import { onMounted, ref } from "vue";
import { VaDateInput, VaIcon, VaInput, VaSelect } from "vuestic-ui";
import AddPenalty from "./AddPenalty.vue";
import Rule from "./Rule.vue";
import Table from "./Table.vue";
import EditPenalty from "./edit/EditPenalty.vue";
import News from "./header/News.vue";
import Statistics from "./header/Statistics.vue";
import Syntax from "./header/Syntax.vue";

document.title = "直播懲罰 - 水星人的夢幻樂園";

let filterDate = defineModel("filterDate", {
    default: {
        start: new Date(1672502400000),
        end: new Date(Date.now() + 28800000),
    },
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
    <main class="penalty-page">
        <section class="penalty-shell">
            <section class="penalty-quick-panels" aria-label="懲罰快速面板">
                <News :penalties="penalties" />
                <Statistics :penalties="penalties" />
                <Syntax />
            </section>

            <section class="penalty-filter-bar" aria-label="懲罰篩選">
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
                <div class="penalty-rule">
                    <Rule />
                </div>
            </section>

            <ViewportHeight>
                <section class="penalty-main">
                    <Table
                        :penalties="penalties"
                        :dateRange="filterDate"
                        :state="filterStatus"
                        :search="filterSearch"
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
    min-height: calc(100vh - 48px);
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

.penalty-shell {
    width: min(1680px, 100%);
    margin: 0 auto;
}

.penalty-quick-panels {
    display: grid;
    grid-template-columns: minmax(18rem, 1.15fr) repeat(2, minmax(16rem, 1fr));
    gap: 1rem;
    margin-bottom: 1rem;
}

.penalty-quick-panels .latest-card__content,
.penalty-quick-panels .side-card__content {
    min-height: 8.25rem;
    gap: 0.7rem;
    padding: 0.85rem !important;
}

.penalty-quick-panels .latest-release__title {
    -webkit-line-clamp: 2;
    font-size: 1rem;
}

.penalty-filter-bar {
    display: grid;
    grid-template-columns:
        minmax(13rem, 0.9fr) minmax(16rem, 1.8fr) minmax(11rem, 0.8fr)
        auto;
    gap: 0.75rem;
    align-items: center;
    margin-bottom: 1rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    background: rgba(22, 25, 31, 0.86);
    padding: 0.8rem;
    backdrop-filter: blur(16px);
}

.penalty-rule {
    display: flex;
    justify-content: flex-end;
}

.penalty-main {
    min-height: 0;
    width: 100%;
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
    }

    .penalty-filter-bar {
        grid-template-columns: 1fr;
        padding: 0.7rem;
    }
}
</style>
