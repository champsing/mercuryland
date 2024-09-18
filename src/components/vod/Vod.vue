<script setup lang="ts">
import { ref, Ref } from "vue";
import { NDatePicker, NGrid, NGi, NSelect, NDivider } from "naive-ui";
import vodLinkData from "@assets/data/vod.json";
import DataTable from "./DataTable.vue";
import TimeSummary from "./TimeSummary.vue";
import TimeDetail from "./TimeDetail.vue";

let dateRange: Ref<[number, number]> = ref([1577836800000, Date.now() + 28800000]);
let selectedTags = ref(null);

let tagMenu = [{ label: "", value: null }].concat(
    [...new Set(vodLinkData.flatMap((x) => x.tags))].sort().map((x) => {
        return { label: x, value: x };
    })
);
let computedTime = ref(0);
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="4" class="w-11/12" item-responsive>
        <n-gi span="4 800:2">
            <n-date-picker type="daterange" v-model:value="dateRange" />
        </n-gi>
        <n-gi span="4 800:2 1200:1">
            <n-select
                v-model:value="selectedTags"
                :options="tagMenu"
                multiple
                filterable
                placeholder="请选择直播的TAG"
                :consistent-menu-width="false"
            />
        </n-gi>
    </n-grid>

    <n-divider class="!mt-2 !mb-2" />

    <n-grid x-gap="8" :cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:2" class="w-full p-0 m-0">
            <DataTable
                :dateRange="dateRange"
                :selectedTags="selectedTags"
                @updateTag="
                    (tag) => {
                        if (selectedTags == null) selectedTags = []
                        else if (selectedTags.includes(tag)) return
                        else selectedTags.push(tag);
                    }
                "
            />
        </n-gi>
        <n-gi span="3 800:1">
            <TimeSummary :t="computedTime" />
            <TimeDetail
                :dateRange="dateRange"
                @computedTime="(time) => (computedTime = time)"
            />
        </n-gi>
    </n-grid>
</template>
