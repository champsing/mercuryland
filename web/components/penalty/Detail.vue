<!-- components/Detail.vue -->
<script setup lang="ts">
import {
    stateColor,
    stateString,
    usePenaltyDetail,
} from "@/composables/penalty";
import { watch } from "vue";
import { VaChip, VaModal } from "vuestic-ui";
import Timeline from "./Timeline.vue";

const props = defineProps<{
    modelValue: number | null;
}>();

const emit = defineEmits<{
    (e: "update:modelValue", value: number | null): void;
}>();

// 🌟 直接解構出我們寫好的共用邏輯
const { penalty, loadPenalty, renderedDetail } = usePenaltyDetail();

watch(
    () => props.modelValue,
    (newId) => {
        if (newId !== null) {
            loadPenalty(newId);
        } else {
            penalty.value = null; // 關閉時清空
        }
    },
    { immediate: true },
);
</script>

<template>
    <VaModal
        :model-value="props.modelValue !== null"
        @update:model-value="
            emit('update:modelValue', $event ? props.modelValue : null)
        "
        hide-default-actions
        size="small"
        close-button
    >
        <div
            v-if="penalty"
            class="max-h-[60vh] flex flex-col text-slate-800 dark:text-slate-100"
        >
            <!-- Header Section: Date & Chip (Column Flex) -->
            <div
                class="flex flex-col gap-1 pb-2 border-b border-black/5 dark:border-white/5"
            >
                <VaChip
                    readonly
                    outline
                    size="small"
                    :color="stateColor(penalty.state, 'raw')"
                    class="w-fit shadow-sm"
                >
                    ● {{ stateString(penalty.state) }}
                </VaChip>
                <div
                    class="text-xl font-medium tracking-wide text-zinc-200 dark:text-zinc-300"
                >
                    {{ penalty.date }}
                </div>
            </div>

            <!-- Hero Section: Penalty Name (Shrinked size) -->
            <div
                class="py-2 my-1 max-h-[20vh] overflow-y-auto custom-scrollbar"
            >
                <div
                    class="text-xl md:text-2xl font-extrabold tracking-tight leading-tight text-transparent bg-clip-text bg-gradient-to-r from-sky-500 via-blue-600 to-indigo-600 dark:from-cyan-300 dark:via-sky-400 dark:to-indigo-400 drop-shadow-sm select-all"
                >
                    {{ penalty.name }}
                </div>
            </div>

            <!-- Progress Section: Timeline wrapped in a subtle card -->
            <div
                class="mb-4 p-4 rounded-2xl bg-white/10 dark:bg-slate-900/40 border border-white/20 dark:border-slate-800/30 shadow-inner"
            >
                <Timeline :history="penalty.history" />
            </div>

            <!-- Detail Scroll Section with custom scrollbar & styled glass border -->
            <div
                class="mt-2 flex-1 flex flex-col overflow-y-auto custom-scrollbar"
            >
                <div
                    class="max-h-60 pr-2 rounded-2xl bg-white/20 dark:bg-slate-950/40 border border-white/30 dark:border-slate-900/50 p-4 shadow-sm"
                >
                    <div
                        v-html="renderedDetail"
                        class="text-sm md:text-base leading-relaxed text-slate-700 dark:text-slate-200"
                    ></div>
                </div>
            </div>
        </div>
    </VaModal>
</template>

<style scoped>
/* 深度穿透修改 Vuestic UI Modal 本體，將其改造為 Apple 風格的玻璃擬態，對標 Login.vue */
:deep(.va-modal__dialog) {
    background: rgba(255, 255, 255, 0.1) !important;
    backdrop-filter: blur(20px) !important;
    -webkit-backdrop-filter: blur(20px) !important;
    border-radius: 24px !important;
    border: 1px solid rgba(255, 255, 255, 0.4) !important;
    box-shadow:
        0 20px 40px -15px rgba(0, 0, 0, 0.08),
        0 0 0 1px rgba(0, 0, 0, 0.02) !important;
    max-width: 520px !important;
    width: 95% !important;
    transition: all 0.3s ease;
}

:global(.dark) :deep(.va-modal__dialog),
:global(.va-theme--dark) :deep(.va-modal__dialog) {
    background: rgba(15, 23, 42, 0.65) !important;
    border: 1px solid rgba(255, 255, 255, 0.08) !important;
    box-shadow:
        0 25px 50px -12px rgba(0, 0, 0, 0.35),
        0 0 0 1px rgba(255, 255, 255, 0.04) !important;
}

/* 修正關閉按鈕的位置 */
:deep(.va-modal__close) {
    top: 1rem !important;
    right: 1rem !important;
    color: currentColor !important;
    opacity: 0.6;
}

/* Custom Scrollbar */
.custom-scrollbar::-webkit-scrollbar {
    width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
    @apply bg-slate-500/20 hover:bg-slate-500/40 dark:bg-slate-400/15 dark:hover:bg-slate-400/35 rounded-full;
}
</style>
