<script setup lang="ts">
import { computed, onMounted, ref, Ref } from "vue";
import { VaChip, VaDateInput, VaSelect, VaSwitch } from "vuestic-ui";
import api from "@composables/axios";
import Table from "./Table.vue";
import Summary from "./time/Summary.vue";
import Calculation from "./time/Calculation.vue";
import AddVod from "./AddVod.vue";
import SetVod from "./SetVod.vue";
import Rule from "./Rule.vue";
import { formatDate, parseDate } from "@/composables/utils";
import { VodItem } from "@/composables/vod";
import { useAuthState } from "@/composables/authState";
import ViewportHeight from "../ViewportHeight.vue";

document.title = "直播隨選 - 水星人的夢幻樂園";

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
    <main class="vod-page">
        <section class="vod-shell">
            <section class="vod-filter-bar" aria-label="直播隨選篩選">
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
                <div class="vod-switch">
                    <VaSwitch
                        v-model="strictFiltering"
                        off-color="#1ccba2"
                        color="#3444a2"
                        style="--va-switch-checker-background-color: #252723"
                        false-inner-label="符合一項"
                        true-inner-label="符合全部"
                    />
                </div>
                <div class="vod-rule">
                    <Rule />
                </div>
            </section>

            <ViewportHeight>
                <section class="vod-main">
                    <div class="vod-table">
                        <Table
                            :dateRange="dateRange"
                            :selectedTags="selectedTags"
                            :strictFiltering="strictFiltering"
                            :vodData="vodData"
                            :isAuthenticated="authState.isAuthenticated"
                            @updateTag="(tag: string) => updateTag(tag)"
                            @editVod="handleEditVod"
                            @addVod="addVodRef?.open()"
                        />
                    </div>

                    <div class="vod-sidebar">
                        <div class="vod-summary">
                            <Summary :t="computedTime" />
                        </div>
                        <div class="vod-calculation">
                            <Calculation
                                :dateRange="dateRange"
                                :vodData="vodData"
                                @computedTime="(time: number) => (computedTime = time)"
                            />
                        </div>
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
    </main>
</template>

<style>
.vod-page {
    min-height: calc(100vh - 48px);
    background:
        radial-gradient(
            circle at 15% 0%,
            rgba(88, 166, 255, 0.2),
            transparent 32rem
        ),
        linear-gradient(135deg, #0f1117 0%, #16191f 48%, #101318 100%);
    color: #f7f7f8;
    padding: 2.5rem 1rem 1.25rem;
}

.vod-shell {
    width: min(1680px, 100%);
    margin: 0 auto;
}

.vod-filter-bar {
    display: grid;
    grid-template-columns:
        minmax(13rem, 0.9fr) minmax(18rem, 2fr) minmax(10rem, 0.7fr)
        auto;
    gap: 0.75rem;
    align-items: center;
    margin-bottom: 1rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    background: rgba(22, 25, 31, 0.86);
    padding: 0.8rem;
    backdrop-filter: blur(16px);
}

.vod-switch {
    display: flex;
    justify-content: center;
}

.vod-rule {
    display: flex;
    justify-content: flex-end;
}

.vod-main {
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    height: 100%;
}

.vod-table {
    width: 75%;
    height: 100%;
}

.vod-sidebar {
    display: flex;
    flex-direction: column;
    width: 25%;
    gap: 0.5rem;
    height: 100%;
}

.vod-summary {
    height: 25%;
}

.vod-calculation {
    height: 75%;
}

.n-base-suffix__arrow {
    --n-arrow-size: 20px;
}

@media (max-width: 1180px) {
    .vod-filter-bar {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .vod-rule {
        justify-content: stretch;
    }
}

@media (max-width: 900px) {
    .vod-main {
        flex-direction: column;
    }

    .vod-table {
        width: 100%;
        height: 60%;
    }

    .vod-sidebar {
        width: 100%;
        height: 40%;
    }
}

@media (max-width: 720px) {
    .vod-page {
        padding: 4rem 0.75rem 1rem;
    }

    .vod-filter-bar {
        grid-template-columns: 1fr;
        padding: 0.7rem;
    }
}
</style>
