<script setup lang="ts">
import { ref, computed, nextTick } from "vue";
import { VaButton, VaModal, VaSelect, VaAlert, VaIcon } from "vuestic-ui";
import axios from "axios";
import { BASE_URL } from "@/composables/utils";
import { PenItem, stateString } from "@/composables/penalty";
import { Gavel } from "@vicons/tabler"; // 法槌 icon

const props = defineProps<{
    textareaRef: any;
}>();

const emit = defineEmits<{
    insertHtml: [html: string, range: { start: number; end: number }];
    changePenaltyId: [id: number | null];
}>();

const isModalOpen = ref(false);
const isLoading = ref(false);
const loadError = ref<string | null>(null);
const penalties = ref<PenItem[]>([]);
const selectedPenaltyId = ref<number | null>(null);
const selectionRange = ref<{ start: number; end: number } | null>(null);

const penaltyOptions = computed(() =>
    penalties.value.map((pen) => ({
        value: pen.id,
        text: `[${pen.date} ${stateString(pen.state)}] ${pen.name}`,
    })),
);

const selectedPenalty = computed(
    () => penalties.value.find((p) => p.id === selectedPenaltyId.value) ?? null,
);

async function ensurePenaltiesLoaded() {
    if (isLoading.value) return;
    if (penalties.value.length > 0) return;

    try {
        isLoading.value = true;
        loadError.value = null;
        const response = await axios.get(`${BASE_URL}/api/penalty/list`);
        penalties.value = response.data ?? [];
    } catch (error) {
        console.error("Failed to load penalties", error);
        loadError.value = "載入懲罰列表失敗";
    } finally {
        isLoading.value = false;
    }
}

function resolveTextarea(): HTMLTextAreaElement | null {
    const candidate = props.textareaRef as any;
    if (!candidate) return null;
    if (candidate instanceof HTMLTextAreaElement) return candidate;

    const el = candidate?.$el?.querySelector?.("textarea");
    if (el instanceof HTMLTextAreaElement) return el;

    const refs = candidate?.$refs;
    if (refs) {
        const possible = [refs.textarea, refs.input, refs.inputElement];
        for (const e of possible)
            if (e instanceof HTMLTextAreaElement) return e;
    }

    return null;
}

function captureSelection() {
    const textarea = resolveTextarea();
    if (!textarea) {
        selectionRange.value = null;
        return;
    }
    selectionRange.value = {
        start: textarea.selectionStart ?? 0,
        end: textarea.selectionEnd ?? 0,
    };
}

function getSelectionFallback() {
    const textarea = resolveTextarea();
    if (!textarea) return { start: 0, end: 0 };
    return {
        start: textarea.selectionStart ?? 0,
        end: textarea.selectionEnd ?? 0,
    };
}

function onTriggerMouseDown(e: MouseEvent) {
    captureSelection();
    e.preventDefault();
}

function openModal() {
    captureSelection();
    selectedPenaltyId.value = null;
    isModalOpen.value = true;
    ensurePenaltiesLoaded();
}

function closeModal() {
    isModalOpen.value = false;
}

const embedHtml = computed(() => {
    const pen = selectedPenalty.value;
    if (!pen) return "";

    const escapedName = pen.name.replace(/</g, "&lt;").replace(/>/g, "&gt;");

    // const penState = ;

    return `\n
            <div class="text-center my-2">
                <button
                    class="inline items-center px-3 py-1.5 bg-teal-700 hover:bg-teal-400 text-white rounded-2xl text-sm font-semibold transition-colors"
                >
                    懲罰：【${pen.date} ${stateString(pen.state)}】${escapedName}
                </button>
            </div>
            \n`;
});

const isInsertDisabled = computed(() => !selectedPenalty.value);

function save() {
    if (isInsertDisabled.value) return;

    const range = selectionRange.value ?? getSelectionFallback();
    const start = Math.max(range.start, 0);
    const end = Math.max(range.end, start);

    const html = embedHtml.value;
    emit("insertHtml", html, { start, end });
    emit("changePenaltyId", selectedPenalty.value?.id ?? null);

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
        color="warning"
        class="w-full h-full"
        gradient
    >
        <VaIcon class="mr-2"><Gavel /></VaIcon>
        插入懲罰
    </VaButton>

    <VaModal
        v-model="isModalOpen"
        @update:model-value="closeModal"
        hide-default-actions
        close-button
        max-width="500px"
    >
        <div class="flex flex-col gap-4">
            <div class="text-lg font-semibold text-zinc-200">選擇懲罰項目</div>

            <VaAlert v-if="loadError" color="danger" outline class="text-sm">
                {{ loadError }}
            </VaAlert>

            <VaSelect
                v-model="selectedPenaltyId"
                :options="penaltyOptions"
                value-by="value"
                text-by="text"
                placeholder="選擇懲罰"
                label="懲罰"
                searchable
                clearable
                :loading="isLoading"
                :disabled="Boolean(loadError)"
            />

            <div
                v-if="selectedPenalty"
                class="border rounded-md p-2 text-sm bg-zinc-900/20"
            >
                <div><b>名稱：</b>{{ selectedPenalty.name }}</div>
                <div><b>日期：</b>{{ selectedPenalty.date }}</div>
                <div><b>狀態：</b>{{ stateString(selectedPenalty.state) }}</div>
            </div>

            <div class="flex justify-end gap-2">
                <VaButton preset="secondary" @click="closeModal">取消</VaButton>
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
