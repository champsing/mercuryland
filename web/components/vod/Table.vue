<script setup lang="ts">
import { computed } from "vue";
import { UseElementBounding } from "@vueuse/components";
import {
    VaButton,
    VaDataTable,
    VaDivider,
    VaIcon,
    VaScrollContainer,
} from "vuestic-ui";
import { FOOTNOTE_HEIGHT, FOOTNOTE_GAP } from "@/composables/constants";
import { useWindowSize } from "@vueuse/core";

interface VodItem {
    id?: number | null;
    date: string;
    link: string;
    title: string;
    tags: string[];
    duration: string;
}

const props = defineProps<{
    dateRange: { start: Date; end: Date };
    selectedTags: string[];
    strictFiltering: boolean;
    vodData: VodItem[];
    isAuthenticated?: boolean;
}>();
const emit = defineEmits<{
    (e: "updateTag", tag: string): void;
    (e: "editVod", vod: VodItem): void;
    (e: "addVod"): void;
}>();

const data = computed(() => {
    return props.vodData
        .filter(
            (v) =>
                v.date >=
                    //prettier-ignore
                    props.dateRange.start.toISOString().slice(0, 10) &&
                v.date <=
                    //prettier-ignore
                    new Date(props.dateRange.end.getTime() + 28800000).toISOString().slice(0, 10),
        )
        .filter((v) => {
            if (props.selectedTags.length === 0) return true;

            if (props.strictFiltering === true) {
                return props.selectedTags.every((tag) => v.tags.includes(tag));
            }

            return v.tags.some((tag) => props.selectedTags.includes(tag));
        })
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
});

const showActions = computed(() => props.isAuthenticated === true);

const baseColumns = [
    {
        key: "date",
        label: "日期",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
        sortingOptions: ["desc" as const, "asc" as const, null], // because these two string is defined as constants in src.
    },
    {
        key: "title",
        label: "直播標題",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        width: 20,
    },
    {
        key: "tags",
        label: "TAG",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        width: 20,
    },
    {
        key: "duration",
        label: "直播時長",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
    },
];

const columns = computed(() => {
    const result = [...baseColumns];

    if (showActions.value) {
        result.push({
            key: "actions",
            label: "",
            thAlign: "center" as const,
            tdAlign: "center" as const,
            width: 12,
        });
    }

    return result;
});

const headerColumns = computed(() =>
    columns.value.filter((column) => column.key !== "actions"),
);
</script>

<template>
    <VaScrollContainer vertical color="#e0feb4" size="medium" class="h-full">
        <VaDataTable
            :items="data"
            :columns="columns"
            style="
                --va-data-table-hover-color: #357286;
                --va-data-table-thead-background: var(--va-background-element);
                --va-data-table-thead-border: 0;
                height: 100%;
            "
            :virtual-scroller="false"
            sticky-header
            hoverable
        >
            <template
                v-for="column in headerColumns"
                #[`header(${column.key})`]="{ label }"
                :key="column.key"
            >
                <div class="text-sm text-center">
                    {{ label }}
                </div>
            </template>
            <template v-if="showActions" #header(actions)>
                <slot name="actions">
                    <VaButton
                        preset="plain"
                        size="small"
                        color="info"
                        aria-label="新增直播"
                        @click="$emit('addVod')"
                    >
                        <VaIcon name="add" />
                    </VaButton>
                </slot>
            </template>
            <!-- for checking day of week -->
            <!-- <template #cell(date)="{ value }">
                {{ value }}  {{ new Date(value).getDay() }}
            </template> -->

            <template #cell(title)="{ value, row }">
                <!-- same as NextPageButton and ReturnTopButton -->
                <VaButton
                    preset="plain"
                    size="small"
                    color="textPrimary"
                    hoverMaskColor="#5bc6a1"
                    hoverOpacity="1"
                    pressedMaskColor="info"
                    :pressedOpacity="1"
                    :href="`https://youtube.com/live/${row.rowData.link}`"
                    target="_blank"
                    rel="noreferrer noopener"
                    class="mt-1"
                >
                    {{ value }}
                </VaButton>
            </template>
            <template #cell(tags)="{ row }">
                <!-- same as NextPageButton and ReturnTopButton -->
                <template v-for="tag in row.rowData.tags">
                    <VaButton
                        preset="plain"
                        size="small"
                        color="textPrimary"
                        hoverMaskColor="#5bc6a1"
                        hoverOpacity="1"
                        pressedMaskColor="info"
                        :pressedOpacity="1"
                        @click="() => emit('updateTag', tag)"
                        class="mt-1"
                    >
                        {{ tag }}
                    </VaButton>
                    <VaDivider
                        vertical
                        class="inline"
                        v-if="tag !== row.rowData.tags.slice().reverse()[0]"
                    />
                </template>
            </template>
            <template v-if="showActions" #cell(actions)="{ row }">
                <VaButton
                    preset="plain"
                    size="small"
                    color="info"
                    aria-label="編輯直播"
                    @click="emit('editVod', row.rowData)"
                >
                    <VaIcon name="edit" />
                </VaButton>
            </template>
        </VaDataTable>
    </VaScrollContainer>
</template>

<style scoped>
:deep(.va-data-table__thead) {
    background-color: var(--va-background-element);
}
</style>
