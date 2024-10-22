<script setup lang="ts">
import { VaDateInput, VaDivider, VaInput, VaSelect } from "vuestic-ui";
import penaltyStatus from "@assets/data/penalty_status.json";
import OverAllList from "./OverAllList.vue";
import PenaltyTable from "./Table.vue";
import PenaltySyntax from "./Syntax.vue";

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

function formatDate(date: Date) {
    return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
}

function parseDate(text) {
    const [year, month, day] = text.split("-");
    return new Date(year, month - 1, day);
}
</script>

<template>
    <div class="mt-4 m-auto w-11/12">
        <div class="flex flex-row w-full justify-center gap-10">
            <div class="w-1/5">
                <VaDateInput
                    v-model="filterDate"
                    :format-date="formatDate"
                    :parse-date="parseDate"
                    manual-input
                    mode="auto"
                />
            </div>
            <div class="w-1/5">
                <VaSelect
                    v-model="filterStatus"
                    :options="finishOptions"
                    placeholder="請選擇一種完成狀態"
                    clearable
                    :clear-value="null"
                />
            </div>
            <div class="w-1/5">
                <VaInput
                    placeholder="輸入懲罰內容來搜尋"
                    v-model="filterSearch"
                />
            </div>
        </div>

        <VaDivider class="!m-2" />

        <div class="flex flex-row">
            <div class="h-80vh w-2/3 p-0 m-0 overflow-y-scroll">
                <PenaltyTable
                    class=""
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
            <OverAllList />
        </div>
        
        <div class="mt-8 mb-6">
            <PenaltySyntax />
        </div>
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
    box-shadow: 0 1px 1px rgba(0, 0, 0, 0.2),
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
