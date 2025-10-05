<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import {
    VaButton,
    VaChip,
    VaDateInput,
    VaInput,
    VaModal,
    VaSelect,
    VaTimeInput,
} from "vuestic-ui";
import axios from "axios";
import { BASE_URL, formatDate, parseDate } from "@/composables/utils";
import { VodItem } from "@/composables/vod";

const props = defineProps<{
    tagList: string[];
    isAuthenticated?: boolean;
}>();

const emit = defineEmits<{
    (event: "updated"): void;
    (event: "deleted"): void;
}>();

const showEditVodModal = ref(false);
const showDeleteConfirmModal = ref(false);
const editVodForm = reactive({
    id: 0 as number,
    date: new Date(),
    link: "",
    title: "",
});
const editVodDuration = ref<Date | null>(null);
const editVodTags = ref<string[]>([]);
const editVodCustomTags = ref<string[]>([]);
const editVodError = ref<string | null>(null);
const editVodSuccess = ref<string | null>(null);
const isSavingVod = ref(false);
const isDeletingVod = ref(false);
const deleteVodError = ref<string | null>(null);

const editVodTagOptions = computed(() => {
    const uniqueTags = new Set(props.tagList ?? []);
    editVodCustomTags.value.forEach((tag) => {
        if (tag) {
            uniqueTags.add(tag);
        }
    });
    editVodTags.value.forEach((tag) => {
        if (tag) {
            uniqueTags.add(tag);
        }
    });
    return Array.from(uniqueTags).sort();
});

watch(showEditVodModal, (visible) => {
    if (!visible) {
        resetEditVodForm();
    }
});

const resetEditVodForm = () => {
    editVodForm.id = 0;
    editVodForm.date = new Date();
    editVodForm.link = "";
    editVodForm.title = "";
    editVodTags.value = [];
    editVodCustomTags.value = [];
    editVodDuration.value = null;
    editVodError.value = null;
    editVodSuccess.value = null;
    showDeleteConfirmModal.value = false;
    deleteVodError.value = null;
    isDeletingVod.value = false;
};

const searchTagsByPrefix = (search: string, option: unknown) => {
    if (!search) {
        return true;
    }
    const normalizedSearch = search.toLowerCase();
    const optionText = (() => {
        if (typeof option === "string" || typeof option === "number") {
            return option.toString();
        }
        if (option && typeof option === "object") {
            const candidate =
                (option as Record<string, unknown>).label ??
                (option as Record<string, unknown>).text ??
                (option as Record<string, unknown>).value;
            if (candidate == null) {
                return "";
            }
            if (
                typeof candidate === "string" ||
                typeof candidate === "number"
            ) {
                return candidate.toString();
            }
        }
        return "";
    })();

    return optionText.toLowerCase().startsWith(normalizedSearch);
};

const removeEditVodTag = (tag: string) => {
    editVodTags.value = editVodTags.value.filter((value) => value !== tag);
};

const handleCreateVodTag = (tag: string) => {
    const trimmed = tag.trim();
    if (!trimmed) {
        return;
    }

    if (!editVodTags.value.includes(trimmed)) {
        editVodTags.value = [...editVodTags.value, trimmed];
    }

    if (
        !editVodCustomTags.value.includes(trimmed) &&
        !props.tagList.includes(trimmed)
    ) {
        editVodCustomTags.value = [...editVodCustomTags.value, trimmed];
    }
};

const parseDurationToDate = (duration: string): Date | null => {
    const [hours, minutes, seconds] = duration.split(":").map(Number);
    if (
        [hours, minutes, seconds].some(
            (value) => value === undefined || Number.isNaN(value),
        )
    ) {
        return null;
    }
    const date = new Date();
    date.setHours(hours, minutes, seconds, 0);
    return date;
};

const open = (vod: VodItem) => {
    if (!props.isAuthenticated) {
        return;
    }

    editVodForm.id = vod.id;
    editVodForm.link = vod.link ?? "";
    editVodForm.title = vod.title ?? "";
    editVodForm.date = parseDate(vod.date);
    editVodTags.value = [...(vod.tags ?? [])];
    editVodCustomTags.value = [];
    editVodDuration.value = parseDurationToDate(vod.duration ?? "00:00:00");
    editVodError.value = null;
    editVodSuccess.value = null;
    deleteVodError.value = null;
    showDeleteConfirmModal.value = false;

    showEditVodModal.value = true;
};

const formatDuration = (value: Date) => {
    const pad = (num: number) => num.toString().padStart(2, "0");
    const hours = pad(value.getHours());
    const minutes = pad(value.getMinutes());
    const seconds = pad(value.getSeconds());
    return `${hours}:${minutes}:${seconds}`;
};

const saveVod = async () => {
    if (isSavingVod.value || !props.isAuthenticated) {
        return;
    }

    const token = localStorage.getItem("token");
    if (!token) {
        editVodError.value = "請先登入管理員帳號";
        return;
    }

    if (
        !editVodForm.date ||
        !editVodForm.title.trim() ||
        !editVodDuration.value
    ) {
        editVodError.value = "請填寫所有必填欄位";
        return;
    }

    const tags = Array.from(
        new Set(
            editVodTags.value
                .map((tag) => tag.trim())
                .filter((tag) => tag.length > 0),
        ),
    );

    const payload = {
        token,
        id: editVodForm.id,
        date: formatDate(editVodForm.date),
        title: editVodForm.title.trim(),
        tags,
        duration: formatDuration(editVodDuration.value),
    };

    try {
        isSavingVod.value = true;
        editVodError.value = null;
        await axios.post(`${BASE_URL}/api/video/update`, payload);
        editVodSuccess.value = "更新成功";
        emit("updated");
        setTimeout(() => {
            showEditVodModal.value = false;
        }, 600);
    } catch (error) {
        console.error("Failed to update VOD", error);
        editVodError.value = "更新失敗，請稍後再試";
    } finally {
        isSavingVod.value = false;
    }
};

const requestDeleteVod = () => {
    deleteVodError.value = null;
    showDeleteConfirmModal.value = true;
};

const deleteVod = async () => {
    if (isDeletingVod.value || !props.isAuthenticated) {
        return;
    }

    const token = localStorage.getItem("token");
    if (!token) {
        deleteVodError.value = "請先登入管理員帳號";
        return;
    }

    try {
        isDeletingVod.value = true;
        deleteVodError.value = null;
        await axios.post(`${BASE_URL}/api/video/delete`, {
            token,
            id: editVodForm.id,
        });
        emit("deleted");
        showDeleteConfirmModal.value = false;
        showEditVodModal.value = false;
    } catch (error) {
        console.error("Failed to delete VOD", error);
        deleteVodError.value = "刪除失敗，請稍後再試";
    } finally {
        isDeletingVod.value = false;
    }
};

const closeDeleteModal = () => {
    showDeleteConfirmModal.value = false;
};

defineExpose({
    open,
});
</script>

<template>
    <VaModal
        v-model="showEditVodModal"
        hide-default-actions
        close-button
        max-width="480px"
    >
        <div class="flex flex-col gap-4 p-4">
            <div class="text-lg font-semibold text-zinc-200">編輯直播紀錄</div>
            <VaInput
                v-model="editVodForm.link"
                label="YouTube 連結代碼"
                color="info"
                readonly
                input-class="cursor-not-allowed text-gray-400 bg-gray-700/60"
            />
            <VaDateInput
                v-model="editVodForm.date"
                label="日期"
                color="info"
                :format-date="formatDate"
                :parse-date="parseDate"
                manual-input
                mode="auto"
            />
            <VaInput
                v-model="editVodForm.title"
                label="直播標題"
                color="info"
                required
            />
            <VaTimeInput
                v-model="editVodDuration"
                label="直播時長"
                color="info"
                :ampm="false"
                :hide-period-switch="true"
                :manual-input="true"
                view="seconds"
            />
            <VaSelect
                v-model="editVodTags"
                label="標籤"
                color="info"
                :options="editVodTagOptions"
                multiple
                clearable
                searchable
                allow-create
                @create-new="handleCreateVodTag"
                :search-fn="searchTagsByPrefix"
                placeholder="請選擇或輸入標籤"
            >
                <template #content>
                    <VaChip
                        v-for="chip in editVodTags"
                        :key="chip"
                        color="#90dc52"
                        outline
                        size="small"
                        class="mr-1 my-1"
                        closeable
                        @update:model-value="removeEditVodTag(chip)"
                    >
                        {{ chip }}
                    </VaChip>
                </template>
            </VaSelect>
            <p v-if="editVodError" class="text-sm text-red-400">
                {{ editVodError }}
            </p>
            <p v-if="editVodSuccess" class="text-sm text-emerald-400">
                {{ editVodSuccess }}
            </p>
            <div class="flex flex-col gap-2">
                <VaButton
                    color="danger"
                    preset="primary"
                    class="w-full"
                    @click="requestDeleteVod"
                >
                    刪除直播
                </VaButton>
                <div class="flex justify-end gap-2">
                    <VaButton
                        preset="secondary"
                        :disabled="isSavingVod"
                        @click="showEditVodModal = false"
                    >
                        取消
                    </VaButton>
                    <VaButton
                        color="info"
                        :loading="isSavingVod"
                        @click="saveVod"
                    >
                        儲存
                    </VaButton>
                </div>
            </div>
        </div>

        <VaModal
            v-model="showDeleteConfirmModal"
            hide-default-actions
            close-button
            max-width="360px"
        >
            <div class="flex flex-col gap-4 p-4">
                <div class="text-lg font-semibold text-red-300">
                    確認刪除直播？
                </div>
                <p class="text-sm text-gray-200">
                    此操作無法復原，確定要刪除這筆直播紀錄嗎？
                </p>
                <p v-if="deleteVodError" class="text-sm text-red-400">
                    {{ deleteVodError }}
                </p>
                <div class="flex justify-end gap-2">
                    <VaButton
                        preset="secondary"
                        :disabled="isDeletingVod"
                        @click="closeDeleteModal"
                    >
                        取消
                    </VaButton>
                    <VaButton
                        color="danger"
                        :loading="isDeletingVod"
                        @click="deleteVod"
                    >
                        確認刪除
                    </VaButton>
                </div>
            </div>
        </VaModal>
    </VaModal>
</template>
