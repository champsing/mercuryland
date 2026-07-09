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
        <div v-if="penalty" class="min-h-[50vh]">
            <div class="flex gap-4 items-center">
                <VaChip
                    readonly
                    outline
                    size="small"
                    :color="stateColor(penalty.state, 'raw')"
                    class="w-24"
                >
                    ● {{ stateString(penalty.state) }}
                </VaChip>
                <div class="text-xl flex-1">{{ penalty.date }}</div>
            </div>

            <div class="text-lg mt-2">{{ penalty.name }}</div>

            <Timeline :history="penalty.history" />

            <div class="mt-4">
                <div class="max-h-60 overflow-y-scroll">
                    <div v-html="renderedDetail"></div>
                </div>
            </div>
        </div>
    </VaModal>
</template>
