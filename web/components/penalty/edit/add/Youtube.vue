<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import {
    VaAlert,
    VaButton,
    VaIcon,
    VaInput,
    VaModal,
    VaSelect,
    VaSwitch,
} from "vuestic-ui";
import api from "@composables/axios";

import type { VodItem } from "@/composables/vod";
import { BrandYoutube } from "@vicons/tabler";

const props = defineProps<{
    textareaRef: any;
}>();

const emit = defineEmits<{
    insertHtml: [html: string, range: { start: number; end: number }];
}>();

const isModalOpen = ref(false);
const isLoading = ref(false);
const loadError = ref<string | null>(null);
const videos = ref<VodItem[]>([]);
const selectedVideoId = ref<number | null>(null);
const selectionRange = ref<{ start: number; end: number } | null>(null);
const videoTimestamp = ref("");
const useManualVideo = ref(false);
const manualVideoId = ref("");

// 🆕 插入模式：iframe 或 button
const insertMode = ref<"iframe" | "button">("iframe");

const insertModeOptions = [
    { text: "嵌入視窗 (iframe)", value: "iframe" },
    { text: "按鈕開啟連結", value: "button" },
];

const videoOptions = computed(() =>
    [...videos.value]
        .sort((a, b) => (b.date ?? "").localeCompare(a.date ?? ""))
        .map((video) => {
            const date = video.date ?? "未設定";
            const title = video.title ?? "(無標題)";
            return {
                value: video.id,
                text: `[${date}] ${title}`,
            };
        }),
);

const selectedVideo = computed(
    () =>
        videos.value.find((video) => video.id === selectedVideoId.value) ??
        null,
);

const sanitizedManualVideoId = computed(() => manualVideoId.value.trim());

const activeVideoLink = computed(() => {
    if (useManualVideo.value) {
        return sanitizedManualVideoId.value || null;
    }
    return selectedVideo.value?.link ?? null;
});

const previewUrl = computed(() => {
    const link = activeVideoLink.value;
    if (!link) return "";
    const base = `https://www.youtube.com/embed/${link}`;
    const startSeconds = timestampInfo.value.seconds;
    if (startSeconds !== null) {
        const clampedStart = Math.max(0, startSeconds);
        const separator = base.includes("?") ? "&" : "?";
        return `${base}${separator}start=${clampedStart}`;
    }
    return base;
});

function parseTimestampToSeconds(value: string): number | null {
    const trimmed = value.trim();
    if (!trimmed) {
        return null;
    }

    if (/^\d+$/.test(trimmed)) {
        return Number.parseInt(trimmed, 10);
    }

    if (/^\d{1,2}:\d{1,2}(?::\d{1,2})?$/.test(trimmed)) {
        const parts = trimmed
            .split(":")
            .map((part) => Number.parseInt(part, 10));
        if (parts.some((part) => Number.isNaN(part))) {
            return null;
        }
        if (parts.length === 3) {
            return parts[0] * 3600 + parts[1] * 60 + parts[2];
        }
        if (parts.length === 2) {
            return parts[0] * 60 + parts[1];
        }
        if (parts.length === 1) {
            return parts[0];
        }
    }

    return null;
}

const timestampInfo = computed(() => {
    const trimmed = videoTimestamp.value.trim();
    if (!trimmed) {
        return { seconds: null, error: null } as const;
    }

    const parsed = parseTimestampToSeconds(trimmed);

    if (parsed === null || Number.isNaN(parsed) || parsed < 0) {
        return {
            seconds: null,
            error: "時間格式錯誤，請輸入秒數或 HH:MM:SS",
        } as const;
    }

    return { seconds: parsed, error: null } as const;
});

// 🆕 根據插入模式動態產生 HTML
const embedHtml = computed(() => {
    const link = activeVideoLink.value;
    if (!link) {
        return "";
    }

    const video = selectedVideo.value;
    const startSeconds = timestampInfo.value.seconds;
    const baseUrl = `https://www.youtube.com/watch?v=${link}`;
    const url = startSeconds ? `${baseUrl}&t=${startSeconds}s` : baseUrl;

    const escapeHtml = (value: string) =>
        value
            .replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;")
            .replace(/"/g, "&quot;")
            .replace(/'/g, "&#39;");

    const iframeTitleSource = useManualVideo.value
        ? sanitizedManualVideoId.value || "YouTube video"
        : (video?.title ?? "YouTube video");
    const escapedTitle = escapeHtml(iframeTitleSource);

    // iframe 模式
    if (insertMode.value === "iframe") {
        const embedUrl = `https://www.youtube.com/embed/${link}`;
        const embedSrc =
            startSeconds !== null
                ? `${embedUrl}?start=${Math.max(0, startSeconds)}`
                : embedUrl;
        return `\n<div class="penalty-youtube" style="margin: 1rem 0;">
                <div style="position: relative; padding-bottom: 56.25%; height: 0; overflow: hidden; border-radius: 12px; background-color: #000;">
                    <iframe src="${embedSrc}" title="${escapedTitle}" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen style="position: absolute; inset: 0; width: 100%; height: 100%; border: 0;"></iframe>
                </div>
                </div>\n`;
    }

    if (video) {
        // 按鈕模式列表直播
        return `\n<div class="youtube-link-button text-center my-4">
            <a href="${url}"
                target="_blank"
                rel="noopener noreferrer"
                class="inline items-center text-sm px-2 py-1 bg-red-600 hover:bg-red-700 text-white rounded-2xl font-semibold no-underline transition-colors">
                直播：【${video.date}】${escapedTitle}
            </a>
            </div>\n`;
    }

    // 按鈕模式非列表直播
    return `\n<div class="youtube-link-button text-center my-4">
            <a href="${url}"
                target="_blank"
                rel="noopener noreferrer"
                class="inline items-center text-sm px-2 py-1 bg-red-600 hover:bg-red-700 text-white rounded-2xl font-semibold no-underline transition-colors">
                影片：${escapedTitle}
            </a>
            </div>\n`;
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

async function ensureVideosLoaded() {
    if (isLoading.value) return;
    if (videos.value.length > 0) return;

    try {
        isLoading.value = true;
        loadError.value = null;
        const response = await api.get<VodItem[]>(`/api/video/list`);
        videos.value = response.data
            .map((video) => ({
                ...video,
                tags: video.tags ?? [],
            }))
            .sort((a, b) => (b.date ?? "").localeCompare(a.date ?? ""));
    } catch (error) {
        console.error("Failed to load videos", error);
        loadError.value = "載入影片列表失敗";
    } finally {
        isLoading.value = false;
    }
}

function openModal() {
    captureSelection();
    selectedVideoId.value = null;
    videoTimestamp.value = "";
    manualVideoId.value = "";
    useManualVideo.value = false;
    insertMode.value = "iframe";
    isModalOpen.value = true;
    ensureVideosLoaded();
}

function closeModal() {
    isModalOpen.value = false;
}

watch(useManualVideo, (manual) => {
    if (manual) {
        selectedVideoId.value = null;
    } else {
        manualVideoId.value = "";
    }
});

const isInsertDisabled = computed(() => {
    const hasVideo = useManualVideo.value
        ? sanitizedManualVideoId.value.length > 0
        : Boolean(selectedVideo.value);
    return !hasVideo || Boolean(timestampInfo.value.error) || !embedHtml.value;
});

function save() {
    if (isInsertDisabled.value) return;

    const range = selectionRange.value ?? getSelectionFallback();
    const start = Math.max(range.start, 0);
    const end = Math.max(range.end, start);

    const html = embedHtml.value;
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
        color="#ff0000"
        class="w-full h-full"
        gradient
    >
        <VaIcon class="mr-2"><BrandYoutube /></VaIcon>
        添加影片
    </VaButton>

    <VaModal
        v-model="isModalOpen"
        @update:model-value="closeModal"
        hide-default-actions
        close-button
        :mobile-fullscreen="false"
        max-width="560px"
    >
        <div class="flex flex-col gap-4">
            <div class="text-lg font-semibold text-zinc-200">選擇影片</div>

            <VaAlert v-if="loadError" color="danger" outline class="text-sm">
                {{ loadError }}
            </VaAlert>

            <div class="flex items-end gap-2 overflow-hidden">
                <VaSelect
                    v-if="!useManualVideo"
                    v-model="selectedVideoId"
                    :options="videoOptions"
                    value-by="value"
                    text-by="text"
                    placeholder="選擇要嵌入的影片"
                    label="影片"
                    searchable
                    clearable
                    :loading="isLoading"
                    :disabled="Boolean(loadError)"
                    class="flex-grow"
                />
                <VaInput
                    v-else
                    label="影片"
                    v-model="manualVideoId"
                    placeholder="輸入 YouTube 影片 ID，例如：ms8uu0zeU88"
                    clearable
                    :disabled="Boolean(loadError)"
                    class="flex-grow"
                />
                <VaSwitch
                    v-model="useManualVideo"
                    size="small"
                    color="info"
                    false-inner-label="列表"
                    true-inner-label="链接"
                    :disabled="Boolean(loadError)"
                    class="mb-1.5 mr-1"
                />
            </div>

            <!-- 🆕 插入模式 -->
            <VaSelect
                v-model="insertMode"
                :options="insertModeOptions"
                label="插入方式"
                class="w-full"
            />

            <VaInput
                v-model="videoTimestamp"
                label="起始時間 (可選)"
                placeholder="例：90 或 01:30 或 01:02:03"
                clearable
                :disabled="Boolean(loadError)"
                :error="Boolean(timestampInfo.error)"
                :error-messages="
                    timestampInfo.error ? [timestampInfo.error] : []
                "
                messages="空白則從頭播放"
            />

            <div v-if="insertMode === 'iframe'">
                <label class="block text-sm font-medium mb-2">預覽:</label>
                <div class="border border-slate-200 rounded-lg p-2">
                    <iframe
                        v-if="previewUrl"
                        :src="previewUrl"
                        frameborder="0"
                        allow="
                            accelerometer;
                            autoplay;
                            clipboard-write;
                            encrypted-media;
                            gyroscope;
                            picture-in-picture;
                            web-share;
                        "
                        allowfullscreen
                        class="w-full h-[300px] rounded-xl"
                    ></iframe>
                    <div
                        v-else
                        class="h-[300px] grid place-content-center text-sm text-slate-400"
                    >
                        選擇影片後顯示預覽
                    </div>
                </div>
            </div>

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
