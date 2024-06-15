<script setup lang="ts">
import { computed, h } from "vue";
import { UseElementBounding } from "@vueuse/components";
import { NButton, NDataTable, NDivider } from "naive-ui";
import { openLink, interleave, parseHMS } from "@composables/utils.ts";
import vodLinkData from "@assets/data/vod.json";

type DataType = (typeof vodLinkData)[0];

const props = defineProps<{
    dateRange: [number, number];
    tagOption?: string;
}>();
const emit = defineEmits<{
    (e: "updateTag", tag: string): void;
}>();

const data = computed(() => {
    return vodLinkData
        .filter(
            (v) =>
                v.date >=
                    new Date(props.dateRange[0]).toISOString().slice(0, 10) &&
                v.date <=
                    new Date(props.dateRange[1]).toISOString().slice(0, 10)
        )
        .filter(
            (v) => props.tagOption == null || v.tags.includes(props.tagOption)
        )
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
});
const columns = [
    {
        title: "日期",
        key: "date",
        className: "!text-center",
        sorter: (row1: DataType, row2: DataType) =>
            new Date(row1.date).getTime() - new Date(row2.date).getTime(),
    },
    {
        title: "直播連結",
        key: "title",
        className: "!text-center",
        render(row: DataType) {
            return h(
                NButton,
                {
                    text: true,
                    focusable: false,
                    onClick: () => openLink(row.link),
                },
                { default: () => row.title }
            );
        },
    },
    {
        title: "TAG",
        key: "tags",
        className: "!text-center",
        render(row: (typeof vodLinkData)[0]) {
            return h(
                "div",
                {},
                {
                    default: () =>
                        interleave(
                            row.tags.map((x) =>
                                h(
                                    NButton,
                                    {
                                        text: true,
                                        focusable: false,
                                        onClick: () => emit("updateTag", x),
                                    },
                                    { default: () => x }
                                )
                            ),
                            h(
                                NDivider,
                                { vertical: true },
                                { default: () => null }
                            )
                        ),
                }
            );
        },
    },
    {
        title: "直播时数",
        key: "duration",
        className: "!text-center",
        sorter: (row1: DataType, row2: DataType) =>
            parseHMS(row1.duration) - parseHMS(row2.duration),
    },
];

function calcStyle(top: number) {
    let mb = 8;
    let vh = document.documentElement.clientHeight;
    let height = Math.max(0, vh - window.scrollY - top - mb);
    return {
        height: "" + height + "px",
        "margin-bottom": mb,
    };
}
</script>

<template>
    <UseElementBounding v-slot="{ top }">
        <n-data-table
            :data="data"
            :columns="columns"
            flex-height
            :style="calcStyle(top)"
        />
    </UseElementBounding>
</template>
