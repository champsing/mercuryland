<script setup lang="ts">
import { computed, h } from "vue";
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "@vueuse/core";
import { NDataTable } from "naive-ui";
import { VaButton, VaDivider } from "vuestic-ui";
import { interleave, parseHMS } from "@composables/utils.ts";
import vodLinkData from "@assets/data/vod.json";

type DataType = (typeof vodLinkData)[0];

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
                VaButton,
                {
                    preset: "plain",
                    size: "small",
                    color: "textPrimary",
                    hoverMaskColor: "#5bc6a1", //same as NextPageButton and ReturnTopButton
                    hoverOpacity: 1,
                    pressedOpacity: 1,
                    href: 'https://youtube.com/live/' + row.link,
                    target: "_blank",
                    rel: "noreferrer noopener"
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
                                    VaButton,
                                    {
                                        preset: "plain",
                                        size: "small",
                                        color: "textPrimary",
                                        hoverMaskColor: "#5bc6a1", //same as NextPageButton and ReturnTopButton
                                        hoverOpacity: 1,
                                        pressedMaskColor: "info",
                                        pressedOpacity: 1,
                                        onClick: () => emit("updateTag", x),
                                    },
                                    { default: () => x }
                                )
                            ),
                            h(
                                VaDivider,
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
        <n-data-table
            :data="data"
            :columns="columns"
            flex-height
            :style="calcStyle(top)"
            class="z-0"
        />
    </use-element-bounding>
</template>
