<!-- components/Detail.vue -->
<script setup lang="ts">
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { BASE_URL } from "@/composables/utils";
import api from "@composables/axios";
import { computed, ref, watch } from "vue";
import { VaChip, VaModal } from "vuestic-ui";
import Timeline from "./Timeline.vue";

const props = defineProps<{
    modelValue: number | null;
}>();

const emit = defineEmits<{
    (e: "update:modelValue", value: number | null): void;
}>();

const penalty = ref<PenItem | null>(null);

const renderedDetail = computed(() => {
    const detail = penalty.value?.detail ?? "";
    if (!detail) return "";
    const base = BASE_URL.replace(/\/$/, "");
    return detail.replace(
        /src=(['"])(\/api\/image\/[^'"]+)\1/g,
        (_match, quote, path) => `src=${quote}${base}${path}${quote}`,
    );
});

async function loadPenalty(id: number) {
    try {
        const response = await api.get(`/api/penalty/detail/${id}`);
        if (response.status === 200) {
            penalty.value = response.data;
        }
    } catch (error) {
        console.error(error);
    }
}

watch(
    () => props.modelValue,
    (newId) => {
        if (newId !== null) {
            loadPenalty(newId);
        } else {
            penalty.value = null;
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
