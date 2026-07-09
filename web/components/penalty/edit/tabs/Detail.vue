<!-- components/edit/Detail.vue -->
<script setup lang="ts">
import { BASE_URL } from "@/composables/utils";
import { computed, ref } from "vue";
import { VaButton, VaTextarea } from "vuestic-ui";
import AddImage from "../add/Image.vue";
import AddPenaltyBtn from "../add/PenaltyBtn.vue";
import AddSteam from "../add/Steam.vue";
import AddSyntax from "../add/Syntax.vue";
import AddYoutube from "../add/Youtube.vue";

const props = defineProps<{
    detail: string;
    saving: boolean;
    error: string | null;
    success: string | null;
}>();

const emit = defineEmits<{
    (e: "update:detail", value: string): void;
    (e: "save"): void;
    (e: "back"): void;
}>();

const textareaRef = ref<HTMLTextAreaElement>();

function insertHtml(html: string, range?: { start: number; end: number }) {
    const content = props.detail ?? "";
    const start = Math.min(
        Math.max(range?.start ?? content.length, 0),
        content.length,
    );
    const end = Math.min(
        Math.max(range?.end ?? range?.start ?? content.length, start),
        content.length,
    );
    const newContent = content.slice(0, start) + html + content.slice(end);
    emit("update:detail", newContent);
}

const renderedDetail = computed(() => {
    const detail = props.detail ?? "";
    if (!detail) return "";
    const base = BASE_URL.replace(/\/$/, "");
    return detail.replace(
        /src=(['"])(\/api\/image\/[^'"]+)\1/g,
        (_match, quote, path) => `src=${quote}${base}${path}${quote}`,
    );
});
</script>

<template>
    <div class="flex flex-col gap-4">
        <div class="flex gap-2">
            <VaTextarea
                ref="textareaRef"
                :model-value="detail"
                @update:model-value="emit('update:detail', $event)"
                placeholder="輸入 HTML 詳情"
                class="w-3/4"
                :resize="false"
                min-rows="9"
                max-rows="9"
            />
            <div class="flex flex-col gap-2 w-1/4">
                <AddSteam
                    :textarea-ref="textareaRef"
                    @insert-html="insertHtml"
                />
                <AddYoutube
                    :textarea-ref="textareaRef"
                    @insert-html="insertHtml"
                />
                <AddImage
                    :textarea-ref="textareaRef"
                    @insert-html="insertHtml"
                />
                <AddSyntax
                    :textarea-ref="textareaRef"
                    @insert-html="insertHtml"
                />
                <AddPenaltyBtn
                    :textarea-ref="textareaRef"
                    @insert-html="insertHtml"
                />
            </div>
        </div>
        <!-- 预览 -->
        <div class="border border-gray-600 p-2 max-h-40 overflow-y-auto">
            <div v-html="renderedDetail"></div>
        </div>
        <p v-if="error" class="text-sm text-red-400">{{ error }}</p>
        <p v-if="success" class="text-sm text-emerald-400">{{ success }}</p>
        <div class="flex justify-between gap-2">
            <VaButton color="secondary" @click="emit('back')">返回</VaButton>
            <VaButton color="info" :disabled="saving" @click="emit('save')">
                {{ saving ? "儲存中..." : "儲存詳情" }}
            </VaButton>
        </div>
    </div>
</template>
