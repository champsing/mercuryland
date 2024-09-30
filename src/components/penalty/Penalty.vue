<script setup lang="ts">
import { Ref } from "vue";
import { NDatePicker, NDivider, NGrid, NGi, NSelect, NInput } from "naive-ui";
import penaltyStatus from "@assets/data/penalty_status.json";
import OverAllList from "./OverAllList.vue";
import PenaltyTable from "./Table.vue";
import PenaltySyntax from "./Syntax.vue";

let filterDate: Ref<[number, number]> = defineModel("filterDate", {
    default: [1672502400000, Date.now() + 28800000] as const,
    set(value: [number, number]) {
        return value;
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

let finishOptions = [{ label: "", value: null }].concat(
    penaltyStatus
        .map((x) => x.name)
        .sort()
        .map((x) => {
            return { label: x, value: x };
        })
);

</script>

<script lang="ts"></script>

<template>
    <n-grid x-gap="12" :cols="4" class="w-11/12">
        <n-gi :span="2">
            <n-date-picker type="daterange" v-model:value="filterDate" />
        </n-gi>
        <n-gi>
            <n-select
                v-model:value="filterStatus"
                :options="finishOptions"
                placeholder="請選擇一種完成狀態"
                :consistent-menu-width="false"
            />
        </n-gi>
        <n-gi>
            <n-input
                round
                placeholder="輸入懲罰內容來搜尋"
                v-model:value="filterSearch"
                type="text"
            />
        </n-gi>
    </n-grid>

    <n-divider class="!m-2" />

    <n-grid x-gap="12" :cols="3" class="w-11/12 h-80vh overflow-y-hidden">
        <n-gi :span="2" class="h-80vh w-full p-0 m-0 overflow-y-scroll">
            <PenaltyTable
                :dateRange="filterDate"
                :status="filterStatus"
                :search="filterSearch"
                @updateStatus="
                    (status) => {
                        if (filterStatus == null) filterStatus = status;
                        else filterStatus = null;
                    }
                "
            />
        </n-gi>
        <n-gi>
            <OverAllList />
        </n-gi>
    </n-grid>

    <div class="mt-8">
        <PenaltySyntax />
    </div>

    <n-divider class="mt-2" />
</template>

<style>
.vachip2 {
    margin-top: 0.5rem;
    margin-bottom: 1rem;
    --va-chip-border: 0.2rem solid transparent;
    --va-chip-font-size: 1.15rem;
    --va-chip-padding: 0 1.2rem;
}

.overall-button {
    margin-top: 0.65rem;
    --va-button-group-font-size: 1.15rem;
    --va-button-group-border-radius: 2px;
    --va-button-group-button-padding: 0.3rem;
    --va-button-group-button-width: 90px;
}

.same-line {
    display: inline-block;
    white-space: nowrap;
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
