<!-- components/edit/HistoryTab.vue -->
<script setup lang="ts">
import { computed } from "vue";
import { VaButton, VaDateInput, VaSelect } from "vuestic-ui";
import { formatDate, parseDate } from "@/composables/utils";

export interface HistoryEntry {
  state: number;
  date: Date;
}

const props = defineProps<{
  entries: HistoryEntry[];
  stateOptions: { value: number; text: string }[];
  saving: boolean;
  error: string | null;
  success: string | null;
}>();

const emit = defineEmits<{
  (e: "update:entries", value: HistoryEntry[]): void;
  (e: "save"): void;
  (e: "reset"): void;
}>();

const sortedEntries = computed(() =>
  [...props.entries].sort((a, b) => a.date.getTime() - b.date.getTime())
);

const allowedDays = (date: Date) => {
  const minDate = sortedEntries.value[0]?.date;
  return !minDate || date.getTime() >= minDate.getTime();
};

function addEntry() {
  const newEntry = { state: 0, date: new Date() };
  emit("update:entries", [...props.entries, newEntry]);
}

function removeEntry(item: HistoryEntry) {
  const idx = props.entries.indexOf(item);
  if (idx !== -1) {
    const newEntries = [...props.entries];
    newEntries.splice(idx, 1);
    emit("update:entries", newEntries);
  }
}

function updateEntry(
  item: HistoryEntry,
  field: keyof HistoryEntry,
  value: any
) {
  const idx = props.entries.indexOf(item);
  if (idx !== -1) {
    const newEntries = [...props.entries];
    newEntries[idx] = { ...newEntries[idx], [field]: value };
    emit("update:entries", newEntries);
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
      <VaButton color="secondary" @click="emit('reset')">取消</VaButton>
      <VaButton color="info" :disabled="saving" @click="emit('save')">
        {{ saving ? "儲存中..." : "儲存歷史" }}
      </VaButton>
    </div>
  </div>
</template>