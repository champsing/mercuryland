<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import {
    VaButton,
    VaDataTable,
    VaScrollContainer,
    VaCard,
    VaCardContent,
} from "vuestic-ui";
import vodData from "@assets/data/vod.json";
import PenaltyModal from "./PenaltyModal.vue";
import { openLinks, PenItem } from "@/composables/utils";
import { stateString } from "@/composables/penalty";

const props = defineProps<{
    penalties: PenItem[];
    dateRange: { start: Date; end: Date };
    state?: number | null;
    search: string;
}>();

const emit = defineEmits<{
    (e: "updateState", state: number): void;
}>();

interface PenaltyDataEntry {
    id: number;
    date: string;
    name: string;
    status: string;
    description?: {
        type: string;
        text?: string;
        uri_link?: string;
        uri_imgs?: string[];
        uri_num?: number;
    }[];
    reapply?: { date: string; status: string }[];
    steamID?: number;
    progress?: number;
}

const YOUTUBE_LIVE = "https://youtube.com/live/";

const showPEM = ref(false); // showPenaltyEntryModal

const PEMContent: Ref<PenaltyDataEntry> = defineModel("PEMContent", {
    default: null,
    set(value) {
        showPEM.value = !showPEM.value;
        return value;
    },
}); // penaltyEntryModalContent

// [DONE] 修正成跟 DataTable.vue 裡面一樣使用 columns {row} 形式
const items = computed(() =>
    filterPenaltyData(
        props.penalties,
        props.dateRange,
        props.state,
        props.search,
    ).slice(),
);

const columns = [
    {
        key: "date",
        label: "日期",
        tdAlign: "center" as const,
        thAlign: "center" as const,
        sortable: true,
        sortingOptions: ["desc" as const, "asc" as const, null],
    },
    {
        key: "name",
        label: "內容",
        tdAlign: "center" as const,
        thAlign: "center" as const,
    },
    {
        key: "state",
        label: "狀態",
        tdAlign: "center" as const,
        thAlign: "center" as const,
    },
];

function vodLinkOfDate(date: string): string[] {
    let linkIDArray = vodData.filter((x) => x.date == date).map((x) => x.link);
    for (let i = 0; i < linkIDArray.length; i++)
        linkIDArray[i] = YOUTUBE_LIVE + linkIDArray[i];
    return linkIDArray;
}

function filterPenaltyData(
    data: PenItem[],
    dateRange: { start: Date; end: Date },
    status: number | null,
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
        .filter((v) => status == null || status == v.state)
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
        style="--va-card-padding: 0rem"
        class="h-full overflow-hidden rounded-xl"
    >
        <VaCardContent class="!p-0 h-full">
            <VaScrollContainer
                vertical
                color="#e0feb4"
                size="medium"
                class="h-full"
            >
                <VaDataTable
                    :items="items"
                    :columns="columns"
                    style="
                        --va-data-table-hover-color: #357286;
                        --va-data-table-thead-background: var(
                            --va-background-element
                        );
                        --va-data-table-thead-border: 0;
                        height: 100%;
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
                        <div class="text-sm text-center">
                            {{ label }}
                        </div>
                    </template>
                    <template #cell(date)="{ value, row }">
                        <div v-if="row.rowData.state == 0">----</div>
                        <div v-else>
                            <VaButton
                                color="textPrimary"
                                preset="plain"
                                @click="openLinks(vodLinkOfDate(value))"
                                class="align-middle"
                            >
                                {{ value }}
                            </VaButton>
                        </div>
                    </template>
                    <template #cell(name)="{ value, row }">
                        <VaButton
                            @click="
                                PEMContent = {
                                    id: row.rowData.id,
                                    date: row.rowData.date,
                                    name: row.rowData.name,
                                    status: stateString(row.rowData.state),
                                    description: [],
                                    reapply: [],
                                    steamID: undefined,
                                    progress: undefined,
                                }
                            "
                            preset="plain"
                            color="textPrimary"
                            class="align-middle inline-block max-w-96"
                        >
                            <div class="truncate">{{ value }}</div>
                        </VaButton>
                    </template>
                    <template #cell(state)="{ value }">
                        <!-- !bg-[#6d8581] !bg-[#b91c1c] !bg-[#4d7c0f] !bg-[#047857] !bg-[#b45309] -->
                        <!-- TAILWIND CSS: DO NOT REMOVE ABOVE COMMENT -->
                        <VaButton
                            :class="`bg-penalty-state-${Number(value)} text-white font-bold rounded-lg px-2`"
                            @click="
                                () => emit('updateState', Number(value))
                            "
                            preset="plain"
                            color="textPrimary"
                            :style="{
                                backgroundClip: 'padding-box',
                            }"
                            class="align-middle"
                        >
                            {{ stateString(Number(value)) }}
                        </VaButton>
                    </template>
                </VaDataTable>
            </VaScrollContainer>

            <PenaltyModal
                v-model="showPEM"
                :penalty="PEMContent"
                @changePenalty="PEMContent = $event"
            />
        </VaCardContent>
    </VaCard>
</template>

<style scoped>
:deep(.va-data-table__thead) {
    background-color: var(--va-background-element);
}
</style>
