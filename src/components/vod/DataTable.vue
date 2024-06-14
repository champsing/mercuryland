<script setup lang="ts">
import { computed, h } from "vue";
import { NButton, NDataTable, NDivider } from "naive-ui";
import { openLink, interleave } from "@composables/utils.ts";
import vodLinkData from "@assets/data/vod.json";

const props = defineProps<{
    dateRange: [number, number];
    tagOption: string;
}>();
const emit = defineEmits<{
    (e: "updateTag", tag: string): void;
}>();

let data = computed(() => {
    return vodLinkData
        .filter(
            (v) =>
                v.date >= new Date(props.dateRange[0]).toISOString().slice(0, 10) &&
                v.date <= new Date(props.dateRange[1]).toISOString().slice(0, 10)
        )
        .filter(
            (v) => props.tagOption == null || v.tags.includes(props.tagOption)
        )
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
});

let columns = [
    {
        title: "日期",
        key: "date",
        className: "!text-center",
    },
    {
        title: "直播連結",
        key: "title",
        className: "!text-center",
        render(row: (typeof vodLinkData)[0]) {
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
                    h(NDivider, { vertical: true }, [])
                )
            );
        },
    },
    {
        title: "直播时数",
        key: "duration",
        className: "!text-center",
    },
];
</script>

<template>
    <n-data-table :data="data" :columns="columns" />
</template>
