<script setup lang="ts">
import { ref } from "vue";
import {
    NButton,
    NDatePicker,
    NGrid,
    NGi,
    NSelect,
    NTable,
    NSpace,
    NDivider,
    NCard,
} from "naive-ui";
import {
    truncateText,
    openLink,
    parseHMS,
    formatHMS,
} from "../composables/utils.ts";
import vodLinkData from "../assets/vod.json";
import vodSchedule from "../assets/schedule.json";

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
let tagOptions = ref(
    [...new Set(vodLinkData.flatMap((x) => x.tags))]
        .concat([""])
        .sort()
        .map((x) => {
            return { label: x, value: x };
        })
);
let filteredData = defineModel("filteredData", {
    default: filterVodData(0, Date.now(), ""),
});

let vodTimeData = computeVodTime();
</script>

<script lang="ts">
class VodTimeEntry {
    date: string;
    offset: number;
    previous: number;
    reason: string;
    divider: boolean;
}

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
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
}

function computeVodTime(): VodTimeEntry[] {
    let re: VodTimeEntry[] = [];

    let sch = vodSchedule.schedule;
    let ove = vodSchedule.override;

    let s = 0;
    let o = 0;
    let v = 0;

    let now = new Date(Date.now());
    let date: Date = new Date(
        Math.min(
            (sch.length == 0 ? now : new Date(sch[0].date)).getTime(),
            (ove.length == 0 ? now : new Date(ove[0].date)).getTime()
        )
    );
    let vod = vodLinkData.filter(
        (x) => x.date > date.toISOString().slice(0, 10)
    );

    let previous = parseHMS(vodSchedule.initial);

    while (date < now) {
        if (
            s + 1 < sch.length &&
            sch[s + 1].date >= date.toISOString().slice(0, 10)
        ) {
            s = s + 1;
        }
        re.push({
            date: date.toISOString().slice(0, 10),
            offset: parseHMS(sch[s].duration),
            previous: previous,
            reason: "计划",
            divider: true,
        });
        previous = previous + re[re.length - 1].offset;

        date.setDate(date.getDate() + 7);

        // while (o + 1 < ove.length && )

        while (v < vod.length && new Date(vod[v].date) < date) {
            re.push({
                date: vod[v].date,
                offset: -parseHMS(vod[v].duration),
                previous: previous,
                reason: "直播",
                divider: false,
            });
            previous = previous + re[re.length - 1].offset;

            v = v + 1;
        }
    }
    return re;
}

function displayTimeOffset(seconds: number): string {
    if (seconds > 0) {
        return "+ " + formatHMS(seconds);
    } else {
        return "− " + formatHMS(-seconds);
    }
}
</script>

<template>
    <n-grid x-gap="12" :cols="4" class="container">
        <n-gi>
            <label style="font-size: 18px">起始日期:</label>
            <n-date-picker type="date" v-model:value="filterBegTs" />
        </n-gi>
        <n-gi>
            <label style="font-size: 18px">结束日期:</label>
            <n-date-picker type="date" v-model:value="filterEndTs" />
        </n-gi>
        <n-gi>
            <label style="font-size: 18px">TAG:</label>
            <n-space vertical>
                <n-select
                    v-model:value="filterTag"
                    :options="tagOptions"
                    :consistent-menu-width="false"
                />
            </n-space>
        </n-gi>
    </n-grid>

    <n-divider />
    <n-grid x-gap="12" :cols="3" class="container">
        <n-gi class="vod-table" :span="2">
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
                                {{ truncateText(item.title, 30) }}
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
                            {{ item.duration }}
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </n-gi>
        <n-gi>
            <n-card title="时间计算明细">
                <div :style="{ 'text-align': 'right' }">
                    <div
                        class="vod-time-text"
                        :style="{ 'text-align': 'right' }"
                    >
                        {{ displayTimeOffset(vodTimeData[0].previous) }}
                    </div>
                    <n-divider vertical />
                    <div
                        class="vod-time-text"
                        :style="{ width: '20%', 'text-align': 'left' }"
                    >
                        初始
                    </div>
                </div>
                <template v-for="item in vodTimeData">
                    <n-divider v-if="item.divider" title-placement="left">
                        {{ item.date }}</n-divider
                    >
                    <div :style="{ 'text-align': 'right' }">
                        <div
                            class="vod-time-text"
                            :style="{ 'text-align': 'right' }"
                        >
                            {{ displayTimeOffset(item.offset) }}
                        </div>
                        <n-divider vertical />
                        <div
                            class="vod-time-text"
                            :style="{ width: '20%', 'text-align': 'left' }"
                        >
                            {{ item.reason }}
                        </div>
                    </div>
                </template>
            </n-card>
        </n-gi>
    </n-grid>
</template>

<style scoped>
.container {
    display: block;
    width: 90vw;
}

.vod-table {
    height: 70vh;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    overflow-y: scroll;
}

.vod-time-text {
    display: inline-block;
    font-weight: bold;
}
</style>
