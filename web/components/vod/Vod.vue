<script setup lang="ts">
import { useAuthState } from "@/composables/authState";
import { formatDate, parseDate } from "@/composables/utils";
import { useBlockViewMode } from "@/composables/viewMode";
import { VodItem } from "@/composables/vod";
import api from "@composables/axios";
import { computed, onMounted, ref, Ref } from "vue";
import { VaChip, VaDateInput, VaIcon, VaSelect, VaSwitch } from "vuestic-ui";
import SortMenu from "../SortMenu.vue";
import ViewModeToggle from "../ViewModeToggle.vue";
import ViewportHeight from "../ViewportHeight.vue";
import AddVod from "./AddVod.vue";
import Blocks from "./Blocks.vue";
import Rule from "./Rule.vue";
import SetVod from "./SetVod.vue";
import Calculation from "./time/Calculation.vue";
import Summary from "./time/Summary.vue";

document.title = "直播隨選 - 水星人的夢幻樂園";

const dateRange = defineModel<{ start: Date; end: Date }>("dateRange", {
    //1519833600 = 2018 03 01 12:00 AM Taipei ST; 8 hours = 28800 seconds
    default: () => ({
        start: new Date(1519833600000),
        end: new Date(Date.now() + 28800000),
    }),
});

const strictFiltering = ref(false);
const selectedTags: Ref<string[]> = ref([]);
const vodData = ref<VodItem[]>([]);

// 卡片（3 欄）或表格（單欄單列）檢視，選擇會記在 localStorage。
const viewMode = useBlockViewMode("vod");

const authState = useAuthState();
const showActions = computed(() => authState.isAuthenticated);

const sortField = ref<string>("date");
const sortDirection = ref<"asc" | "desc">("desc");
const sortOptions = [
    { value: "date", label: "日期", defaultDirection: "desc" as const },
    { value: "title", label: "名稱", defaultDirection: "asc" as const },
];

const tagList = computed(() =>
    [...new Set(vodData.value.flatMap((x) => x.tags))].sort(),
);

const computedTime = ref(0);
const filteredCount = ref(0);
const setVodRef = ref<{ open: (vod: VodItem) => void } | null>(null);
const addVodRef = ref<{ open: () => void } | null>(null);
const calculationModalRef = ref<{ open: () => void } | null>(null);

async function loadVodData() {
    try {
        const response = await api.get<VodItem[]>(`/api/video/list`);
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
    <main class="vod-page bg-page-surface">
        <section class="w-full max-w-[1680px] mx-auto">
            <div
                class="mx-4 md:mx-20 mb-3 flex flex-wrap items-center gap-x-6 gap-y-3"
            >
                <h1 class="text-2xl font-bold text-neutral-100">直播隨選</h1>
                <p class="flex items-center gap-1 text-sm text-zinc-400">
                    <span>共</span>
                    <span
                        class="text-base font-bold leading-none text-zinc-100 tabular-nums"
                    >
                        {{ filteredCount }}
                    </span>
                    <span>筆直播</span>
                </p>
                <Rule />
                <button
                    v-if="showActions"
                    type="button"
                    class="ml-auto inline-flex shrink-0 cursor-pointer items-center gap-1.5 rounded-full border border-white/10 bg-[#5bc6a1] px-4 py-1.5 text-sm font-medium text-zinc-900 transition-colors hover:bg-[#3db088] focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                    aria-label="新增直播"
                    @click="addVodRef?.open()"
                >
                    <VaIcon name="add" size="small" aria-hidden="true" />
                    新增直播
                </button>
            </div>
            <div
                class="mx-4 md:mx-20 mb-3 flex flex-col gap-3 md:flex-row md:items-stretch"
            >
                <section
                    class="vod-filter-bar flex-1 rounded-3xl"
                    aria-label="直播隨選篩選"
                >
                    <VaDateInput
                        v-model="dateRange"
                        :format-date="formatDate"
                        :parse-date="parseDate"
                        manual-input
                        mode="auto"
                        class="vod-date"
                    />
                    <VaSelect
                        v-model="selectedTags"
                        :options="tagList"
                        multiple
                        clearable
                        searchable
                        placeholder="請選擇直播的TAG"
                        dropdownIcon="va-plus"
                        class="vod-tags"
                    >
                        <template #content>
                            <VaChip
                                v-for="chip in selectedTags"
                                :key="chip"
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
                    <div
                        class="flex w-full items-center justify-center gap-3 px-4"
                    >
                        <div class="vod-switch">
                            <VaSwitch
                                v-model="strictFiltering"
                                off-color="#1ccba2"
                                color="#3444a2"
                                style="
                                    --va-switch-checker-background-color: #252723;
                                "
                                false-inner-label="符合一項"
                                true-inner-label="符合全部"
                            />
                        </div>
                    </div>
                </section>
                <div class="hidden items-center gap-2 md:flex">
                    <ViewModeToggle v-model="viewMode" />
                    <SortMenu
                        v-model:field="sortField"
                        v-model:direction="sortDirection"
                        :options="sortOptions"
                    />
                </div>
            </div>

            <ViewportHeight>
                <section
                    class="relative flex flex-col lg:flex-row gap-2 h-full px-4 md:px-20"
                >
                    <div class="w-full h-full">
                        <Blocks
                            :dateRange="dateRange"
                            :selectedTags="selectedTags"
                            :strictFiltering="strictFiltering"
                            :vodData="vodData"
                            v-model:viewMode="viewMode"
                            v-model:sort-field="sortField"
                            v-model:sort-direction="sortDirection"
                            @updateTag="(tag: string) => updateTag(tag)"
                            @editVod="handleEditVod"
                            @countChange="filteredCount = $event"
                        />
                    </div>

                    <div
                        class="pointer-events-none absolute bottom-0 left-0 right-0 z-1 pl-4 pr-5 [&>*]:pointer-events-auto md:bottom-4 md:left-1/2 md:right-auto md:w-auto md:-translate-x-1/2 md:px-0"
                    >
                        <Summary
                            :t="computedTime"
                            @openCalculation="calculationModalRef?.open()"
                        />
                    </div>
                </section>
            </ViewportHeight>
        </section>
        <SetVod
            ref="setVodRef"
            :tag-list="tagList"
            @updated="loadVodData"
            @deleted="loadVodData"
        />
        <AddVod ref="addVodRef" :tag-list="tagList" @saved="loadVodData" />
        <Calculation
            ref="calculationModalRef"
            :dateRange="dateRange"
            :vodData="vodData"
            @computedTime="(time: number) => (computedTime = time)"
        />
    </main>
</template>

<style>
.vod-page {
    min-height: calc(100vh - 48px);
    color: #f7f7f8;
    padding: 1.25rem 1rem 1.25rem;
}

.vod-filter-bar {
    display: grid;
    grid-template-columns: minmax(13rem, 1fr) minmax(18rem, 2fr) auto;
    gap: 0.75rem;
    align-items: center;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    background: linear-gradient(
        135deg,
        rgba(28, 65, 84, 0.94) 0%,
        rgba(20, 70, 100, 0.92) 50%,
        rgba(10, 60, 90, 0.95) 100%
    );
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.3);
    padding: 0.8rem;
    backdrop-filter: blur(16px);
}

.n-base-suffix__arrow {
    --n-arrow-size: 20px;
}

@media (max-width: 1180px) {
    .vod-filter-bar {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }
}

@media (max-width: 767px) {
    .vod-page {
        padding: 3.5rem 0.75rem 1rem;
    }

    .vod-filter-bar {
        grid-template-columns: 1fr;
        padding: 0.7rem;
    }
}
</style>
