<script setup lang="ts">
import { computed, nextTick, ref } from "vue";
import { VaButton, VaModal, VaSelect, VaTextarea, VaIcon } from "vuestic-ui";
import { InfoCircle } from "@vicons/tabler";

const props = defineProps<{
    textareaRef: any;
}>();

const emit = defineEmits<{
    insertHtml: [html: string, range: { start: number; end: number }];
}>();

const syntaxOptions = [
    "🆙增加",
    "🔁重抽",
    "2️⃣備案",
    "😇復活",
    "📝修改",
    "✅已抽",
    "➕其他",
];

const isModalOpen = ref(false);
const selectedSyntax = ref<string | null>(syntaxOptions[0] ?? null);
const syntaxDetail = ref("");
const selectionRange = ref<{ start: number; end: number } | null>(null);

function resolveTextarea(): HTMLTextAreaElement | null {
    const candidate = props.textareaRef as any;
    if (!candidate) return null;

    if (candidate instanceof HTMLTextAreaElement) {
        return candidate;
    }

    const directEl = candidate?.$el?.querySelector?.("textarea");
    if (directEl instanceof HTMLTextAreaElement) {
        return directEl;
    }

    const refs = candidate?.$refs;
    if (refs) {
        const possible = [refs.textarea, refs.input, refs.inputElement];
        for (const el of possible) {
            if (el instanceof HTMLTextAreaElement) {
                return el;
            }
        }
    }

    return null;
}

function captureSelection() {
    const textarea = resolveTextarea();
    if (!textarea) {
        selectionRange.value = null;
        return;
    }

    const start = textarea.selectionStart ?? textarea.value.length ?? 0;
    const end = textarea.selectionEnd ?? textarea.value.length ?? start;

    selectionRange.value = { start, end };
}

function getSelectionFallback() {
    const textarea = resolveTextarea();
    if (!textarea) {
        return { start: 0, end: 0 };
    }

    const start = textarea.selectionStart ?? textarea.value.length ?? 0;
    const end = textarea.selectionEnd ?? textarea.value.length ?? start;

    return { start, end };
}

function onTriggerMouseDown(event: MouseEvent) {
    captureSelection();
    event.preventDefault();
}

function openModal() {
    captureSelection();
    syntaxDetail.value = "";
    isModalOpen.value = true;
}

function closeModal() {
    isModalOpen.value = false;
}

function escapeHtml(value: string) {
    return value
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#39;");
}

const syntaxHtml = computed(() => {
    const syntax = selectedSyntax.value;
    if (!syntax) {
        return "";
    }

    const detail = syntaxDetail.value.trim();
    const escapedDetail = escapeHtml(detail).replace(/\n/g, "<br />");

    const container = `<div style="border: 1px solid grey; border-radius: 12px; margin: 1rem 0; padding: 0.5rem;">${syntax}: ${escapedDetail}</div>`;

    return `\n${container}\n`;
});

const isInsertDisabled = computed(() => !selectedSyntax.value);

function save() {
    if (isInsertDisabled.value) {
        return;
    }

    const range = selectionRange.value ?? getSelectionFallback();
    const start = Math.max(range.start, 0);
    const end = Math.max(range.end, start);

    const html = syntaxHtml.value;
    emit("insertHtml", html, { start, end });

    const insertionEnd = start + html.length;
    selectionRange.value = { start: insertionEnd, end: insertionEnd };

    closeModal();

    nextTick(() => {
        const textarea = resolveTextarea();
        if (!textarea) return;
        textarea.focus();
        textarea.setSelectionRange(insertionEnd, insertionEnd);
    });
}
</script>

<template>
    <VaButton
        @mousedown="onTriggerMouseDown"
        @click="openModal"
        color="#28c9c7"
        class="w-full h-full"
        gradient
    >
        <VaIcon class="mr-2">
            <InfoCircle />
        </VaIcon>
        詳細資料
    </VaButton>

    <VaModal
        v-model="isModalOpen"
        @update:model-value="closeModal"
        hide-default-actions
        size="small"
        close-button
    >
        <div class="flex flex-col gap-4">
            <VaSelect
                v-model="selectedSyntax"
                :options="syntaxOptions"
                label="選擇圖示"
                placeholder="請選擇"
                :clearable="false"
            />

            <VaTextarea
                v-model="syntaxDetail"
                label="內容"
                placeholder="請輸入詳細內容"
                min-rows="4"
                max-rows="6"
                autosize
            />

            <div class="flex justify-end gap-2">
                <VaButton preset="secondary" @click="closeModal">
                    取消
                </VaButton>
                <VaButton
                    color="primary"
                    :disabled="isInsertDisabled"
                    @click="save"
                >
                    插入
                </VaButton>
            </div>
        </div>
    </VaModal>
</template>
