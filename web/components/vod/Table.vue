<script setup lang="ts">
import { computed } from "vue";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaDataTable,
    VaDivider,
    VaIcon,
    VaScrollContainer,
} from "vuestic-ui";
import { VodItem } from "@/composables/vod";
import { useAuthState } from "@/composables/authState";

const props = defineProps<{
    dateRange: { start: Date; end: Date };
    selectedTags: string[];
    strictFiltering: boolean;
    vodData: VodItem[];
}>();
const emit = defineEmits<{
    (e: "updateTag", tag: string): void;
    (e: "editVod", vod: VodItem): void;
    (e: "addVod"): void;
}>();

const authState = useAuthState();

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

const showActions = computed(() => authState.isAuthenticated);

const baseColumns = [
    {
        key: "date",
        label: "日期",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
        sortingOptions: ["desc" as const, "asc" as const, null], // because these two string is defined as constants in src.
        width: 100,
    },
    {
        key: "title",
        label: "標題",
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
        label: "時長",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
        width: 100,
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
                                @click="$emit('addVod')"
                            >
                                <VaIcon name="add" />
                            </VaButton>
                        </slot>
                    </template>
                    <template #cell(date)="{ value }">
                        <div class="text-[1rem] text-center align-middle">
                            {{ value }}
                        </div>
                    </template>
                    <template #cell(title)="{ value, row }">
                        <VaButton
                            preset="plain"
                            color="textPrimary"
                            hoverMaskColor="#5bc6a1"
                            hoverOpacity="1"
                            pressedMaskColor="info"
                            :pressedOpacity="1"
                            :href="`https://youtube.com/live/${row.rowData.link}`"
                            target="_blank"
                            rel="noreferrer noopener"
                            class="align-middle inline-block max-w-64"
                        >
                            <div class="truncate">
                                {{ value }}
                            </div>
                        </VaButton>
                    </template>
                    <template #cell(tags)="{ row }">
                        <div class="flex flex-wrap gap-2">
                            <!-- 外層 flex-wrap 讓按鈕換行 -->
                            <template
                                v-for="(tag, index) in row.rowData.tags"
                                :key="tag"
                            >
                                <VaButton
                                    size="small"
                                    preset="plain"
                                    color="textPrimary"
                                    hoverMaskColor="#5bc6a1"
                                    hoverOpacity="1"
                                    pressedMaskColor="info"
                                    :pressedOpacity="1"
                                    @click="() => emit('updateTag', tag)"
                                    class="align-middle inline-block max-w-24 truncate"
                                >
                                    <div class="text-center truncate">
                                        {{ tag }}
                                    </div>
                                </VaButton>

                                <VaDivider
                                    vertical
                                    class="inline align-middle"
                                    v-if="index !== row.rowData.tags.length - 1"
                                />
                            </template>
                        </div>
                    </template>
                    <template #cell(duration)="{ value }">
                        <div class="text-[1rem] text-center align-middle">
                            {{ value }}
                        </div>
                    </template>
                    <template v-if="showActions" #cell(actions)="{ row }">
                        <VaButton
                            preset="plain"
                            color="info"
                            aria-label="編輯直播"
                            @click="emit('editVod', row.rowData)"
                        >
                            <VaIcon name="edit" />
                        </VaButton>
                    </template>
                </VaDataTable>
            </VaScrollContainer>
        </VaCardContent>
    </VaCard>
</template>

<style scoped>
:deep(.va-data-table__td) {
    display: flex;
    align-items: center;
}
</style>
