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
    VaIcon,
} from "vuestic-ui";
import axios from "axios";
import { BASE_URL, formatDate, parseDate } from "@/composables/utils";
import { Robot as FaRobot } from "@vicons/fa";

const props = defineProps<{
    tagList: string[];
    isAuthenticated?: boolean;
}>();

const emit = defineEmits<{
    (event: "saved"): void;
}>();

const showAddVodModal = ref(false);
const isSavingVod = ref(false);
const addVodError = ref<string | null>(null);
const addVodSuccess = ref<string | null>(null);
const addVodForm = reactive({
    date: new Date(),
    link: "",
    title: "",
});
const addVodDuration = ref<Date | null>(null);
const addVodTags = ref<string[]>([]);
const addVodCustomTags = ref<string[]>([]);

const addVodTagOptions = computed(() => {
    const uniqueTags = new Set(props.tagList ?? []);
    addVodCustomTags.value.forEach((tag) => {
        if (tag) {
            uniqueTags.add(tag);
        }
    });
    addVodTags.value.forEach((tag) => {
        if (tag) {
            uniqueTags.add(tag);
        }
    });
    return Array.from(uniqueTags).sort();
});

watch(showAddVodModal, (visible) => {
    if (!visible) {
        resetAddVodForm();
    }
});

function resetAddVodForm() {
    addVodForm.date = new Date();
    addVodForm.link = "";
    addVodForm.title = "";
    addVodTags.value = [];
    addVodCustomTags.value = [];
    addVodDuration.value = null;
    addVodError.value = null;
    addVodSuccess.value = null;
}

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

const removeAddVodTag = (tag: string) => {
    addVodTags.value = addVodTags.value.filter((value) => value !== tag);
};

const handleCreateVodTag = (tag: string) => {
    const trimmed = tag.trim();
    if (!trimmed) {
        return;
    }

    if (!addVodTags.value.includes(trimmed)) {
        addVodTags.value = [...addVodTags.value, trimmed];
    }

    if (
        !addVodCustomTags.value.includes(trimmed) &&
        !props.tagList.includes(trimmed)
    ) {
        addVodCustomTags.value = [...addVodCustomTags.value, trimmed];
    }
};

const handleLinkRobotClick = async () => {
    if (!addVodForm.link.trim()) {
        addVodError.value = "請先輸入 YouTube 連結代碼";
        return;
    }

    try {
        addVodError.value = null;
        const response = await axios.post(`${BASE_URL}/api/video/metadata`, {
            url: addVodForm.link.trim(),
        });

        const metadata = response.data;

        // Auto-fill title if available
        if (metadata.title) {
            addVodForm.title = metadata.title;
        }

        // Auto-fill date if available
        if (metadata.date) {
            // Parse the ISO date string to Date object
            const uploadDate = new Date(metadata.date);
            if (!isNaN(uploadDate.getTime())) {
                addVodForm.date = uploadDate;
            }
        }

        // Auto-fill duration if available
        if (metadata.duration) {
            // Parse duration string (e.g., "3:34" or "1:23:45") to Date object for time input
            const durationParts = metadata.duration.split(":").map(Number);
            const durationDate = new Date();
            durationDate.setHours(0, 0, 0, 0); // Reset to start of day

            if (durationParts.length === 2) {
                // MM:SS format
                durationDate.setMinutes(durationParts[0], durationParts[1]);
            } else if (durationParts.length === 3) {
                // HH:MM:SS format
                durationDate.setHours(
                    durationParts[0],
                    durationParts[1],
                    durationParts[2],
                );
            }

            addVodDuration.value = durationDate;
        }

        addVodSuccess.value = "成功獲取影片資訊";
        setTimeout(() => {
            addVodSuccess.value = null;
        }, 3000);
    } catch (error) {
        console.error("Failed to fetch video metadata", error);
        addVodError.value = "無法獲取影片資訊，請檢查連結代碼是否正確";
    }
};

const openAddVodModal = () => {
    if (!props.isAuthenticated) {
        return;
    }
    showAddVodModal.value = true;
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
        addVodError.value = "請先登入管理員帳號";
        return;
    }

    if (
        !addVodForm.date ||
        !addVodForm.link.trim() ||
        !addVodForm.title.trim() ||
        !addVodDuration.value
    ) {
        addVodError.value = "請填寫所有必填欄位";
        return;
    }

    const tags = Array.from(
        new Set(
            addVodTags.value
                .map((tag) => tag.trim())
                .filter((tag) => tag.length > 0),
        ),
    );

    const duration = formatDuration(addVodDuration.value);

    const payload = {
        token,
        date: formatDate(addVodForm.date),
        link: addVodForm.link.trim(),
        title: addVodForm.title.trim(),
        duration,
        tags,
    };

    try {
        isSavingVod.value = true;
        addVodError.value = null;
        await axios.post(`${BASE_URL}/api/video/insert`, payload);
        addVodSuccess.value = "新增成功";
        emit("saved");
        setTimeout(() => {
            showAddVodModal.value = false;
        }, 600);
    } catch (error) {
        console.error("Failed to insert VOD", error);
        addVodError.value = "新增失敗，請稍後再試";
    } finally {
        isSavingVod.value = false;
    }
};

defineExpose({ open: openAddVodModal });
</script>

<template>
    <VaModal
        v-model="showAddVodModal"
        hide-default-actions
        close-button
        max-width="480px"
    >
        <div class="flex flex-col gap-4 p-4">
            <div class="text-lg font-semibold text-zinc-200">新增直播紀錄</div>
            <div class="flex items-end gap-2">
                <VaInput
                    v-model="addVodForm.link"
                    label="YouTube 連結代碼"
                    placeholder="例如：mCW9..."
                    color="info"
                    class="flex-1"
                    required
                />
                <VaButton
                    preset="secondary"
                    color="info"
                    aria-label="使用機器人填入連結"
                    @click="handleLinkRobotClick"
                >
                    <VaIcon>
                        <FaRobot />
                    </VaIcon>
                </VaButton>
            </div>
            <VaDateInput
                v-model="addVodForm.date"
                label="日期"
                color="info"
                :format-date="formatDate"
                :parse-date="parseDate"
                manual-input
                mode="auto"
            />
            <VaInput
                v-model="addVodForm.title"
                label="直播標題"
                color="info"
                required
            />
            <VaTimeInput
                v-model="addVodDuration"
                label="直播時長"
                color="info"
                :ampm="false"
                :hide-period-switch="true"
                :manual-input="true"
                view="seconds"
            />
            <VaSelect
                v-model="addVodTags"
                label="標籤"
                color="info"
                :options="addVodTagOptions"
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
                        v-for="chip in addVodTags"
                        :key="chip"
                        color="#90dc52"
                        outline
                        size="small"
                        class="mr-1 my-1"
                        closeable
                        @update:model-value="removeAddVodTag(chip)"
                    >
                        {{ chip }}
                    </VaChip>
                </template>
            </VaSelect>
            <p v-if="addVodError" class="text-sm text-red-400">
                {{ addVodError }}
            </p>
            <p v-if="addVodSuccess" class="text-sm text-emerald-400">
                {{ addVodSuccess }}
            </p>
            <div class="flex justify-end gap-2">
                <VaButton
                    preset="secondary"
                    :disabled="isSavingVod"
                    @click="showAddVodModal = false"
                >
                    取消
                </VaButton>
                <VaButton color="info" :loading="isSavingVod" @click="saveVod">
                    儲存
                </VaButton>
            </div>
        </div>
    </VaModal>
</template>
