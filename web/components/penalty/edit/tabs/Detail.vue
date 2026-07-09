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
    saving: boolean;
    error: string | null;
    success: string | null;
}>();

const emit = defineEmits<{
    (e: "save"): void;
    (e: "back"): void;
}>();

// 🌟 使用 defineModel 自動接管 v-model:detail 的雙向綁定
const detail = defineModel<string>("detail", { default: "" });

const textareaRef = ref<HTMLTextAreaElement>();

function insertHtml(html: string, range?: { start: number; end: number }) {
    const content = detail.value ?? "";
    const start = Math.min(
        Math.max(range?.start ?? content.length, 0),
        content.length,
    );
    const end = Math.min(
        Math.max(range?.end ?? range?.start ?? content.length, start),
        content.length,
    );
    // 直接修改 detail.value，Vue 會自動 emit 更新給父組件
    detail.value = content.slice(0, start) + html + content.slice(end);
}

const renderedDetail = computed(() => {
    const content = detail.value ?? "";
    if (!content) return "";
    const base = BASE_URL.replace(/\/$/, "");
    return content.replace(
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
                v-model="detail"
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
