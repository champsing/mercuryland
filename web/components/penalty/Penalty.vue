<script setup lang="ts">
// TODO: Convert Penalty Page to SQL based from JSON based Data Storage
import { ref, onMounted } from "vue";
import { VaDateInput, VaInput, VaSelect, VaDivider } from "vuestic-ui";
import axios from "axios";
import penaltyStatus from "@assets/data/penalty_status.json";
import Table from "./Table.vue";
import Statistics from "./sidebar/Statistics.vue";
import Syntax from "./sidebar/Syntax.vue";
import News from "./sidebar/News.vue";
import Rule from "./Rule.vue";
import { formatDate, parseDate, PenItem, BASE_URL } from "@/composables/utils";
import ViewportHeight from "../ViewportHeight.vue";

document.title = "直播懲罰 - 水星人的夢幻樂園";

let filterDate = defineModel("filterDate", {
    default: {
        start: new Date(1672502400000),
        end: new Date(Date.now() + 28800000),
    },
});

let filterStatus = defineModel("filterStatus", {
    default: null,
    set(value) {
        return value;
    },
});

let filterSearch = defineModel("filterSearch", {
    default: "",
    set(value) {
        return value;
    },
});

let finishOptions = penaltyStatus.map((x) => x.name).sort();

const penalties = ref<PenItem[]>([]);

async function loadPenData() {
    try {
        const response = await axios.get<PenItem[]>(
            `${BASE_URL}/api/penalty/list`,
        );
        penalties.value = response.data;
        console.log("Penalty data loaded:", penalties.value);
    } catch (error) {
        console.error("Failed to load penalty data", error);
    }
}

onMounted(loadPenData);
</script>

<template>
    <div class="m-auto w-full z-10">
        <div
            class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2"
        >
            <div class="flex w-3/4 flex-row items-center gap-4">
                <div class="w-1/4 ml-12">
                    <VaDateInput
                        v-model="filterDate"
                        :format-date="formatDate"
                        :parse-date="parseDate"
                        manual-input
                        mode="auto"
                    />
                </div>
                <div class="w-3/4">
                    <VaInput
                        class="w-full"
                        placeholder="輸入懲罰內容來搜尋"
                        v-model="filterSearch"
                    />
                </div>
            </div>
            <div class="flex w-1/4 flex-row items-center gap-4">
                <div class="flex w-1/2 justify-center">
                    <VaSelect
                        v-model="filterStatus"
                        :options="finishOptions"
                        placeholder="完成狀態"
                        clearable
                        :clear-value="null"
                    />
                </div>
                <div class="flex w-1/2 justify-center">
                    <Rule />
                </div>
            </div>
        </div>

        <VaDivider class="!mt-0 !mb-2" />

        <ViewportHeight>
            <div class="flex flex-row gap-2 px-2 h-full">
                <div class="w-3/4 h-full">
                    <Table
                        :penalties="penalties"
                        :dateRange="filterDate"
                        :status="filterStatus"
                        :search="filterSearch"
                        @updateStatus="
                            (status) => {
                                filterStatus == null
                                    ? (filterStatus = status)
                                    : (filterStatus = null);
                            }
                        "
                    />
                </div>
                <div class="flex flex-col w-1/4 gap-2 h-full">
                    <div class="h-1/3">
                        <News />
                    </div>
                    <div class="h-1/3">
                        <Statistics />
                    </div>
                    <div class="h-1/3">
                        <Syntax />
                    </div>
                </div>
            </div>
        </ViewportHeight>
    </div>
</template>

<style>
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
</style>
