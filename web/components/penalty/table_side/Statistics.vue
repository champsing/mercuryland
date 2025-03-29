<script setup lang="ts">
import { ref } from "vue";
import {
    VaButton,
    VaButtonGroup,
    VaDivider,
    VaModal,
    VaTextarea,
} from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import { copyToClipboard } from "@/composables/utils";


const showExistModal = ref(false);
const showCompleteModal = ref(false);

function queryStatusPenaties(status: string) {
    return penaltyData
        .filter((x) => x.status == status)
        .map((x) => x.name)
        .join("\n")
        .toString();
}

let notYetStartedPenalties = queryStatusPenaties("未開始");

let completedPenalties = queryStatusPenaties("已完成");

let barelyPassedPenalties = queryStatusPenaties("勉強過");

let proceedingPenalties = queryStatusPenaties("進行中");
</script>

<template>
    <div>
                <div class="text-center text-xl bg-stone-600">懲罰數量統計</div>
                <!--This div is for its own size, don't delete.-->
                <VaButtonGroup round class="overall-button">
                    <VaButton
                        color="danger"
                        @click="showExistModal = !showExistModal"
                    >
                        現存
                    </VaButton>
                    <VaButton
                        color="#3D9209"
                        @click="showCompleteModal = !showCompleteModal"
                    >
                        完成
                    </VaButton>
                </VaButtonGroup>
            </div>

            <!-- 現存 和 完成 -->
    <VaModal
        v-model="showExistModal"
        title="懲罰數量統計"
        size="small"
        close-button
        hide-default-actions
    >
        <div class="flex flex-row mb-8">
            <div class="text-xl flex-grow">懲罰數量統計：現存</div>
            <VaButton
                color="warning"
                gradient
                class="-mt-2"
                @click="
                    copyToClipboard(
                        notYetStartedPenalties + '\n' + proceedingPenalties
                    )
                "
            >
                複製所有現存懲罰
            </VaButton>
        </div>

        <!-- Colors below are specially picked, don't use statusColorSet constant. -->

        <div class="flex justify-center text-center gap-32 ml-4">
            <div class="flex flex-col">
                <div class="text-sm mt-1 text-[#ef3b3b]">未開始</div>
                <div class="text-3xl mt-1">
                    {{ penaltyData.filter((x) => x.status == "未開始").length }}
                </div>
            </div>
            <div class="flex flex-col">
                <div class="text-sm mt-1 text-[#de8039]">進行中</div>
                <div class="text-3xl mt-1">
                    {{ penaltyData.filter((x) => x.status == "進行中").length }}
                </div>
            </div>
            <div class="flex flex-col">
                <div class="text-sm mt-1 text-[#eda9a9]">現存總計</div>
                <div class="text-3xl mt-1">
                    {{
                        penaltyData.filter(
                            (x) => x.status == "未開始" || x.status == "進行中"
                        ).length
                    }}
                </div>
            </div>
        </div>
        <VaDivider class="!mt-2 !mb-1" />
        <div class="flex text-center justify-between">
            <div class="flex flex-col">
                <div class="text-sm mt-4 mb-2">未開始</div>
                <VaTextarea
                    v-model="notYetStartedPenalties"
                    :maxRows="7"
                    :resize="false"
                    readonly
                />
            </div>
            <div class="flex flex-col">
                <div class="text-sm mt-4 mb-2">進行中</div>
                <VaTextarea
                    v-model="proceedingPenalties"
                    :maxRows="7"
                    :resize="false"
                    readonly
                />
            </div>
        </div>
        <div class="flex justify-start mt-4 text-sm">
            <kbd>Ctrl</kbd>&nbsp;<kbd>A</kbd>&ensp;可快速選取全部項目
        </div>
    </VaModal>

    <VaModal
        v-model="showCompleteModal"
        title="懲罰數量統計"
        size="small"
        hide-default-actions
        close-button
    >
        <div class="flex flex-row mb-8">
            <div class="text-xl flex-grow">懲罰數量統計：完成</div>
            <VaButton
                color="success"
                gradient
                class="-mt-2"
                @click="
                    copyToClipboard(
                        completedPenalties + '\n' + barelyPassedPenalties
                    )
                "
            >
                複製所有完成懲罰
            </VaButton>
        </div>
        <div class="flex justify-center text-center gap-32 ml-4">
            <div class="flex flex-col">
                <div class="text-sm mt-1 text-[#4be66c]">已完成</div>
                <div class="text-3xl mt-1">
                    {{ penaltyData.filter((x) => x.status == "已完成").length }}
                </div>
            </div>
            <div class="flex flex-col">
                <div class="text-sm mt-1 text-[#218d37]">勉強過</div>
                <div class="text-3xl mt-1">
                    {{ penaltyData.filter((x) => x.status == "勉強過").length }}
                </div>
            </div>
            <div class="flex flex-col">
                <div class="text-sm mt-1 text-[#39e3e3]">完成總計</div>
                <div class="text-3xl mt-1">
                    {{
                        penaltyData.filter(
                            (x) => x.status == "已完成" || x.status == "勉強過"
                        ).length
                    }}
                </div>
            </div>
        </div>
        <VaDivider class="!mt-2 !mb-1" />
        <div class="flex justify-between text-center">
            <div class="flex flex-col">
                <div class="text-sm mt-4 mb-2">已完成</div>
                <VaTextarea
                    v-model="completedPenalties"
                    :maxRows="7"
                    :resize="false"
                    readonly
                />
            </div>
            <div class="flex flex-col">
                <div class="text-sm mt-4 mb-2">勉強過</div>
                <VaTextarea
                    v-model="barelyPassedPenalties"
                    :maxRows="7"
                    :resize="false"
                    readonly
                />
            </div>
        </div>
        <div class="flex justify-start mt-4 text-sm">
            <kbd>Ctrl</kbd>&nbsp;<kbd>A</kbd>&ensp;可快速選取全部項目
        </div>
    </VaModal>
</template>