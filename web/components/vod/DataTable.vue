<script setup lang="ts">
import { computed } from "vue";
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "@vueuse/core";
import {
    VaButton,
    VaDataTable,
    VaDivider,
    VaIcon,
    VaScrollContainer,
} from "vuestic-ui";

interface VodItem {
    id?: number | null;
    date: string;
    link: string;
    title: string;
    tags: string[];
    duration: string;
}

const vh = useWindowSize().height;
const props = defineProps<{
    dateRange: { start: Date; end: Date };
    selectedTags?: string[];
    strictFiltering: boolean;
    vodData: VodItem[];
    isAuthenticated?: boolean;
}>();
const emit = defineEmits<{
    (e: "updateTag", tag: string): void;
    (e: "editVod", vod: VodItem): void;
}>();

const data = computed(() => {
    let selectedTags = new Set(props.selectedTags);

    return props.vodData
        .filter(
            (v) =>
                v.date >=
                    //prettier-ignore
                    props.dateRange.start.toISOString().slice(0, 10) &&
                v.date <=
                    //prettier-ignore
                    new Date(props.dateRange.end.getTime() + 28800000).toISOString().slice(0, 10)
        )
        .filter((v) => {
            if (selectedTags.size === 0) return true;

            if (props.strictFiltering === true) {
                const vTags = v.tags.slice().sort();
                return (
                    vTags.length === selectedTags.size &&
                    vTags.every((tag) => selectedTags.has(tag))
                );
            }

            return v.tags.some((tag) => selectedTags.has(tag));
        })
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
});

const CENTER = "center" as const;

const showActions = computed(() => props.isAuthenticated === true);

const baseColumns = [
    {
        key: "date",
        label: "日期",
        thAlign: CENTER,
        tdAlign: CENTER,
        sortable: true,
        sortingOptions: ["desc" as const, "asc" as const, null], // because these two string is defined as constants in src.
    },
    {
        key: "title",
        label: "直播標題",
        thAlign: CENTER,
        tdAlign: CENTER,
        width: 20,
    },
    {
        key: "tags",
        label: "TAG",
        thAlign: CENTER,
        tdAlign: CENTER,
        width: 20,
    },
    {
        key: "duration",
        label: "直播時長",
        thAlign: CENTER,
        tdAlign: CENTER,
        sortable: true,
    },
];

const columns = computed(() => {
    const result = [...baseColumns];

    if (showActions.value) {
        result.push({
            key: "actions",
            label: "",
            thAlign: CENTER,
            tdAlign: CENTER,
            width: 12,
        });
    }

    return result;
});

const headerColumns = computed(() =>
    columns.value.filter((column) => column.key !== "actions")
);

function calcStyle(top: number) {
    let parentMarginBottom = 8;
    let parentPaddingBottom = 8;
    let footnoteHeight = 48;

    let delta = parentMarginBottom + footnoteHeight + parentPaddingBottom;
    let height = Math.max(
        vh.value * 0.5,
        vh.value - window.scrollY - top - delta
    );
    return {
        height: "" + height + "px",
    };
}
</script>

<template>
    <use-element-bounding v-slot="{ top }" class="mb-2">
        <VaScrollContainer
            vertical
            color="#e0feb4"
            size="medium"
            class="h-full"
            :style="calcStyle(top)"
        >
            <VaDataTable
                :items="data"
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
    </use-element-bounding>
</template>

<style scoped>
:deep(.va-data-table__thead) {
    background-color: var(--va-background-element);
}
</style>