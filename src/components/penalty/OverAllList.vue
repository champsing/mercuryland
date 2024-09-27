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
    VaButton,
    VaButtonGroup,
    VaChip,
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
</script>

<template>
    <n-flex size="small" vertical class="m-auto ml-4" item-responsive>
        <n-flex justify="center" size="small" class="m-auto" item-responsive>
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
                disabled
            >
                <n-icon size="25" class="mr-2">
                    <Info24Regular />
                </n-icon>
                <div class="text-center">
                    <div class="text-lg">規則說明</div>
                </div>
            </VaButton>
        </n-flex>

        <VaChip class="vachip2 mt-4" color="#3d807c" readonly>
            <n-icon size="25" class="mt-1 mr-2">
                <InfoCircle />
            </n-icon>
            <div class="text-center text-amber-200">
                <div class="text-lg mt-1">圖表維護中，敬請期待開放</div>
            </div>
        </VaChip>
    </n-flex>

    <!-- 規則說明 -->
    <VaModal v-model="showRuleDescModal" >
        <h3 class="text-pretty">
            直播懲罰規則說明
        </h3>
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
