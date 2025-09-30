<script setup lang="ts">
import { computed } from "vue";
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "@vueuse/core";
import { VaButton, VaDataTable, VaDivider, VaScrollContainer } from "vuestic-ui";
import vodLinkData from "@assets/data/vod.json";

const vh = useWindowSize().height;
const props = defineProps<{
    dateRange: { start: Date; end: Date };
    selectedTags?: string[];
    strictFiltering: boolean;
}>();
const emit = defineEmits<{
    (e: "updateTag", tag: string): void;
}>();

const data = computed(() => {
    return vodLinkData
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
            if (props.strictFiltering == true)
                return (
                    props.selectedTags == null ||
                    props.selectedTags.toString() == new Array().toString() ||
                    v.tags.slice().sort().toString() ==
                        props.selectedTags.slice().sort().toString()
                );
            else
                return (
                    props.selectedTags == null ||
                    props.selectedTags.toString() == new Array().toString() ||
                    new Set(v.tags).intersection(new Set(props.selectedTags))
                        .size !== 0
                );
        })
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
});

const CENTER = "center" as const;

const columns = [
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
        width: 20
    },
    {
        key: "tags",
        label: "TAG",
        thAlign: CENTER,
        tdAlign: CENTER,
        width: 20
    },
    {
        key: "duration",
        label: "直播時長",
        thAlign: CENTER,
        tdAlign: CENTER,
        sortable: true,
    },
];

function calcStyle(top: number) {
    let pmb = 8; // parent margin
    let height = Math.max(
        vh.value * 0.5,
        vh.value - window.scrollY - top - pmb
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
                style="--va-data-table-hover-color: #357286; height: 100%;"
                virtual-scroller
                sticky-header
                hoverable
            >
                <template v-for="item in columns" #[`header(${item.key})`]="{ label }">
                    <div class="text-sm text-center">
                        {{ label }}
                    </div>
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
            </VaDataTable>
        </VaScrollContainer>
    </use-element-bounding>
</template>
