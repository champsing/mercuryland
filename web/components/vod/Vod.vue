<script setup lang="ts">
import { ref, Ref } from "vue";
import {
    VaButton,
    VaChip,
    VaDateInput,
    VaDivider,
    VaIcon,
    VaModal,
    VaSelect,
    VaSwitch,
} from "vuestic-ui";
import vodLinkData from "@assets/data/vod.json";
import DataTable from "./DataTable.vue";
import TimeSummary from "./TimeSummary.vue";
import TimeDetail from "./TimeDetail.vue";
import { Info24Regular } from "@vicons/fluent";

let dateRange = defineModel("dateRange", {
    //1582992000 = 2020 03 01 12:00 AM Taipei ST; 8 hours = 28800 seconds
    default: {
        start: new Date(1582992000000),
        end: new Date(Date.now() + 28800000),
    },
});

let strictFiltering = ref(false);

let selectedTags: Ref<Array<string>> = ref(null);

let tagList = [...new Set(vodLinkData.flatMap((x) => x.tags))].sort();

let computedTime = ref(0);

const showRuleDescModal = ref(false);
const showVodDescImg = ref(false);
// const monthNames = [
//     "一月",
//     "二月",
//     "三月",
//     "四月",
//     "五月",
//     "六月",
//     "七月",
//     "八月",
//     "九月",
//     "十月",
//     "十一月",
//     "十二月",
// ];

function tagAlreadyExist(tag: string) {
    selectedTags.value = selectedTags.value.filter((x) => x !== tag);
    if (selectedTags.value.toString() == new Array().toString())
        selectedTags.value = null;
}

function updateTag(tag: string) {
    if (selectedTags.value == null) {
        selectedTags.value = new Array();
        selectedTags.value.push(tag);
    } else if (selectedTags.value.includes(tag)) tagAlreadyExist(tag);
    else selectedTags.value.push(tag);
}

function formatDate(date: Date) {
    return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
}

function parseDate(text) {
    const [year, month, day] = text.split("-");
    return new Date(year, month - 1, day);
}
</script>

<template>
    <div class="mt-4 m-auto w-11/12 z-10">
        <div class="flex flex-row w-full justify-center gap-10">
            <div class="w-1/8 flex flex-row">
                <VaDateInput
                    v-model="dateRange"
                    :format-date="formatDate"
                    :parse-date="parseDate"
                    manual-input
                    mode="auto"
                />
                <!--[DONE] need more adjusting -->
            </div>
            <div class="w-2/5">
                <VaSelect
                    class="w-full"
                    v-model="selectedTags"
                    :options="tagList"
                    multiple
                    clearable
                    placeholder="请选择直播的TAG"
                    dropdownIcon="va-plus"
                    @update:model-value="
                        if (selectedTags.toString() == new Array().toString())
                            selectedTags = null;
                    "
                >
                    <!-- [SOLVED] has a problem, sometimes the first entry won't show as 
                    a chip until the second entry is selected and removed. -->
                    <!-- [SOLUTION] by setting selectTags to null every time it becomes an empty array. -->
                    <template #content>
                        <VaChip
                            v-for="chip in selectedTags"
                            color="#90dc52"
                            outline
                            size="small"
                            class="mr-1 my-1"
                            closeable
                            @update:model-value="tagAlreadyExist(chip)"
                        >
                            {{ chip }}
                        </VaChip>
                    </template>
                </VaSelect>
            </div>
            <!-- not yet -->
            <div class="w-1/8">
                <VaSwitch
                    v-model="strictFiltering"
                    off-color="#1ccba2"
                    color="#3444a2"
                    style="--va-switch-checker-background-color: #252723"
                    false-inner-label="符合一項"
                    true-inner-label="符合全部"
                    screen-responsive
                />
            </div>
            <div class="w-1/8">
                <VaButton
                    preset="plain"
                    color="#FFFFFF"
                    class="mt-1"
                    @click="showRuleDescModal = !showRuleDescModal"
                >
                    <VaIcon size="large" class="mr-2">
                        <Info24Regular />
                    </VaIcon>
                    <div class="text-lg text-center">規則說明</div>
                </VaButton>
            </div>
        </div>

        <!-- 規則說明 -->
        <VaModal
            v-model="showRuleDescModal"
            title="規則說明"
            hide-default-actions
            z-index="4"
        >
            <span class="text-3xl"> 直播時數規則說明 </span>
            <div class="text-2xl mt-2">●概述</div>
            <div class="text-bg mt-2">
                惡靈公布直播紀錄檔時，此處會同步更新計算加班台的剩餘時數，並標上當天遊玩的遊戲，供使用者藉由上方的標籤篩選功能找到自己想看的遊戲直播。
            </div>
            <div class="text-2xl mt-2">●時數計算說明</div>
            <div class="text-bg mt-2">
                在計算明細表中，會以「計劃」、「直播」等項目的時數互相加減得出剩餘時數。
                <br />
                <ol class="ml-3">
                    <li>
                        <!-- 1. -->
                        「計劃」為惡靈的常規直播時數，目前落在2小時左右，因此以2小時計算。
                    </li>
                    <li>
                        <!-- 2. -->
                        每次直播的時數沒意外的話以YouTube影片時長為準。若直播紀錄檔被和諧了，則以2小時計算。
                    </li>
                    <li>
                        <!-- 3. -->
                        有時在直播懲罰會生成加班台懲罰，此處也會以「懲罰」項目來增加剩餘的加班台時數。
                    </li>
                    <li>
                        <!-- 4. -->
                        若有其他因素導致時數加減也會以獨立項目處理。
                    </li>
                    <li>
                        <!-- 5. -->
                        伺服器時間每週三 00:00 會在計算明細表生成一項「計劃」。
                    </li>
                </ol>
                <VaButton
                    class="mt-2"
                    preset="primary"
                    border-color="info"
                    color="info"
                    gradient
                    @click="showVodDescImg = !showVodDescImg"
                >
                    查看說明圖例
                </VaButton>
            </div>

            <div class="text-2xl mt-2">●不可抗力因素</div>
            <div class="text-bg mt-2">
                若惡靈在直播過程中斷網或停電，則中間嘗試恢復的多次黑畫面直播紀錄檔將不會被採計，直到恢復1分鐘以上的穩定直播為止。
            </div>
        </VaModal>

        <VaModal v-model="showVodDescImg" hide-default-actions z-index="5">
            <img src="@assets/images/vod_time.png" alt="直播時數規則說明" />
        </VaModal>

        <VaDivider class="!mt-2 !mb-2" />

        <div class="flex flex-row gap-2">
            <div class="w-2/3">
                <DataTable
                    :dateRange="dateRange"
                    :selectedTags="selectedTags"
                    :strictFiltering="strictFiltering"
                    @updateTag="(tag) => updateTag(tag)"
                />
            </div>

            <div class="flex flex-col w-1/3">
                <TimeSummary :t="computedTime" />
                <TimeDetail
                    :dateRange="dateRange"
                    @computedTime="(time) => (computedTime = time)"
                />
            </div>
        </div>
    </div>
</template>

<style>
.n-base-suffix__arrow {
    --n-arrow-size: 20px;
}

.vod-card {
    background-color: var(--va-background-element) !important;
    --va-card-box-shadow: 0px;
}
</style>
