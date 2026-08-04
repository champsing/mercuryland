<script setup lang="ts">
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { copyToClipboard } from "@/composables/utils";
import { ref } from "vue";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaDivider,
    VaIcon,
    VaModal,
    VaTextarea,
} from "vuestic-ui";
import StatisticsChartModal from "./StatisticsChartModal.vue";

interface ModalData {
    title: string;
    color: string;
    leftSlot: number;
    leftCount: number;
    leftText: string;
    rightSlot: number;
    rightCount: number;
    rightText: string;
}

const props = defineProps<{ penalties: PenItem[] }>();

const modal = ref<ModalData | null>(null);
const showChartModal = ref(false);

function fillModalData() {
    if (!modal.value) return;
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
    modal.value = {
        title: "現存",
        color: "#b73813",
        leftSlot: 1,
        leftCount: 0,
        leftText: "",
        rightSlot: 2,
        rightCount: 0,
        rightText: "",
    };
    fillModalData();
}

function clickDone() {
    modal.value = {
        title: "完成",
        color: "#297a33",
        leftSlot: 3,
        leftCount: 0,
        leftText: "",
        rightSlot: 4,
        rightCount: 0,
        rightText: "",
    };
    fillModalData();
}
</script>

<template>
    <div>
        <VaCard
            class="side-card border border-white/[0.08] rounded-lg !bg-[rgba(18,21,27,0.92)]"
        >
            <VaCardContent
                class="side-card__content flex flex-col min-h-[8.25rem] gap-[0.7rem] !p-[0.85rem]"
            >
                <div
                    class="side-card__header flex items-center justify-between gap-3 text-[#45d483]"
                >
                    <div>
                        <span
                            class="side-card__eyebrow text-[#45d483] text-[0.72rem] font-extrabold uppercase"
                            >Statistics</span
                        >
                        <h2
                            class="mt-[0.1rem] text-[#f7f7f8] text-base font-extrabold"
                        >
                            統計
                        </h2>
                    </div>
                    <button
                        class="stat-chart-trigger rounded-lg p-1 -mr-1 hover:bg-white/[0.08] transition-colors"
                        title="查看統計圖表"
                        @click="showChartModal = true"
                    >
                        <VaIcon name="query_stats" size="large" />
                    </button>
                </div>
                <div class="stat-actions grid grid-cols-2 gap-3 flex-1">
                    <VaButton
                        class="stat-action stat-action--danger min-h-4 rounded-lg"
                        color="danger"
                        size="small"
                        @click="clickExist"
                    >
                        <strong class="text-[0.98rem] leading-none"
                            >現存懲罰</strong
                        >
                    </VaButton>
                    <VaButton
                        class="stat-action stat-action--success min-h-4 rounded-lg"
                        color="success"
                        size="small"
                        @click="clickDone"
                    >
                        <strong class="text-[0.98rem] leading-none"
                            >完成懲罰</strong
                        >
                    </VaButton>
                </div>
            </VaCardContent>
        </VaCard>

        <VaModal
            :model-value="modal !== null"
            @update:model-value="
                (value) => {
                    if (!value) modal = null;
                }
            "
            size="small"
            close-button
            hide-default-actions
            :mobile-fullscreen="false"
            class="stat-modal"
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

        <StatisticsChartModal v-model="showChartModal" :penalties="penalties" />
    </div>
</template>

<style scoped>
/* Vuestic 深度覆蓋 */
.side-card {
    --va-card-padding: 0;
}

/* Vuestic 內部 span 樣式 */
.stat-action span {
    font-size: 0.78rem;
    font-weight: 700;
}

.stat-action--danger {
    background: #b73813 !important;
}

.stat-action--success {
    background: #297a33 !important;
}

/* Sizing override — appearance (glassomorphism) comes from
   styles/va-modal-glass.css */
.stat-modal :deep(.va-modal__dialog) {
    max-width: 680px !important;
    width: 95% !important;
}

/* md 以上縮小現存/完成統計彈窗寬度 */
@media (min-width: 768px) {
    .stat-modal :deep(.va-modal__dialog) {
        max-width: 600px !important;
    }
}

.stat-chart-trigger {
    color: inherit;
    cursor: pointer;
    border: none;
    background: none;
    display: flex;
    align-items: center;
    justify-content: center;
}
</style>
