<script setup lang="ts">
import { ref, watch } from "vue";
import { VaChip, VaModal } from "vuestic-ui";
import { BASE_URL } from "@/composables/utils";
import { stateString, stateColor, PenItem } from "@/composables/penalty";
import axios from "axios";

const props = defineProps<{
    modelValue: number | null;
}>();

const emit = defineEmits<{
    (e: "update:modelValue", value: number | null): void;
}>();

const penalty = ref<PenItem | null>(null);

async function loadPenalty(id: number) {
    try {
        const response = await axios.get(
            `${BASE_URL}/api/penalty/detail/${id}`,
        );
        if (response.status === 200) {
            console.log("Fetched penalty:", response.data);
            penalty.value = response.data;
        } else {
            console.error("Failed to fetch penalty");
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
        <template v-if="penalty">
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
                <div class="truncate text-xl flex-1">{{ penalty.name }}</div>
            </div>
            <div
                v-if="penalty.history && penalty.history.length > 0"
                class="mt-4"
            >
                <div class="relative">
                    <div
                        class="absolute top-4 left-0 right-0 h-0.5 bg-gray-300"
                    ></div>
                    <div class="flex justify-between">
                        <div
                            v-for="(item, index) in penalty.history"
                            :key="index"
                            class="flex flex-col items-center"
                            :style="{
                                flex:
                                    index === 0 ||
                                    index === penalty.history.length - 1
                                        ? '0 0 auto'
                                        : '1',
                            }"
                        >
                            <div
                                class="w-3 h-3 rounded-full shadow-md"
                                :style="{
                                    backgroundColor: stateColor(item[0], 'raw'),
                                }"
                            ></div>
                            <div class="text-xs text-center mt-1">
                                <div
                                    class="font-medium mt-1"
                                    :style="{
                                        color: stateColor(item[0], 'raw'),
                                    }"
                                >
                                    {{ stateString(item[0]) }}
                                </div>
                                <div class="text-gray-500">{{ item[1] }}</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div v-html="penalty.detail" class="mt-4"></div>
        </template>
    </VaModal>
</template>
