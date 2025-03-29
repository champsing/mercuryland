<script setup lang="ts">
import {
    VaButton,
    VaDateInput,
    VaDivider,
    VaIcon,
    VaInput,
    VaModal,
    VaSelect,
} from "vuestic-ui";
import penaltyStatus from "@assets/data/penalty_status.json";
import TableSide from "./table_side/TableSide.vue";
import Table from "./Table.vue";
import { formatDate, parseDate } from "@/composables/utils";
import { Info24Regular } from "@vicons/fluent";
import { ref } from "vue";

const showRuleDescModal = ref(false);

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
</script>

<template>
    <div class="mt-4 m-auto w-11/12">
        <div class="flex-row justify-evenly hidden xl:flex">
            <div class="w-3/8">
                <VaDateInput
                    v-model="filterDate"
                    :format-date="formatDate"
                    :parse-date="parseDate"
                    manual-input
                    mode="auto"
                />
            </div>
            <div class="w-3/8">
                <VaSelect
                    v-model="filterStatus"
                    :options="finishOptions"
                    placeholder="請選擇一種完成狀態"
                    clearable
                    :clear-value="null"
                />
            </div>
            <div class="w-1/4">
                <VaInput
                    class="w-full"
                    placeholder="輸入懲罰內容來搜尋"
                    v-model="filterSearch"
                />
            </div>
            <div class="w-1/8">
                <VaButton
                    class="mt-1"
                    preset="plain"
                    color="#FFFFFF"
                    @click="showRuleDescModal = !showRuleDescModal"
                >
                    <VaIcon size="large" class="mr-2">
                        <Info24Regular />
                    </VaIcon>
                    <div class="text-lg text-center">規則說明</div>
                </VaButton>
            </div>
        </div>

        <VaDivider class="!m-2" />

        <div class="flex flex-row gap-4">
            <div class="h-80vh w-5/6">
                <Table
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
            <TableSide class="w-1/6"/>
        </div>
    </div>

    <!-- 規則說明 -->
    <VaModal
        v-model="showRuleDescModal"
        title="規則說明"
        hide-default-actions
        close-button
    >
        <span class="text-3xl"> 直播懲罰規則說明 </span>
        <div class="text-2xl mt-2">●概述</div>
        <div class="text-base mt-2">
            惡靈會在直播的時候跟觀眾打賭該局遊戲加減懲罰的賭注，然後在直播最後以隨機輪盤抽出當天的懲罰數量。
            <br />
            每個懲罰會各自擁有一個完成狀態，分別有：未開始、已完成、勉強過、進行中。
            <br />
            關於各完成狀態的說明，請點擊下方圖例中的「完成狀態」查看。
        </div>
        <div class="text-2xl mt-4">●加班台懲罰</div>
        <div class="text-base mt-2">
            如果懲罰主文要求加班台時數，則只有在該懲罰生成「之後」加的班才會被計算進該懲罰的完成進度裡。
            <br />
            例如：
            <br />
            01/01被懲罰加班台2小時，01/02惡靈有加班時數47分鐘，則這47分鐘可以被計算進01/01的「加班台2小時」懲罰裡。
            <br />
            反之，若在01/03也有懲罰加班台2小時懲罰，01/02的47分鐘就不會被算進01/03懲罰完成進度裡。
        </div>
    </VaModal>
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
