<script setup lang="ts">
import { formatDate, parseDate } from "@/composables/utils";
import { computed } from "vue";
import { VaButton, VaDateInput, VaSelect } from "vuestic-ui";

export interface HistoryEntry {
    state: number;
    date: Date;
}

const props = defineProps<{
    stateOptions: { value: number; text: string }[];
    saving: boolean;
    error: string | null;
    success: string | null;
}>();

const emit = defineEmits<{
    (e: "save"): void;
    (e: "back"): void;
}>();

// 🌟 使用 defineModel 自動接管 v-model:entries
const entries = defineModel<HistoryEntry[]>("entries", { default: () => [] });

const sortedEntries = computed(() =>
    [...entries.value].sort((a, b) => a.date.getTime() - b.date.getTime()),
);

const allowedDays = (date: Date) => {
    const minDate = sortedEntries.value[0]?.date;
    return !minDate || date.getTime() >= minDate.getTime();
};

function addEntry() {
    entries.value = [...entries.value, { state: 0, date: new Date() }];
}

function removeEntry(item: HistoryEntry) {
    entries.value = entries.value.filter((e) => e !== item);
}

function updateEntry(
    item: HistoryEntry,
    field: keyof HistoryEntry,
    value: any,
) {
    const idx = entries.value.indexOf(item);
    if (idx !== -1) {
        const newEntries = [...entries.value];
        newEntries[idx] = { ...newEntries[idx], [field]: value };
        entries.value = newEntries;
    }
}

const textBy = (opt: { text: string }) => opt.text;
const valueBy = (opt: { value: number }) => opt.value;
</script>

<template>
    <div class="flex flex-col gap-4">
        <div
            v-for="(item, index) in sortedEntries"
            :key="index"
            class="flex items-center gap-2 pb-2"
        >
            <VaDateInput
                :model-value="item.date"
                @update:model-value="updateEntry(item, 'date', $event)"
                :format-date="formatDate"
                :parse-date="parseDate"
                :disabled="index === 0"
                :allowed-days="allowedDays"
                manual-input
                mode="auto"
                class="w-40 flex-1"
            />
            <VaSelect
                :model-value="item.state"
                @update:model-value="updateEntry(item, 'state', $event)"
                :options="stateOptions"
                :text-by="textBy"
                :value-by="valueBy"
                class="w-32 flex-1"
            />
            <VaButton
                v-if="entries.length > 1"
                @click="removeEntry(item)"
                color="danger"
                size="medium"
                icon="remove"
            />
        </div>
        <VaButton @click="addEntry" color="secondary" class="w-full"
            >添加</VaButton
        >
        <p v-if="error" class="text-sm text-red-400">{{ error }}</p>
        <p v-if="success" class="text-sm text-emerald-400">{{ success }}</p>
        <div class="flex justify-between gap-2">
            <VaButton color="secondary" @click="emit('back')">返回</VaButton>
            <VaButton color="info" :disabled="saving" @click="emit('save')">
                {{ saving ? "儲存中..." : "儲存歷史" }}
            </VaButton>
        </div>
    </div>
</template>
