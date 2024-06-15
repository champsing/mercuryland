<script setup lang="ts">
import { ref, Ref } from "vue";
import { NDatePicker, NGrid, NGi, NSelect, NDivider } from "naive-ui";
import { parseHMS } from "../composables/utils.ts";
import vodLinkData from "../assets/data/vod.json";
import vodSchedule from "../assets/data/schedule.json";
import DataTable from "./vod/DataTable.vue";
import TimeSummary from "./vod/TimeSummary.vue";
import TimeDetail from "./vod/TimeDetail.vue";

const vodTimeData = calculateVodTime();

let filterDate: Ref<[number, number]> = defineModel("filterDate", {
    default: [1577836800000, Date.now()] as const,
    set(value: [number, number]) {
        filteredVodTime.value = filterVodTimeData(vodTimeData, value);
        return value;
    },
});
let filterTag: Ref<string> = defineModel("filterTag", {
    default: null,
    set(value) {
        return value;
    },
});
let tagOptions = ref(
    [{ label: "", value: null }].concat(
        [...new Set(vodLinkData.flatMap((x) => x.tags))].sort().map((x) => {
            return { label: x, value: x };
        })
    )
);
let filteredVodTime = defineModel("filteredVodTime", {
    default: filterVodTimeData(calculateVodTime(), [0, Date.now()]),
});
</script>

<script lang="ts">
class VodTimeEntry {
    date: string;
    offset: number;
    previous: number;
    reason: string;
    divider: boolean;
}

function filterVodTimeData(
    data: VodTimeEntry[],
    ts: [number, number]
): VodTimeEntry[] {
    let filtered = data.filter(
        (v) =>
            v.date >= new Date(ts[0]).toISOString().slice(0, 10) &&
            v.date <= new Date(ts[1]).toISOString().slice(0, 10)
    );
    let i0 = filtered.findIndex((x) => x.divider);

    if (i0 == null) {
        return Array();
    } else {
        return filtered.slice(i0);
    }
}

function calculateVodTime(): VodTimeEntry[] {
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

        while (o < ove.length && new Date(ove[o].date) < date) {
            re.push({
                date: ove[o].date,
                offset: parseHMS(ove[o].duration),
                previous: previous,
                reason: ove[o].reason,
                divider: false,
            });
            previous = previous + re[re.length - 1].offset;

            o = o + 1;
        }
    }
    return re;
}

function computeFinalTime(entry: VodTimeEntry): number {
    return entry.previous + entry.offset;
}
</script>

<template>
    <n-grid x-gap="12" :cols="4" class="w-11/12">
        <n-gi :span="2">
            <n-date-picker type="daterange" v-model:value="filterDate" />
        </n-gi>
        <n-gi>
            <n-select
                v-model:value="filterTag"
                :options="tagOptions"
                placeholder="请选择直播的TAG"
                :consistent-menu-width="false"
            />
        </n-gi>
    </n-grid>

    <n-divider class="!mt-2 !mb-2" />

    <n-grid x-gap="8" :cols="3" class="w-11/12">
        <n-gi :span="2" class="w-full p-0 m-0">
            <DataTable
                :dateRange="filterDate"
                :tagOption="filterTag"
                @updateTag="(tag) => (filterTag = tag)"
            />
        </n-gi>
        <n-gi>
            <TimeSummary
                :t="computeFinalTime(vodTimeData[vodTimeData.length - 1])"
            />
            <TimeDetail :dateRange="filterDate" />
            <!-- <n-card
                title="计算明细"
                class="text-center h-2/3 overflow-y-scroll"
            >
                <template v-for="item in filteredVodTime">
                    <n-divider
                        v-if="item.divider"
                        title-placement="left"
                        class="!mt-0 !mb-0"
                    >
                        {{ item.date }}
                        <n-divider vertical />
                        {{ showTimeOffset(item.previous) }}
                    </n-divider>
                    <div class="text-right">
                        <div
                            class="w-1/5 inline-block font-bold"
                            :style="{ width: '20%' }"
                        >
                            {{ item.reason }}
                        </div>
                        <n-divider vertical />
                        <div class="inline-block font-bold">
                            {{ showTimeOffset(item.offset) }}
                        </div>
                    </div>
                </template>
            </n-card> -->
        </n-gi>
    </n-grid>
</template>
