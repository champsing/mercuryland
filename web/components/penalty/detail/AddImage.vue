<script setup lang="ts">
import { nextTick, onBeforeUnmount, ref } from "vue";
import { VaAlert, VaButton, VaIcon, VaInput, VaModal } from "vuestic-ui";
import axios, { isAxiosError } from "axios";
import { BASE_URL } from "@/composables/utils";
import { Photo } from "@vicons/tabler";

const props = defineProps<{
    textareaRef: any;
}>();

const emit = defineEmits<{
    insertHtml: [html: string, range: { start: number; end: number }];
}>();

const isModalOpen = ref(false);
const isUploading = ref(false);
const uploadError = ref<string | null>(null);
const selectedFile = ref<File | null>(null);
const previewUrl = ref<string | null>(null);
const imageAlt = ref("");
const selectionRange = ref<{ start: number; end: number } | null>(null);
const fileInputRef = ref<HTMLInputElement | null>(null);

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

function resetState() {
    selectedFile.value = null;
    imageAlt.value = "";
    uploadError.value = null;
    isUploading.value = false;
    revokePreviewUrl();
    if (fileInputRef.value) {
        fileInputRef.value.value = "";
    }
}

function openModal() {
    captureSelection();
    resetState();
    isModalOpen.value = true;
}

function closeModal() {
    isModalOpen.value = false;
    resetState();
}

function revokePreviewUrl() {
    if (previewUrl.value) {
        URL.revokeObjectURL(previewUrl.value);
        previewUrl.value = null;
    }
}

function onFileChange(event: Event) {
    const target = event.target as HTMLInputElement | null;
    const file = target?.files?.[0] ?? null;
    revokePreviewUrl();
    selectedFile.value = file;
    if (file) {
        previewUrl.value = URL.createObjectURL(file);
    }
}

function ensureAbsoluteUrl(url: string) {
    if (/^https?:\/\//i.test(url)) {
        return url;
    }

    const base = BASE_URL.replace(/\/$/, "");
    const path = url.startsWith("/") ? url : `/${url}`;
    return `${base}${path}`;
}

function escapeHtml(value: string) {
    return value
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#39;");
}

async function save() {
    if (!selectedFile.value) {
        uploadError.value = "請選擇圖片";
        return;
    }

    const token = localStorage.getItem("token");
    if (!token) {
        uploadError.value = "需要登入後才能上傳";
        return;
    }

    const formData = new FormData();
    formData.append("token", token);
    formData.append("file", selectedFile.value);

    try {
        isUploading.value = true;
        uploadError.value = null;

        const response = await axios.post<{ url: string }>(
            `${BASE_URL}/api/image/upload`,
            formData,
        );

        const { url } = response.data ?? {};
        if (!url) {
            uploadError.value = "無法取得圖片位置";
            return;
        }

        const finalUrl = ensureAbsoluteUrl(url);
        const altText = imageAlt.value.trim();
        const escapedAlt = escapeHtml(altText);

        const htmlSegments = [
            "",
            '<div class="penalty-image" style="margin: 1rem 0; text-align: center;">',
            `  <img src="${finalUrl}" alt="${escapedAlt}" style="max-width: 100%; border-radius: 12px;" />`,
            "</div>",
            "",
        ];
        const html = htmlSegments.join("\n");

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
    } catch (error) {
        console.error("Failed to upload image", error);
        if (isAxiosError(error)) {
            const status = error.response?.status;
            if (status === 413) {
                uploadError.value = "圖片大小超過 1 MB 限制";
            } else if (typeof error.response?.data === "string") {
                uploadError.value = error.response.data;
            } else {
                uploadError.value = "上傳失敗，請稍後再試";
            }
        } else {
            uploadError.value = "上傳失敗，請稍後再試";
        }
    } finally {
        isUploading.value = false;
    }
}

onBeforeUnmount(() => {
    revokePreviewUrl();
});
</script>

<template>
    <VaButton
        @mousedown="onTriggerMouseDown"
        @click="openModal"
        color="#2563eb"
        class="w-full h-full"
        gradient
    >
        <VaIcon class="mr-2">
            <Photo />
        </VaIcon>
        添加圖片
    </VaButton>

    <VaModal
        v-model="isModalOpen"
        @update:model-value="closeModal"
        hide-default-actions
        size="small"
        close-button
    >
        <div class="flex flex-col gap-4">
            <VaAlert v-if="uploadError" color="danger" outline>
                {{ uploadError }}
            </VaAlert>

            <div>
                <label class="block text-sm font-medium mb-2">選擇圖片</label>
                <input
                    ref="fileInputRef"
                    type="file"
                    accept="image/*"
                    @change="onFileChange"
                    class="block w-full text-sm text-slate-200 file:mr-4 file:rounded-md file:border-0 file:bg-slate-700 file:px-4 file:py-2 file:text-sm file:font-semibold hover:file:bg-slate-600"
                />
                <p class="mt-1 text-xs text-slate-400">
                    檔案限制 1 MB，僅支援圖片格式
                </p>
            </div>

            <VaInput
                v-model="imageAlt"
                label="圖片說明 (可選)"
                placeholder="將作為 alt 文字"
                clearable
            />

            <div>
                <label class="block text-sm font-medium mb-2">預覽</label>
                <div
                    class="border border-slate-700 rounded-lg p-2 min-h-[220px] grid place-content-center bg-slate-900/30"
                >
                    <img
                        v-if="previewUrl"
                        :src="previewUrl"
                        :alt="imageAlt"
                        class="max-h-[200px] object-contain"
                    />
                    <div v-else class="text-sm text-slate-400">
                        尚未選擇圖片
                    </div>
                </div>
            </div>

            <div class="flex justify-end gap-2">
                <VaButton
                    @click="closeModal"
                    preset="secondary"
                    :disabled="isUploading"
                >
                    取消
                </VaButton>
                <VaButton
                    color="primary"
                    @click="save"
                    :loading="isUploading"
                    :disabled="!selectedFile || isUploading"
                >
                    上傳並插入
                </VaButton>
            </div>
        </div>
    </VaModal>
</template>
