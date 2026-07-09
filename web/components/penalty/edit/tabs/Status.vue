<!-- components/edit/BasicTab.vue -->
<script setup lang="ts">
import { VaButton, VaDateInput, VaSelect, VaTextarea } from "vuestic-ui";
import { formatDate, parseDate } from "@/composables/utils";

defineProps<{
  date: Date;
  name: string;
  state: number;
  saving: boolean;
  error: string | null;
  success: string | null;
  stateOptions: { value: number; text: string }[];
}>();

const emit = defineEmits<{
  (e: "update:date", value: Date): void;
  (e: "update:name", value: string): void;
  (e: "update:state", value: number): void;
  (e: "save"): void;
  (e: "delete"): void;
  (e: "cancel"): void;
}>();

const textBy = (opt: { text: string }) => opt.text;
const valueBy = (opt: { value: number }) => opt.value;
</script>

<template>
  <div class="flex flex-col gap-4">
    <VaDateInput
      :model-value="date"
      @update:model-value="emit('update:date', $event)"
      label="日期"
      color="info"
      :format-date="formatDate"
      :parse-date="parseDate"
      manual-input
      mode="auto"
      required
    />
    <VaTextarea
      :model-value="name"
      @update:model-value="emit('update:name', $event)"
      label="內容"
      color="info"
      :resize="false"
      min-rows="3"
      max-rows="3"
      required
    />
    <VaSelect
      :model-value="state"
      @update:model-value="emit('update:state', $event)"
      label="狀態"
      color="info"
      :options="stateOptions"
      :text-by="textBy"
      :value-by="valueBy"
      required
    />
    <p v-if="error" class="text-sm text-red-400">{{ error }}</p>
    <p v-if="success" class="text-sm text-emerald-400">{{ success }}</p>
    <div class="flex justify-between gap-2">
      <VaButton color="danger" :disabled="saving" @click="emit('delete')">
        刪除
      </VaButton>
      <div class="flex gap-2">
        <VaButton color="secondary" @click="emit('cancel')">取消</VaButton>
        <VaButton color="info" :disabled="saving" @click="emit('save')">
          {{ saving ? "儲存中..." : "更新" }}
        </VaButton>
      </div>
    </div>
  </div>
</template>