<script setup lang="ts">
import { computed, onMounted, reactive, ref, Ref, watch } from "vue";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaChip,
    VaDateInput,
    VaDivider,
    VaIcon,
    VaModal,
    VaInput,
    VaSelect,
    VaSwitch,
    VaTimeInput,
} from "vuestic-ui";
import axios from "axios";
import DataTable from "./DataTable.vue";
import TimeSummary from "./TimeSummary.vue";
import TimeDetail from "./TimeDetail.vue";
import { BASE_URL, formatDate, parseDate } from "@/composables/utils";
import { useAuthState } from "@/composables/authState";
import { Info24Regular } from "@vicons/fluent";

document.title = '直播隨選 - 水星人的夢幻樂園'

interface VodItem {
    id?: number | null;
    date: string;
    link: string;
    title: string;
    tags: string[];
    duration: string;
}

const dateRange = defineModel("dateRange", {
    //1582992000 = 2020 03 01 12:00 AM Taipei ST; 8 hours = 28800 seconds
    default: {
        start: new Date(1582992000000),
        end: new Date(Date.now() + 28800000),
    },
});

const strictFiltering = ref(false);
const selectedTags: Ref<Array<string> | null> = ref(null);
const vodData = ref<VodItem[]>([]);
const tagList = computed(() =>
    [...new Set(vodData.value.flatMap((x) => x.tags))].sort()
);

const computedTime = ref(0);
const authState = useAuthState();

const showRuleDescModal = ref(false);
const showVodDescImg = ref(false);
const showAddVodModal = ref(false);
const isSavingVod = ref(false);
const addVodError = ref<string | null>(null);
const addVodSuccess = ref<string | null>(null);
const addVodForm = reactive({
    date: new Date(),
    link: "",
    title: "",
    tags: "",
});
const addVodDuration = ref<Date | null>(null);

async function loadVodData() {
    try {
        const response = await axios.get<VodItem[]>(
            `${BASE_URL}/api/video/list`
        );
        vodData.value = response.data
            .map((item) => ({
                ...item,
                tags: item.tags ?? [],
            }))
            .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));
    } catch (error) {
        console.error("Failed to load VOD data", error);
    }
}

onMounted(loadVodData);

function tagAlreadyExist(tag: string) {
    if (!selectedTags.value) return;
    selectedTags.value = selectedTags.value.filter((x) => x !== tag);
    if (selectedTags.value.toString() == new Array().toString())
        selectedTags.value = null;
}

function updateTag(tag: string) {
    if (selectedTags.value == null) {
        selectedTags.value = new Array();
        selectedTags.value.push(tag);
    } else if (selectedTags.value.includes(tag)) tagAlreadyExist(tag);
    else selectedTags.value.push(tag);
}

const handleEditVod = (_vod: VodItem) => {};

const handleAddVod = () => {
    resetAddVodForm();
    addVodError.value = null;
    addVodSuccess.value = null;
    showAddVodModal.value = true;
};

watch(showAddVodModal, (visible) => {
    if (!visible) {
        resetAddVodForm();
    }
});

function resetAddVodForm() {
    addVodForm.date = new Date();
    addVodForm.link = "";
    addVodForm.title = "";
    addVodForm.tags = "";
    addVodDuration.value = null;
    addVodError.value = null;
    addVodSuccess.value = null;
}

async function saveVod() {
    if (isSavingVod.value) return;

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

    const tags = addVodForm.tags
        .split(",")
        .map((tag) => tag.trim())
        .filter((tag) => tag.length > 0);

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
        await loadVodData();
        setTimeout(() => {
            showAddVodModal.value = false;
        }, 600);
    } catch (error) {
        console.error("Failed to insert VOD", error);
        addVodError.value = "新增失敗，請稍後再試";
    } finally {
        isSavingVod.value = false;
    }
}

function formatDuration(value: Date) {
    const pad = (num: number) => num.toString().padStart(2, "0");
    const hours = pad(value.getHours());
    const minutes = pad(value.getMinutes());
    const seconds = pad(value.getSeconds());
    return `${hours}:${minutes}:${seconds}`;
}

</script>

<template>
    <div class="m-auto w-full z-10">
        <div class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2">
            <div class="flex w-3/4 flex-row items-center gap-4">
                <div class="w-1/4 ml-12">
                    <VaDateInput
                        class="w-full"
                        v-model="dateRange"
                        :format-date="formatDate"
                        :parse-date="parseDate"
                        manual-input
                        mode="auto"
                    />
                </div>
                <div class="w-3/4">
                    <VaSelect
                        class="w-full"
                        v-model="selectedTags"
                        :options="tagList"
                        multiple
                        clearable
                        placeholder="請選擇直播的TAG"
                        dropdownIcon="va-plus"
                        @update:model-value="
                            if (selectedTags.toString() == new Array().toString())
                                selectedTags = null;
                        "
                    >
                        <!-- [SOLVED] has a problem, sometimes the first entry won't show as 
                        a chip until the second entry is selected and removed. -->
                        <!-- [SOLUTION] by setting selectTags to null every time it becomes an empty array. -->
                        <template #content>
                            <VaChip
                                v-for="chip in selectedTags"
                                color="#90dc52"
                                outline
                                size="small"
                                class="mr-1 my-1"
                                closeable
                                @update:model-value="tagAlreadyExist(chip)"
                            >
                                {{ chip }}
                            </VaChip>
                        </template>
                    </VaSelect>
                </div>
            </div>
            <div class="flex w-1/4 flex-row items-center gap-4">
                <div class="flex w-1/2 justify-center">
                    <VaSwitch
                        v-model="strictFiltering"
                        off-color="#1ccba2"
                        color="#3444a2"
                        style="--va-switch-checker-background-color: #252723"
                        false-inner-label="符合一項"
                        true-inner-label="符合全部"
                    />
                </div>
                <div class="flex w-1/2 justify-center">
                    <VaButton
                        preset="plain"
                        class="w-full"
                        color="#FFFFFF"
                        @click="showRuleDescModal = !showRuleDescModal"
                    >
                        <VaIcon size="large" class="mr-2">
                            <Info24Regular />
                        </VaIcon>
                        <div class="text-lg text-center">規則說明</div>
                    </VaButton>
                </div>
            </div>
        </div>

        <!-- 規則說明 -->
        <VaModal
            v-model="showRuleDescModal"
            title="規則說明"
            hide-default-actions
            close-button
            z-index="4"
        >
            <span class="text-3xl"> 直播時數規則說明 </span>
            <div class="text-2xl mt-2">●概述</div>
            <div class="text-bg mt-2">
                惡靈公布直播紀錄檔時，此處會同步更新計算加班台的剩餘時數，並標上當天遊玩的遊戲，供使用者藉由上方的標籤篩選功能找到自己想看的遊戲直播。
            </div>
            <div class="text-2xl mt-2">●時數計算說明</div>
            <div class="text-bg mt-2">
                在計算明細表中，會以「計劃」、「直播」等項目的時數互相加減得出剩餘時數。
                <br />
                <ol class="ml-3">
                    <li>
                        <!-- 1. -->
                        「計劃」為惡靈的常規直播時數，目前落在2小時左右，因此以2小時計算。
                    </li>
                    <li>
                        <!-- 2. -->
                        每次直播的時數沒意外的話以YouTube影片時長為準。若直播紀錄檔被和諧了，則以2小時計算。
                    </li>
                    <li>
                        <!-- 3. -->
                        有時在直播懲罰會生成加班台懲罰，此處也會以「懲罰」項目來增加剩餘的加班台時數。
                    </li>
                    <li>
                        <!-- 4. -->
                        觀眾可以用每小時真錢 10 美元或 1000 水星幣的價格購買加班台，此處也會以「課金」項目來增加剩餘的加班台時數。
                    </li>
                    <li>
                        <!-- 5. -->
                        若有其他因素導致時數加減也會以獨立項目處理。
                    </li>
                    <li>
                        <!-- 6. -->
                        伺服器時間每週三 00:00 會在計算明細表生成一項「計劃」。
                    </li>
                </ol>
                <VaButton
                    class="mt-2"
                    preset="primary"
                    border-color="info"
                    color="info"
                    gradient
                    @click="showVodDescImg = !showVodDescImg"
                >
                    查看說明圖例
                </VaButton>
            </div>

            <div class="text-2xl mt-2">●不可抗力因素</div>
            <div class="text-bg mt-2">
                若惡靈在直播過程中斷網或停電，則中間嘗試恢復的多次黑畫面直播紀錄檔將不會被採計，直到恢復1分鐘以上的穩定直播為止。
            </div>
        </VaModal>

        <VaModal
            v-model="showVodDescImg"
            hide-default-actions
            close-button
            z-index="5"
        >
            <img src="/images/vod_time.webp" alt="直播時數規則說明" />
        </VaModal>

        <VaModal
            v-model="showAddVodModal"
            hide-default-actions
            close-button
            max-width="480px"
        >
            <div class="flex flex-col gap-4 p-4">
                <div class="text-lg font-semibold text-zinc-200">新增直播紀錄</div>
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
                    v-model="addVodForm.link"
                    label="YouTube 連結代碼"
                    placeholder="例如：mCW9..."
                    color="info"
                    required
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
                    :manual-input="false"
                    view="seconds"
                />
                <VaInput
                    v-model="addVodForm.tags"
                    label="標籤 (以逗號分隔)"
                    placeholder="tag1, tag2, ..."
                    color="info"
                />
                <p v-if="addVodError" class="text-sm text-red-400">{{ addVodError }}</p>
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

        <VaDivider class="!mt-0 !mb-2" />

        <div class="flex flex-row gap-2 px-2 pb-2">
            <div class="w-3/4">
                <VaCard
                    style="--va-card-padding: 0rem"
                    class="h-full overflow-hidden rounded-xl"
                >
                    <VaCardContent class="!p-0">
                        <DataTable
                            :dateRange="dateRange"
                            :selectedTags="selectedTags"
                            :strictFiltering="strictFiltering"
                            :vodData="vodData"
                            :isAuthenticated="authState.isAuthenticated"
                            @updateTag="(tag) => updateTag(tag)"
                            @addVod="handleAddVod"
                            @editVod="handleEditVod"
                        />
                    </VaCardContent>
                </VaCard>
            </div>

            <div class="flex flex-col w-1/4">
                <TimeSummary :t="computedTime" />
                <TimeDetail
                    :dateRange="dateRange"
                    :vodData="vodData"
                    @computedTime="(time) => (computedTime = time)"
                />
            </div>
        </div>
    </div>
</template>

<style>
.n-base-suffix__arrow {
    --n-arrow-size: 20px;
}
</style>
