<script setup lang="ts">
import { computed, ref, watch } from "vue";
import {
    VaButton,
    VaChip,
    VaDivider,
    VaIcon,
    VaModal,
    VaProgressBar,
} from "vuestic-ui";
import { openLinks, ofId, BASE_URL } from "@/composables/utils";
import {
    stateString,
    stateColor,
    PenItem,
} from "@/composables/penalty";
import axios from "axios";

const props = defineProps<{
    modelValue: number | null;
}>();

const emit = defineEmits<{
    (e: "update:modelValue", value: number | null): void;
}>();

const penalty = ref<PenItem>({
    id: 0,
    date: "1970-01-01",
    name: "加载中",
    detail: "加载中",
    state: 0,
    history: [[0, "1970-01-01"]],
});

async function loadPenalty(id: number) {
    try {
        const response = await axios.get(`${BASE_URL}/api/penalty/detail/${id}`);
        if (response.status === 200) {
            console.log("Fetched penalty:", response.data);
            penalty.value = response.data;
        } else {
            console.error("Failed to fetch penalty");
        }
    } catch (error) {
        console.error(error);
    }
};
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
        @update:model-value="emit('update:modelValue', $event ? props.modelValue : null)"
        hide-default-actions
        size="small"
        close-button
    >
        <div v-if="penalty" class="text-xl">
            {{ penalty.name }}
            <VaChip
                readonly
                outline
                size="small"
                :color="stateColor(penalty.state, 'raw')"
                class="ml-4"
            >
                ● {{ stateString(penalty.state) }}
            </VaChip>
        </div>

    </VaModal>
</template>
