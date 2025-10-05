<script setup lang="ts">
import { ref } from "vue";
import {
    VaButton,
    VaDivider,
    VaModal,
    VaTextarea,
    VaCard,
    VaCardTitle,
    VaCardContent,
} from "vuestic-ui";
import { copyToClipboard } from "@/composables/utils";
import { stateColor, stateString, PenItem } from "@/composables/penalty";

const props = defineProps<{ penalties: PenItem[] }>();

const modal = ref({
    show: false,
    title: "現存",
    color: "#ffffff",
    leftSlot: 0,
    leftCount: 0,
    leftText: "",
    rightSlot: 0,
    rightCount: 0,
    rightText: "",
});

function fillModalData() {
    modal.value.leftCount = props.penalties.filter(
        (x) => x.state === modal.value.leftSlot,
    ).length;
    modal.value.rightCount = props.penalties.filter(
        (x) => x.state === modal.value.rightSlot,
    ).length;

    modal.value.leftText = props.penalties
        .filter((x) => x.state === modal.value.leftSlot)
        .map((x) => x.name)
        .join("\n");

    modal.value.rightText = props.penalties
        .filter((x) => x.state === modal.value.rightSlot)
        .map((x) => x.name)
        .join("\n");
}

function clickExist() {
    modal.value.show = true;
    modal.value.title = "現存";
    modal.value.color = "#b73813";
    modal.value.leftSlot = 1;
    modal.value.rightSlot = 2;
    fillModalData();
}

function clickDone() {
    modal.value.show = true;
    modal.value.title = "完成";
    modal.value.color = "#297a33";
    modal.value.leftSlot = 3;
    modal.value.rightSlot = 4;
    fillModalData();
}
</script>

<template>
    <div class="h-full">
        <VaCard
            style="--va-card-padding: 1rem"
            class="rounded-xl h-full flex flex-col"
        >
            <VaCardTitle class="!text-xl justify-center"> 統計 </VaCardTitle>
            <VaCardContent class="flex justify-stretch gap-4 flex-1">
                <VaButton
                    class="w-full h-full"
                    color="danger"
                    @click="clickExist"
                >
                    <div class="text-xl">現存<br />懲罰</div>
                </VaButton>
                <VaButton
                    class="w-full h-full"
                    color="success"
                    @click="clickDone"
                >
                    <div class="text-xl">完成<br />懲罰</div>
                </VaButton>
            </VaCardContent>
        </VaCard>

        <VaModal
            v-model="modal.show"
            size="small"
            close-button
            hide-default-actions
        >
            <div class="flex flex-row mb-8 mr-4 justify-center items-center">
                <div class="text-lg font-semibold text-zinc-200 flex-grow">
                    懲罰統計：{{ modal.title }}
                </div>
                <VaButton
                    :color="modal.color"
                    gradient
                    @click="
                        copyToClipboard(
                            penalties
                                .filter(
                                    (x) =>
                                        x.state === modal.leftSlot ||
                                        x.state === modal.rightSlot,
                                )
                                .map((x) => x.name)
                                .join('\n'),
                        )
                    "
                >
                    複製所有{{ modal.title }}懲罰
                </VaButton>
            </div>

            <!-- Colors below are specially picked, don't use statusColorSet constant. -->

            <div class="flex justify-center text-center gap-32 ml-4">
                <div class="flex flex-col">
                    <div
                        class="text-sm mt-1"
                        :class="stateColor(modal.leftSlot, 'text')"
                    >
                        {{ stateString(modal.leftSlot) }}
                    </div>
                    <div class="text-3xl mt-1">
                        {{ modal.leftCount }}
                    </div>
                </div>
                <div class="flex flex-col">
                    <div
                        class="text-sm mt-1"
                        :class="stateColor(modal.rightSlot, 'text')"
                    >
                        {{ stateString(modal.rightSlot) }}
                    </div>
                    <div class="text-3xl mt-1">
                        {{ modal.rightCount }}
                    </div>
                </div>
                <div class="flex flex-col">
                    <div class="text-sm mt-1" :style="{ color: modal.color }">
                        {{ modal.title }}總計
                    </div>
                    <div class="text-3xl mt-1">
                        {{ modal.leftCount + modal.rightCount }}
                    </div>
                </div>
            </div>
            <VaDivider class="!mt-2 !mb-1" />
            <div class="flex text-center justify-between">
                <div class="flex flex-col">
                    <div
                        class="text-sm mt-4 mb-2"
                        :class="stateColor(modal.leftSlot, 'text')"
                    >
                        {{ stateString(modal.leftSlot) }}
                    </div>
                    <VaTextarea
                        v-model="modal.leftText"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                </div>
                <div class="flex flex-col">
                    <div
                        class="text-sm mt-4 mb-2"
                        :class="stateColor(modal.rightSlot, 'text')"
                    >
                        {{ stateString(modal.rightSlot) }}
                    </div>
                    <VaTextarea
                        v-model="modal.rightText"
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
    </div>
</template>
