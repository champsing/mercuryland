<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaDataTable,
    VaIcon,
    VaScrollContainer,
} from "vuestic-ui";

import { useAuthState } from "@/composables/authState";
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { openLinks } from "@/composables/utils";
import { VodItem } from "@/composables/vod";
import api from "@composables/axios";
import Detail from "./detail/Detail.vue";

const props = defineProps<{
    penalties: PenItem[];
    dateRange: { start: Date; end: Date };
    state?: number | null;
    search: string;
}>();

const emit = defineEmits<{
    (e: "updateState", state: number): void;
    (e: "addPenalty"): void;
    (e: "editPenalty", penalty: PenItem): void;
}>();

const vodData = ref<VodItem[]>([]);

async function loadVodData() {
    try {
        const response = await api.get<VodItem[]>(`/api/video/list`);
        vodData.value = response.data;
        console.log("Penalty data loaded:", vodData.value);
    } catch (error) {
        console.error("Failed to load penalty data", error);
    }
}

onMounted(loadVodData);

const authState = useAuthState();
const showActions = computed(() => authState.isAuthenticated);

const YOUTUBE_LIVE = "https://youtube.com/live/";

const modal = ref(null as number | null);

const items = computed(() =>
    filterPenaltyData(
        props.penalties,
        props.dateRange,
        props.state,
        props.search,
    ).slice(),
);

const baseColumns = [
    {
        key: "date",
        label: "日期",
        thAlign: "left" as const,
        tdAlign: "left" as const,
        sortable: true,
        sortingOptions: ["desc" as const, "asc" as const, null],
        width: 132,
    },
    {
        key: "name",
        label: "內容",
        thAlign: "left" as const,
        tdAlign: "left" as const,
        width: 560,
    },
    {
        key: "state",
        label: "狀態",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        width: 128,
    },
];

const columns = computed(() => {
    const result = [...baseColumns];

    if (showActions.value) {
        result.push({
            key: "actions",
            label: "操作",
            thAlign: "center" as const,
            tdAlign: "center" as const,
            width: 76,
        });
    }

    return result;
});

function vodLinkOfDate(date: string): string[] {
    let linkIDArray = vodData.value
        .filter((x) => x.date == date)
        .map((x) => x.link);
    for (let i = 0; i < linkIDArray.length; i++)
        linkIDArray[i] = YOUTUBE_LIVE + linkIDArray[i];
    return linkIDArray;
}

function filterPenaltyData(
    data: PenItem[],
    dateRange: { start: Date; end: Date },
    state: number | null,
    search: string,
): PenItem[] {
    return data
        .filter(
            (v) =>
                v.date >= dateRange.start.toISOString().slice(0, 10) &&
                v.date <=
                    new Date(dateRange.end.getTime() + 28800000)
                        .toISOString()
                        .slice(0, 10),
        )
        .filter((v) => state == null || state == v.state)
        .filter(
            (v) =>
                search == "" ||
                v.name.toLowerCase().includes(search.toLowerCase()),
        )
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
}
</script>

<template>
    <VaCard class="penalty-table-card">
        <VaCardContent class="penalty-table-card__content">

            <VaScrollContainer
                vertical
                horizontal
                color="#8ea2ff"
                size="medium"
                class="penalty-table-scroll"
            >
                <VaDataTable
                    :items="items"
                    :columns="columns"
                    class="penalty-data-table"
                    :virtual-scroller="false"
                    sticky-header
                    hoverable
                >
                    <template
                        v-for="column in columns"
                        #[`header(${column.key})`]="{ label }"
                        :key="column.key"
                    >
                        <div class="penalty-table-header">
                            {{ label }}
                        </div>
                    </template>
                    <template v-if="showActions" #header(actions)>
                        <VaButton
                            class="penalty-add-button"
                            color="#5865f2"
                            aria-label="新增懲罰"
                            @click="$emit('addPenalty')"
                        >
                            <VaIcon name="add" />
                            <span>新增</span>
                        </VaButton>
                    </template>
                    <template #cell(date)="{ value }">
                        <VaButton
                            color="textPrimary"
                            preset="plain"
                            @click="openLinks(vodLinkOfDate(value))"
                            class="penalty-date-button"
                        >
                            <VaIcon name="play_circle" size="small" />
                            <span>{{ value }}</span>
                        </VaButton>
                    </template>
                    <template #cell(name)="{ value, row }">
                        <button
                            type="button"
                            class="penalty-name-button"
                            @click="modal = row.rowData.id"
                        >
                            {{ value }}
                        </button>
                    </template>
                    <template #cell(state)="{ value }">
                        <!-- Tailwind safelist: bg-[6d8581] bg-b91c1c bg-b45309 bg-047857 bg-4d7c0f -->
                        <VaButton
                            :class="stateColor(Number(value), 'bg')"
                            :style="{
                                backgroundColor: stateColor(
                                    Number(value),
                                    'raw',
                                ),
                            }"
                            @click="() => emit('updateState', Number(value))"
                            preset="plain"
                            color="textPrimary"
                            class="penalty-state-button"
                        >
                            {{ stateString(Number(value)) }}
                        </VaButton>
                    </template>
                    <template v-if="showActions" #cell(actions)="{ row }">
                        <VaButton
                            class="penalty-edit-button"
                            preset="plain"
                            color="#8ea2ff"
                            aria-label="編輯懲罰"
                            @click="emit('editPenalty', row.rowData)"
                        >
                            <VaIcon name="edit" size="small" />
                        </VaButton>
                    </template>
                </VaDataTable>
            </VaScrollContainer>

            <Detail v-model="modal" />
        </VaCardContent>
    </VaCard>
</template>

<style scoped>
.penalty-table-card {
    --va-card-padding: 0;
    height: min(68vh, 46rem);
    min-height: 34rem;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    background: rgba(18, 21, 27, 0.92) !important;
}

.penalty-table-card__content {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 0 !important;
}

.penalty-table-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem 1rem 0.85rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.penalty-table-toolbar h2 {
    margin: 0.1rem 0 0;
    color: #f7f7f8;
    font-size: 1.15rem;
    font-weight: 800;
    letter-spacing: 0;
}

.penalty-table-eyebrow {
    color: #8ea2ff;
    font-size: 0.72rem;
    font-weight: 800;
    letter-spacing: 0;
    text-transform: uppercase;
}

.penalty-table-actions {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    flex-shrink: 0;
}

.penalty-count {
    min-width: 4.5rem;
    justify-content: center;
}

.penalty-add-button {
    min-width: 5.5rem;
    border-radius: 8px;
    font-weight: 800;
}

.penalty-add-button :deep(.va-button__content) {
    gap: 0.35rem;
}

.penalty-table-scroll {
    flex: 1;
    min-height: 0;
}

.penalty-data-table {
    min-width: 46rem;
    height: 100%;
    --va-data-table-hover-color: rgba(88, 101, 242, 0.14);
    --va-data-table-thead-background: #191d25;
    --va-data-table-thead-border: 0;
}

.penalty-table-header {
    color: #aeb7c7;
    font-size: 0.76rem;
    font-weight: 800;
    letter-spacing: 0;
    text-transform: uppercase;
}

.penalty-date-button,
.penalty-name-button {
    display: inline-flex;
    align-items: center;
    max-width: 100%;
    color: #f7f7f8;
}

.penalty-date-button :deep(.va-button__content) {
    gap: 0.35rem;
    white-space: nowrap;
}

.penalty-name-button {
    width: min(100%, 44rem);
    border: 0;
    background: transparent;
    padding: 0.4rem 0;
    text-align: left;
    font: inherit;
    line-height: 1.35;
    cursor: pointer;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.penalty-name-button:hover {
    color: #8ea2ff;
}

.penalty-state-button {
    min-width: 5.4rem;
    border-radius: 8px;
    color: #ffffff !important;
    font-weight: 800;
    background-clip: padding-box;
}

.penalty-edit-button {
    width: 2.25rem;
    height: 2.25rem;
    border-radius: 8px;
    background: rgba(142, 162, 255, 0.1);
}

:deep(.va-data-table__table-tr) {
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

:deep(.va-data-table__table-td),
:deep(.va-data-table__table-th) {
    padding: 0.72rem 0.9rem;
}

@media (max-width: 1180px) {
    .penalty-table-card {
        height: 42rem;
    }
}

@media (max-width: 720px) {
    .penalty-table-card {
        height: 36rem;
        min-height: 30rem;
    }

    .penalty-table-toolbar {
        align-items: flex-start;
        flex-direction: column;
    }

    .penalty-table-actions {
        width: 100%;
        justify-content: space-between;
    }
}
</style>
