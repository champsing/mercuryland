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
        width: "132px",
    },
    {
        key: "name",
        label: "內容",
        thAlign: "left" as const,
        tdAlign: "left" as const,
        // 移除固定的 width: 560，讓此欄位彈性填滿剩餘寬度
    },
    {
        key: "state",
        label: "狀態",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        width: "128px",
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
            width: "76px",
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
    <VaCard
        class="h-[36rem] min-h-[30rem] md:h-[42rem] lg:h-[min(68vh,46rem)] lg:min-h-[34rem] overflow-hidden border border-white/10 rounded-lg !bg-[#12151b]/90"
        style="--va-card-padding: 0"
    >
        <VaCardContent class="flex flex-col h-full !p-0">
            <VaScrollContainer
                vertical
                horizontal
                color="#8ea2ff"
                size="medium"
                class="flex-1 min-h-0"
            >
                <VaDataTable
                    :items="items"
                    :columns="columns"
                    class="w-full h-full [&_.va-data-table__table-tr]:border-b [&_.va-data-table__table-tr]:border-white/5 [&_.va-data-table__table-td]:p-[0.72rem_0.9rem] [&_.va-data-table__table-th]:p-[0.72rem_0.9rem]"
                    style="
                        --va-data-table-hover-color: rgba(88, 101, 242, 0.14);
                        --va-data-table-thead-background: #191d25;
                        --va-data-table-thead-border: 0;
                    "
                    :virtual-scroller="false"
                    sticky-header
                    hoverable
                >
                    <template
                        v-for="column in columns"
                        #[`header(${column.key})`]="{ label }"
                        :key="column.key"
                    >
                        <div
                            class="text-[#aeb7c7] text-[0.76rem] font-extrabold tracking-normal uppercase"
                        >
                            {{ label }}
                        </div>
                    </template>

                    <template v-if="showActions" #header(actions)>
                        <VaButton
                            class="rounded-lg font-extrabold [&_.va-button__content]:gap-[0.35rem]"
                            color="#5865f2"
                            aria-label="新增懲罰"
                            @click="$emit('addPenalty')"
                        >
                            <VaIcon name="add" />
                        </VaButton>
                    </template>

                    <template #cell(date)="{ value }">
                        <VaButton
                            color="textPrimary"
                            preset="plain"
                            @click="openLinks(vodLinkOfDate(value))"
                            class="inline-flex items-center max-w-full text-[#f7f7f8] [&_.va-button__content]:gap-[0.35rem] [&_.va-button__content]:whitespace-nowrap"
                        >
                            <VaIcon name="play_circle" size="small" />
                            <span>{{ value }}</span>
                        </VaButton>
                    </template>

                    <template #cell(name)="{ value, row }">
                        <button
                            type="button"
                            class="inline-flex items-center w-full max-w-[44rem] border-0 bg-transparent py-[0.4rem] px-0 text-left leading-[1.35] cursor-pointer overflow-hidden text-ellipsis whitespace-nowrap text-[#f7f7f8] hover:text-[#8ea2ff]"
                            @click="modal = row.rowData.id"
                        >
                            {{ value }}
                        </button>
                    </template>

                    <template #cell(state)="{ value }">
                        <VaButton
                            :class="stateColor(Number(value), 'bg')"
                            @click="() => emit('updateState', Number(value))"
                            preset="plain"
                            color="textPrimary"
                            :style="{
                                backgroundClip: 'padding-box',
                            }"
                            class="align-middle text-white font-bold rounded-lg px-2"
                        >
                            {{ stateString(Number(value)) }}
                        </VaButton>
                    </template>

                    <template v-if="showActions" #cell(actions)="{ row }">
                        <VaButton
                            class="w-9 h-9 rounded-lg bg-[#8ea2ff]/10"
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
