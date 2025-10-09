<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { VaButton, VaSelect, VaDateInput } from "vuestic-ui";
import { stateString } from "@/composables/penalty";
import { BASE_URL, formatDate, parseDate } from "@/composables/utils";
import axios from "axios";

const props = defineProps<{
    history: Array<[number, string]>;
    penaltyId: number;
}>();

const emit = defineEmits<{
    (e: "update:history", value: Array<[number, string]>): void;
    (e: "cancel"): void;
}>();

const editedHistory = ref<Array<[number, Date]>>(
    props.history.map(([state, dateStr]) => [state, parseDate(dateStr)]),
);

const stateOptions = [
    { value: 0, text: stateString(0) },
    { value: 1, text: stateString(1) },
    { value: 2, text: stateString(2) },
    { value: 3, text: stateString(3) },
    { value: 4, text: stateString(4) },
];

const sortedHistory = computed(() => {
    return [...editedHistory.value].sort((a, b) => {
        const dateA = new Date(a[1]);
        const dateB = new Date(b[1]);
        return dateA.getTime() - dateB.getTime();
    });
});

function addNewEntry() {
    const today = new Date();
    editedHistory.value.push([0, today]);
}

function removeEntry(index: number) {
    editedHistory.value.splice(index, 1);
}

function updateState(index: number, newState: number) {
    editedHistory.value[index][0] = newState;
}

async function saveHistory() {
    const token = localStorage.getItem("token");
    if (!token) return;

    try {
        const response = await axios.post(
            `${BASE_URL}/api/penalty/history/update`,
            {
                token,
                id: props.penaltyId,
                history: sortedHistory.value.map(([state, date]) => [
                    state,
                    formatDate(date),
                ]),
            },
        );
        if (response.data.success) {
            emit(
                "update:history",
                sortedHistory.value.map(([state, date]) => [
                    state,
                    formatDate(date),
                ]),
            );
        }
    } catch (error) {
        console.error(error);
    }
}

function cancelEdit() {
    emit("cancel");
}

watch(
    () => props.history,
    (newHistory) => {
        editedHistory.value = newHistory.map(([state, dateStr]) => [
            state,
            parseDate(dateStr),
        ]);
    },
    { immediate: true },
);
</script>

<template>
    <div class="mt-4">
        <div>
            <div
                v-for="(item, index) in editedHistory"
                :key="index"
                class="flex items-center gap-2 pb-2"
            >
                <VaDateInput
                    v-model="item[1]"
                    :format-date="formatDate"
                    :parse-date="parseDate"
                    manual-input
                    mode="auto"
                    class="w-40 flex-1"
                />
                <VaSelect
                    v-model="item[0]"
                    :options="stateOptions"
                    class="w-32 flex-1"
                    @update:model-value="(value) => updateState(index, value)"
                />
                <VaButton
                    v-if="editedHistory.length > 1"
                    @click="removeEntry(index)"
                    color="danger"
                    size="medium"
                    icon="remove"
                />
            </div>
            <VaButton @click="addNewEntry" color="secondary" class="w-full">
                添加
            </VaButton>
        </div>
        <div class="mt-4 flex justify-between">
            <VaButton @click="cancelEdit" color="secondary"> 取消 </VaButton>
            <VaButton @click="saveHistory" color="primary"> 保存 </VaButton>
        </div>
    </div>
</template>
