<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { VaButton, VaSelect, VaDateInput } from "vuestic-ui";
import { stateString } from "@/composables/penalty";
import { formatDate, parseDate } from "@/composables/utils";
import api from "@composables/axios";

const props = defineProps<{
    history: Array<[number, string]>;
    penaltyId: number;
}>();

const emit = defineEmits<{
    (e: "update:history", value: Array<[number, string]>): void;
    (e: "cancel"): void;
}>();

const editedHistory = ref<Array<{ state: number; date: Date }>>(
    props.history.map(([state, dateStr]) => ({
        state,
        date: parseDate(dateStr),
    })),
);

const isSaving = ref(false);
const saveError = ref<string | null>(null);
const saveSuccess = ref<string | null>(null);

const stateOptions = [
    { value: 0, text: stateString(0) },
    { value: 1, text: stateString(1) },
    { value: 2, text: stateString(2) },
    { value: 3, text: stateString(3) },
    { value: 4, text: stateString(4) },
];

const textBy = (option: { text: string }) => option.text;
const valueBy = (option: { value: number }) => option.value;

const allowedDays = (date: Date) => {
    const minDate = sortedHistory.value[0]?.date;
    return !minDate || date.getTime() >= minDate.getTime();
};

const sortedHistory = computed(() => {
    return [...editedHistory.value].sort((a, b) => {
        return a.date.getTime() - b.date.getTime();
    });
});

function addNewEntry() {
    const today = new Date();
    editedHistory.value.push({ state: 0, date: today });
}

function removeEntry(item: { state: number; date: Date }) {
    const idx = editedHistory.value.indexOf(item);
    if (idx !== -1) {
        editedHistory.value.splice(idx, 1);
    }
}

async function saveHistory() {
    const token = localStorage.getItem("token");
    if (!token) {
        saveError.value = "請先登入管理員帳號";
        return;
    }

    try {
        isSaving.value = true;
        saveError.value = null;
        saveSuccess.value = null;
        const response = await api.post(`/api/penalty/history/update`, {
            token,
            id: props.penaltyId,
            history: sortedHistory.value.map(({ state, date }) => [
                state,
                formatDate(date),
            ]),
        });
        if (response.data.success) {
            saveSuccess.value = "更新成功";
            emit(
                "update:history",
                sortedHistory.value.map(({ state, date }) => [
                    state,
                    formatDate(date),
                ]),
            );
            setTimeout(() => {
                emit("cancel");
            }, 600);
        } else {
            saveError.value = "更新失敗";
        }
    } catch (error) {
        console.error(error);
        saveError.value = "更新失敗，請稍後再試";
    } finally {
        isSaving.value = false;
    }
}

function cancelEdit() {
    emit("cancel");
}

watch(
    () => props.history,
    (newHistory) => {
        editedHistory.value = newHistory.map(([state, dateStr]) => ({
            state,
            date: parseDate(dateStr),
        }));
    },
    { immediate: true },
);
</script>

<template>
    <div class="mt-4">
        <div>
            <div
                v-for="(item, index) in sortedHistory"
                :key="index"
                class="flex items-center gap-2 pb-2"
            >
                <VaDateInput
                    v-model="item.date"
                    :format-date="formatDate"
                    :parse-date="parseDate"
                    :disabled="index === 0"
                    :allowed-days="allowedDays"
                    manual-input
                    mode="auto"
                    class="w-40 flex-1"
                />
                <VaSelect
                    v-model="item.state"
                    :options="stateOptions"
                    :text-by="textBy"
                    :value-by="valueBy"
                    class="w-32 flex-1"
                />
                <VaButton
                    v-if="editedHistory.length > 1"
                    @click="removeEntry(item)"
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
            <VaButton @click="saveHistory" color="primary" :disabled="isSaving">
                {{ isSaving ? "儲存中..." : "保存" }}
            </VaButton>
        </div>
        <p v-if="saveError" class="text-sm text-red-400 mt-2">
            {{ saveError }}
        </p>
        <p v-if="saveSuccess" class="text-sm text-emerald-400 mt-2">
            {{ saveSuccess }}
        </p>
    </div>
</template>
