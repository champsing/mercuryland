<script setup lang="ts">
import { ref, computed, nextTick } from "vue";
import { VaInput, VaModal, VaButton, VaIcon } from "vuestic-ui";
import { BrandSteam } from "@vicons/tabler";

const props = defineProps<{
    textareaRef: any; // Ref to the VaTextarea component instance or element
}>();

const emit = defineEmits<{
    insertHtml: [html: string, range: { start: number; end: number }];
}>();

const isModalOpen = ref(false);
const steamId = ref("");
const selectionRange = ref<{ start: number; end: number } | null>(null);

const steamWidgetUrl = computed(() => {
    const id = steamId.value.trim();
    if (!id) return "";
    return `https://store.steampowered.com/widget/${id}/`;
});

const steamHtml = computed(() => {
    const url = steamWidgetUrl.value;
    if (!url) return "";
    const container = [
        '<div class="steam-widget" style="max-width: 100%; min-height: 200px; overflow: hidden;">',
        `    <iframe src="${url}" frameborder="0" width="100%" height="200" scrolling="no"></iframe>`,
        "</div>",
    ].join("\n");

    return `\n${container}\n`;
});

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

    selectionRange.value = {
        start,
        end,
    };
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
    // Preserve the selection on the textarea before focus leaves it
    captureSelection();
    event.preventDefault();
}

function openModal() {
    captureSelection();
    isModalOpen.value = true;
    steamId.value = "";
}

function closeModal() {
    isModalOpen.value = false;
}

function save() {
    const html = steamHtml.value;
    if (!html) return;

    const range = selectionRange.value ?? getSelectionFallback();
    const start = Math.max(range.start, 0);
    const end = Math.max(range.end, start);

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
        color="#2a475e"
        class="w-full h-full"
    >
        <VaIcon class="mr-2">
            <BrandSteam />
        </VaIcon>
        添加游戏
    </VaButton>

    <VaModal
        v-model="isModalOpen"
        @update:model-value="closeModal"
        hide-default-actions
        size="small"
        close-button
    >
        <div class="space-y-4">
            <VaInput
                v-model="steamId"
                label="Steam游戏ID"
                placeholder="请输入Steam游戏ID，例如：289070"
                class="w-full"
            />

            <div>
                <label class="block text-sm font-medium mb-2">预览:</label>
                <div class="border border-slate-200 rounded-lg p-2">
                    <iframe
                        v-if="steamWidgetUrl"
                        :src="steamWidgetUrl"
                        frameborder="0"
                        scrolling="no"
                        class="w-full h-[200px]"
                    ></iframe>
                    <div
                        v-else
                        class="h-[200px] grid place-content-center text-sm text-slate-400"
                    >
                        输入Steam游戏ID后显示预览
                    </div>
                </div>
            </div>

            <div class="flex justify-end gap-2">
                <VaButton @click="closeModal" color="secondary">
                    取消
                </VaButton>
                <VaButton
                    @click="save"
                    color="primary"
                    :disabled="!steamId.trim()"
                >
                    保存
                </VaButton>
            </div>
        </div>
    </VaModal>
</template>
