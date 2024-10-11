<script setup lang="ts">
import { ref } from "vue";
import {
    NButton,
    NCard,
    NDivider,
    NFlex,
    NGrid,
    NGi,
    NIcon,
    NModal,
} from "naive-ui";
import {
    VaAlert,
    VaButton,
    VaButtonGroup,
    VaModal,
    VaTextarea,
} from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import { copyToClipboard } from "@/composables/utils";
import { InfoCircle } from "@vicons/tabler";
import { Info24Regular } from "@vicons/fluent";

const showExistModal = ref(false);
const showCompleteModal = ref(false);
const showRuleDescModal = ref(false);

let notYetStartedPenalties = penaltyData
    .filter((x) => x.status == "未開始")
    .map((x) => x.name)
    .join("\n")
    .toString();

let completedPenalties = penaltyData
    .filter((x) => x.status == "已完成")
    .map((x) => x.name)
    .join("\n")
    .toString();

let barelyPassedPenalties = penaltyData
    .filter((x) => x.status == "勉強過")
    .map((x) => x.name)
    .join("\n")
    .toString();

let proceedingPenalties = penaltyData
    .filter((x) => x.status == "進行中")
    .map((x) => x.name)
    .join("\n")
    .toString();

const notEffectivePenalties = [
    "麥塊蓋故宮",
    "唸一遍所有馬來西亞官方郵局的地址",
    "直播玩雀魂觀眾場",
    "玩歐卡*買五星貨車",
]
    .join("\n")
    .toString();
</script>

<template>
    <n-flex size="small" vertical class="m-auto" item-responsive>
        <VaAlert class="mt-2" color="#3d807c" closeable>
            <div class="flex flex-row">
                <n-icon size="25" class="mr-2">
                    <InfoCircle />
                </n-icon>
                <div class="text-center text-amber-200">
                    <div class="text-lg">圖表維護中，敬請期待開放</div>
                </div>
            </div>
        </VaAlert>

        <VaAlert class="mt-2" color="#59753f" closeable>
            <div class="flex flex-row">
                <n-icon size="25" class="mt-1 mr-2">
                    <InfoCircle />
                </n-icon>
                <div class="text-center text-yellow-300">
                    <div class="text-lg mt-1">點擊完成狀態可快速切換</div>
                </div>
            </div>
        </VaAlert>

        <n-flex size="small" class="m-auto" item-responsive>
            <!-- <VaChip class="vachip2" color="#3d807c" readonly>
            <n-icon size="25" class="mt-1 mr-2">
                <InfoCircle />
            </n-icon>
            <div class="text-center text-amber-200">
                <div class="text-lg mt-1">將滑鼠移至圖表上可查看數量</div>
            </div>
        </VaChip> -->
            <div>
                <!--This div is for its own size, don't delete.-->
                <VaButtonGroup round class="overall-button">
                    <VaButton
                        color="danger"
                        @click="showExistModal = !showExistModal"
                    >
                        現存
                    </VaButton>
                    <VaButton
                        color="success"
                        @click="showCompleteModal = !showCompleteModal"
                    >
                        完成
                    </VaButton>
                </VaButtonGroup>
            </div>
            <VaButton
                preset="plain"
                color="#FFFFFF"
                class="ml-4 mt-2"
                @click="showRuleDescModal = !showRuleDescModal"
            >
                <n-icon size="25" class="mr-2">
                    <Info24Regular />
                </n-icon>
                <div class="text-center">
                    <div class="text-lg">規則說明</div>
                </div>
            </VaButton>
        </n-flex>

        <div class="flex flex-col text-center">
            <div class="text-sm mt-4 text-zinc-200">尚未生效的懲罰</div>
            <div class="text-sm mt-1 text-zinc-200">
                <VaTextarea
                    v-model="notEffectivePenalties"
                    :maxRows="7"
                    :resize="false"
                    readonly
                />
            </div>
        </div>
    </n-flex>

    <!-- 規則說明 -->
    <VaModal
        v-model="showRuleDescModal"
        title="規則說明"
        hide-default-actions
        :closeButton="true"
    >
        <span class="text-3xl"> 直播懲罰規則說明 </span>
        <div class="text-2xl mt-2">●概述</div>
        <div class="text-bg mt-2">
            惡靈會在直播的時候跟觀眾打賭該局遊戲加減懲罰的賭注，然後在直播最後以隨機輪盤抽出當天的懲罰數量。
            <br />
            每個懲罰會各自擁有一個完成狀態，分別有：未開始、已完成、勉強過、進行中。
        </div>
        <div class="text-bg mt-4">
            <div class="same-line !text-[#b91c1c]">▲未開始</div>
            ：尚未開始嘗試完成該懲罰，沒有進度
            <br />
            <div class="same-line !text-[#4d7c0f]">▲已完成</div>
            ：已經完成該懲罰主文要求的全部條件
            <br />
            <div class="same-line !text-[#047857]">▲勉強過</div>
            ：該懲罰的原主人或是投票決定讓惡靈在沒有完成主文要求的全部條件下完成該懲罰
            <br />
            <div class="same-line !text-[#b45309]">▲進行中</div>
            ：正在嘗試完成，已經有進度的懲罰
        </div>
        <div class="text-2xl mt-4">●加班台懲罰</div>
        <div class="text-bg mt-2">
            如果懲罰主文要求加班台時數，則只有在該懲罰生成「之後」加的班才會被計算進該懲罰的完成進度裡。
            <br />
            例如：
            <br />
            01/01被懲罰加班台2小時，01/02惡靈有加班時數47分鐘，則這47分鐘可以被計算進01/01的「加班台2小時」懲罰裡。
            <br />
            反之，若在01/03也有懲罰加班台2小時懲罰，01/02的47分鐘就不會被算進01/03懲罰完成進度裡。
        </div>
    </VaModal>

    <!-- 現存 和 完成 -->
    <n-modal v-model:show="showExistModal">
        <n-card
            style="width: 600px"
            title="懲罰一覽表：現存"
            :bordered="false"
            size="huge"
            role="dialog"
            aria-modal="true"
        >
            <template #header-extra>
                <n-button
                    @click="
                        copyToClipboard(
                            notYetStartedPenalties + '\n' + proceedingPenalties
                        )
                    "
                >
                    複製所有現存懲罰
                </n-button>
            </template>
            <n-grid
                :x-gap="3"
                :y-gap="2"
                :cols="3"
                class="text-center"
                screen-responsive
            >
                <n-gi>
                    <div class="text-sm mt-1 text-[#ef3b3b]">未開始</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter((x) => x.status == "未開始")
                                .length
                        }}
                    </div>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-1 text-[#de8039]">進行中</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter((x) => x.status == "進行中")
                                .length
                        }}
                    </div>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-1 text-[#eda9a9]">現存總計</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter(
                                (x) =>
                                    x.status == "未開始" || x.status == "進行中"
                            ).length
                        }}
                    </div>
                </n-gi>
            </n-grid>
            <n-divider class="!mt-2 !mb-1" />
            <n-grid
                :x-gap="4"
                :y-gap="4"
                :cols="2"
                class="text-center mt-2"
                screen-responsive
            >
                <n-gi>
                    <div class="text-sm mt-4 mb-2">未開始</div>
                    <VaTextarea
                        v-model="notYetStartedPenalties"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                    <n-flex style="justify-content: start" class="mt-4">
                        <div class="text-sm">
                            <kbd>Ctrl</kbd>
                            &nbsp;<kbd>A</kbd>&ensp;可快速複製全部項目
                        </div>
                    </n-flex>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-4 mb-2">進行中</div>
                    <VaTextarea
                        v-model="proceedingPenalties"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                </n-gi>
            </n-grid>
        </n-card>
    </n-modal>

    <n-modal v-model:show="showCompleteModal">
        <n-card
            style="width: 600px"
            title="懲罰一覽表：完成"
            :bordered="false"
            size="huge"
            role="dialog"
            aria-modal="true"
        >
            <template #header-extra>
                <n-button
                    @click="
                        copyToClipboard(
                            completedPenalties + '\n' + barelyPassedPenalties
                        )
                    "
                >
                    複製所有完成懲罰
                </n-button>
            </template>
            <n-grid
                :x-gap="3"
                :y-gap="2"
                :cols="3"
                class="text-center"
                screen-responsive
            >
                <n-gi>
                    <div class="text-sm mt-1 text-[#4be66c]">已完成</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter((x) => x.status == "已完成")
                                .length
                        }}
                    </div>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-1 text-[#218d37]">勉強過</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter((x) => x.status == "勉強過")
                                .length
                        }}
                    </div>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-1 text-[#39e3e3]">完成總計</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter(
                                (x) =>
                                    x.status == "已完成" || x.status == "勉強過"
                            ).length
                        }}
                    </div>
                </n-gi>
            </n-grid>
            <n-divider class="!mt-2 !mb-1" />
            <n-grid
                :x-gap="4"
                :y-gap="4"
                :cols="2"
                class="text-center"
                screen-responsive
            >
                <n-gi>
                    <div class="text-sm mt-4 mb-2">已完成</div>
                    <VaTextarea
                        v-model="completedPenalties"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                    <n-flex style="justify-content: start" class="mt-4">
                        <div class="text-sm">
                            <kbd>Ctrl</kbd>
                            &nbsp;<kbd>A</kbd>&ensp;可快速複製全部項目
                        </div>
                    </n-flex>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-4 mb-2">勉強過</div>
                    <VaTextarea
                        v-model="barelyPassedPenalties"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                </n-gi>
            </n-grid>
        </n-card>
    </n-modal>
</template>
