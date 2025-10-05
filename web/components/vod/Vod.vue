<script setup lang="ts">
import { computed, onMounted, ref, Ref } from "vue";
import {
    VaCard,
    VaCardContent,
    VaChip,
    VaDateInput,
    VaDivider,
    VaSelect,
    VaSwitch,
} from "vuestic-ui";
import axios from "axios";
import DataTable from "./Table.vue";
import Summary from "./time/Summary.vue";
import Calculation from "./time/Calculation.vue";
import AddVod from "./AddVod.vue";
import SetVod from "./SetVod.vue";
import Rule from "./Rule.vue";
import { BASE_URL, formatDate, parseDate } from "@/composables/utils";
import { useAuthState } from "@/composables/authState";
import ViewportHeight from "../ViewportHeight.vue";

document.title = "直播隨選 - 水星人的夢幻樂園";

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
const selectedTags: Ref<string[]> = ref([]);
const vodData = ref<VodItem[]>([]);
const tagList = computed(() =>
    [...new Set(vodData.value.flatMap((x) => x.tags))].sort(),
);

const computedTime = ref(0);
const authState = useAuthState();
const setVodRef = ref<{ open: (vod: VodItem) => void } | null>(null);
const addVodRef = ref<{ open: () => void } | null>(null);

async function loadVodData() {
    try {
        const response = await axios.get<VodItem[]>(
            `${BASE_URL}/api/video/list`,
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

function updateTag(tag: string) {
    if (selectedTags.value.includes(tag)) {
        selectedTags.value = selectedTags.value.filter((x) => x !== tag);
    } else {
        selectedTags.value = [...selectedTags.value, tag];
    }
}

const handleEditVod = (vod: VodItem) => {
    setVodRef.value?.open(vod);
};
</script>

<template>
    <div>
        <div
            class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2"
        >
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
                        searchable
                        placeholder="請選擇直播的TAG"
                        dropdownIcon="va-plus"
                    >
                        <template #content>
                            <VaChip
                                v-for="chip in selectedTags"
                                color="#90dc52"
                                outline
                                size="small"
                                class="mr-1 my-1"
                                closeable
                                @update:model-value="updateTag(chip)"
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
                    <Rule />
                </div>
            </div>
        </div>

        <VaDivider class="!mt-0 !mb-2" />

        <ViewportHeight>
            <div class="flex flex-row gap-2 px-2 h-full">
                <div class="w-3/4 h-full">
                    <VaCard
                        style="--va-card-padding: 0rem"
                        class="h-full overflow-hidden rounded-xl"
                    >
                        <VaCardContent class="!p-0 h-full">
                            <DataTable
                                :dateRange="dateRange"
                                :selectedTags="selectedTags"
                                :strictFiltering="strictFiltering"
                                :vodData="vodData"
                                :isAuthenticated="authState.isAuthenticated"
                                @updateTag="(tag) => updateTag(tag)"
                                @editVod="handleEditVod"
                                @addVod="addVodRef?.open()"
                            />
                        </VaCardContent>
                    </VaCard>
                </div>

                <div class="flex flex-col w-1/4">
                    <Summary :t="computedTime" />
                    <Calculation
                        :dateRange="dateRange"
                        :vodData="vodData"
                        @computedTime="(time) => (computedTime = time)"
                    />
                </div>
            </div>
        </ViewportHeight>
        <SetVod
            ref="setVodRef"
            :tag-list="tagList"
            :is-authenticated="authState.isAuthenticated"
            @updated="loadVodData"
            @deleted="loadVodData"
        />
        <AddVod
            ref="addVodRef"
            :tag-list="tagList"
            :is-authenticated="authState.isAuthenticated"
            @saved="loadVodData"
        />
    </div>
</template>

<style>
.n-base-suffix__arrow {
    --n-arrow-size: 20px;
}
</style>
