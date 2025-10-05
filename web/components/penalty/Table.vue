<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import {
    VaButton,
    VaDataTable,
    VaScrollContainer,
    VaCard,
    VaCardContent,
} from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import vodData from "@assets/data/vod.json";
import PenaltyModal from "./PenaltyModal.vue";
import { openLinks, truncateString } from "@/composables/utils";
import { statusOf } from "@/composables/penalty";

const props = defineProps<{
    dateRange: { start: Date; end: Date };
    status?: string;
    search: string;
}>();

const emit = defineEmits<{
    (e: "updateStatus", status: string): void;
}>();

class PenaltyDataEntry {
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
    filterPenaltyData(props.dateRange, props.status, props.search).slice(),
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
        key: "status",
        label: "完成狀態",
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
    dateRange: { start: Date; end: Date },
    status: string,
    search: string,
): typeof penaltyData {
    return penaltyData
        .filter(
            (v) =>
                v.date >= dateRange.start.toISOString().slice(0, 10) &&
                v.date <=
                    new Date(dateRange.end.getTime() + 28800000)
                        .toISOString()
                        .slice(0, 10),
        )
        .filter((v) => status == null || status == v.status)
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

                    <!-- check day of week:  {{ new Date(value).getDay() }} -->
                    <template #cell(date)="{ value, row }">
                        <div class="text-center">
                            <div v-if="row.rowData.status == '未生效'">
                                ----
                            </div>
                            <div v-else>
                                <VaButton
                                    color="textPrimary"
                                    preset="plain"
                                    class=""
                                    @click="openLinks(vodLinkOfDate(value))"
                                >
                                    {{ value }}
                                </VaButton>
                            </div>
                        </div>
                    </template>

                    <template #cell(name)="{ value, row }">
                        <div class="text-center">
                            <VaButton
                                @click="
                                    PEMContent = row.rowData as PenaltyDataEntry
                                "
                                preset="plain"
                                color="textPrimary"
                            >
                                {{ truncateString(value, 25) }}
                            </VaButton>
                        </div>
                    </template>
                    <template #cell(status)="{ value }">
                        <!-- !bg-[#6d8581] !bg-[#b91c1c] !bg-[#4d7c0f] !bg-[#047857] !bg-[#b45309] -->
                        <!-- TAILWIND CSS: DO NOT REMOVE ABOVE COMMENT -->
                        <VaChip
                            :class="`!bg-[${statusOf(value).color}] text-white font-bold rounded-lg px-2`"
                            @click="() => emit('updateStatus', value)"
                            size="medium"
                        >
                            {{ value }}
                        </VaChip>
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
