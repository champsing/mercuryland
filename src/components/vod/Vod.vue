<script setup lang="ts">
import { ref, Ref } from "vue";
import { NDatePicker, NGrid, NGi, NSelect, NDivider } from "naive-ui";
import { VaSwitch } from "vuestic-ui";
import vodLinkData from "@assets/data/vod.json";
import DataTable from "./DataTable.vue";
import TimeSummary from "./TimeSummary.vue";
import TimeDetail from "./TimeDetail.vue";
import { Add24Regular } from "@vicons/fluent";

//prettier-ignore
let dateRange: Ref<[number, number]> = ref([1577836800000, Date.now() + 28800000]);

let strictFiltering = ref(false);

let isDownListOpened = ref(false);

let selectedTags: Ref<Array<string>> = ref(null);

let tagMenu: Array<{ label: string; value: any; disabled?: boolean }> = [
    { label: "", value: null },
].concat(
    [...new Set(vodLinkData.flatMap((x) => x.tags))].sort().map((x) => {
        return { label: x, value: x };
    })
);

tagMenu[0] = { label: "", value: null, disabled: true };

let computedTime = ref(0);
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="4" class="w-11/12" item-responsive>
        <n-gi span="4 800:2">
            <n-date-picker type="daterange" v-model:value="dateRange" />
        </n-gi>
        <n-gi span="4 800:2 1200:1">
            <n-select
                v-model:show="isDownListOpened"
                v-model:value="selectedTags"
                :options="tagMenu"
                multiple
                placeholder="请选择直播的TAG"
                :consistent-menu-width="false"
            >
                <template v-if="isDownListOpened" #arrow>
                    <Add24Regular />
                </template>
            </n-select>
        </n-gi>
        <n-gi>
            <VaSwitch
                class="ml-4"
                v-model="strictFiltering"
                off-color="#1ccba2"
                color="#3444a2"
                style="--va-switch-checker-background-color: #252723"
                false-inner-label="符合一項"
                true-inner-label="符合全部"
                disabled
            />
        </n-gi>
    </n-grid>

    <n-divider class="!mt-2 !mb-2" />

    <n-grid x-gap="8" :cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:2" class="w-full p-0 m-0">
            <DataTable
                :dateRange="dateRange"
                :selectedTags="selectedTags"
                :strictFiltering="strictFiltering"
                @updateTag="
                    (tag) => {
                        if (selectedTags == null) {
                            selectedTags = [];
                            selectedTags.push(tag);
                        } else if (selectedTags.includes(tag)) return;
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

<style>
.n-base-suffix__arrow {
    --n-arrow-size: 20px;
}
</style>