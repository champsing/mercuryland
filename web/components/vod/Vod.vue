<script setup lang="ts">
import { ref, Ref } from "vue";
import { NGrid, NGi, NSelect } from "naive-ui";
import {
    VaButton,
    VaCollapse,
    VaDatePicker,
    VaDivider,
    VaIcon,
    VaModal,
    VaSwitch,
} from "vuestic-ui";
import vodLinkData from "@assets/data/vod.json";
import DataTable from "./DataTable.vue";
import TimeSummary from "./TimeSummary.vue";
import TimeDetail from "./TimeDetail.vue";
import { Add24Regular, Info24Regular } from "@vicons/fluent";

//prettier-ignore
let dateRange = defineModel(
    "dateRange", {
    default: {start: new Date(1577836800000), end: new Date(Date.now() + 28800000)},
});

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

const showRuleDescModal = ref(false);
const showVodDescImg = ref(false);
</script>

<template>
    <div class="mt-8 ml-auto mr-auto w-11/12">
        <n-grid x-gap="12" y-gap="12" cols="4" class="w-11/12" item-responsive>
            <n-gi span="4 800:2">
                <VaCollapse header="選擇日期範圍">
                    <div class="flex">
                        <VaDatePicker v-model="dateRange" mode="range" />
                        <VaDatePicker v-model="dateRange" mode="range" />
                    </div>
                </VaCollapse>

                <!-- not yet -->
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
                    screen-responsive
                />
                <VaButton
                    preset="plain"
                    color="#FFFFFF"
                    class="ml-12 mt-1"
                    @click="showRuleDescModal = !showRuleDescModal"
                >
                    <VaIcon size="large" class="mr-2">
                        <Info24Regular />
                    </VaIcon>
                    <div class="text-center">
                        <div class="text-lg">規則說明</div>
                    </div>
                </VaButton>
            </n-gi>
        </n-grid>

        <!-- 規則說明 -->
        <VaModal
            v-model="showRuleDescModal"
            title="規則說明"
            hide-default-actions
            :closeButton="true"
            z-index="20"
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
                1.
                「計劃」為惡靈的常規直播時數，目前落在2小時左右，因此以2小時計算。
                <br />
                2.
                每次直播的時數沒意外的話以YouTube影片時長為準。若直播紀錄檔被和諧了，則以2小時計算。
                <br />
                3.
                有時在直播懲罰會生成加班台懲罰，此處也會以「懲罰」項目來增加剩餘的加班台時數。
                <br />
                4. 若有其他因素導致時數加減也會以獨立項目處理。
                <br />
                5. 伺服器時間每週三 00:00 會在計算明細表生成一項「計劃」。
                <br />
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

        <VaModal v-model="showVodDescImg" hide-default-actions>
            <img src="@assets/images/vod_time.png" alt="直播時數規則說明" />
        </VaModal>

        <VaDivider class="!mt-2 !mb-2" />

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
                            } else if (selectedTags.includes(tag))
                                selectedTags.splice(
                                    selectedTags.indexOf(tag),
                                    1
                                );
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
    </div>
</template>

<style>
.n-base-suffix__arrow {
    --n-arrow-size: 20px;
}
</style>
