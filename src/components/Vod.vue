<script setup lang="ts">
import {
    NButton,
    NDatePicker,
    NGrid,
    NGi,
    NSelect,
    NTable,
    NSpace,
    NDivider,
} from "naive-ui";
import vodLinkData from "../assets/vod.json";

let filteredData = defineModel("filteredData", {
    default: vodLinkData,
});

function filterVodData(
    begTs: number,
    endTs: number,
    tag: string
): typeof vodLinkData {
    return vodLinkData
        .filter(
            (v) =>
                v.date >= new Date(begTs).toISOString().slice(0, 10) &&
                v.date <= new Date(endTs).toISOString().slice(0, 10)
        )
        .filter((v) => tag == "" || v.tags.includes(tag))
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));
}

let filterBegTs = defineModel("filterBegTs", {
    default: 1577836800000,
    set(value) {
        filteredData.value = filterVodData(
            value,
            filterEndTs.value,
            filterTag.value
        );
        return value;
    },
});
let filterEndTs = defineModel("filterEndTs", {
    default: Date.now(),
    set(value) {
        filteredData.value = filterVodData(
            filterBegTs.value,
            value,
            filterTag.value
        );
        return value;
    },
});
let filterTag = defineModel("filterTag", {
    default: "",
    set(value) {
        filteredData.value = filterVodData(
            filterBegTs.value,
            filterEndTs.value,
            value
        );
        return value;
    },
});

function tagOptions(): Array<{label: string, value: any}> {
    return [...new Set(vodLinkData.flatMap(x => x.tags))]
        .concat([""])
        .sort()
        .map(x => { return {label: x, value: x} })
}

function truncate(str: string, len: number) {
    if (str.length > len) {
        return str.substring(0, len - 2) + "...";
    } else {
        return str;
    }
}

function openLink(link: string) {
    window.open(link);
}
</script>

<template>
    <n-grid x-gap="12" :cols="4" class="main">
        <n-gi>
            <label style="font-size: 18px">起始日期:</label>
            <n-date-picker type="date" v-model:value="filterBegTs" />
        </n-gi>
        <n-gi>
            <label style="font-size: 18px">结束日期:</label>
            <n-date-picker type="date" v-model:value="filterEndTs" />
        </n-gi>
        <n-gi>
            <label style="font-size: 18px;">TAG:</label>
            <n-space vertical>
                <n-select v-model:value="filterTag" :options="tagOptions()" :consistent-menu-width="false" />
            </n-space>
        </n-gi>
    </n-grid>

    <n-divider />
    <n-grid x-gap="12" :cols="3" class="main">
        <n-gi class="detail" :span="2">
            <n-table :bordered="true" size="small" style="text-align: center">
                <thead>
                    <tr>
                        <td style="font-size: 18px">日期</td>
                        <td style="font-size: 18px">直播連結</td>
                        <td style="font-size: 18px">TAG</td>
                        <td style="font-size: 18px">直播时数</td>
                    </tr>
                </thead>

                <tbody>
                    <tr v-for="item in filteredData">
                        <td>
                            {{ item.date }}
                        </td>
                        <td>
                            <n-button
                                @click="openLink(item.link)"
                                :text="true"
                                :focusable="false"
                            >
                                {{ truncate(item.title, 30) }}
                            </n-button>
                        </td>
                        <td>
                            <template v-for="(tag, index) in item.tags">
                                <template v-if="index > 0">
                                    <n-divider vertical
                                /></template>
                                <span
                                    ><n-button
                                        @click="filterTag = tag"
                                        :text="true"
                                        :focusable="false"
                                    >
                                        {{ tag }}
                                    </n-button>
                                </span>
                            </template>
                        </td>
                        <td>
                            {{ item.hours }}
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </n-gi>
    </n-grid>
</template>

<style scoped>
.main {
    display: block;
    width: 90vw;
}

.detail {
    height: 100%;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    overflow-y: scroll;
}

.pie {
    height: 100%;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
}
</style>
