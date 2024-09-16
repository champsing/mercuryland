<script setup lang="ts">
import { computed } from "vue";
import { NDivider, NCard, NScrollbar } from "naive-ui";
import { UseElementBounding, UseWindowSize } from "@vueuse/components";
import { parseHMS, formatHMS } from "@composables/utils.ts";
import vodLinkData from "@assets/data/vod.json";
import vodSchedule from "@assets/data/schedule.json";

class DataType {
    date: string;
    offset: number;
    previous: number;
    reason: string;
    divider: boolean;
}

const props = defineProps<{ dateRange: [number, number] }>();
const emit = defineEmits<{ (e: "computedTime", tag: number): void }>();
const rawData = calcRawData();
const data = computed(() => {
    let filtered = rawData.filter(
        (v: DataType) =>
            v.date >= new Date(props.dateRange[0]).toISOString().slice(0, 10) &&
            v.date <= new Date(props.dateRange[1] + 28800000).toISOString().slice(0, 10)
    );
    let i0 = filtered.findIndex((x: DataType) => x.divider);

    if (i0 == null) {
        return Array();
    } else {
        return filtered.slice(i0);
    }
});

const lastItem = rawData[rawData.length - 1];
emit("computedTime", lastItem.previous + lastItem.offset);

function calcRawData(): DataType[] {
    let re: DataType[] = [];

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

function format(seconds: number): string {
    return seconds > 0 ? "+ " + formatHMS(seconds) : "− " + formatHMS(-seconds);
}

function calcStyle(top: number, vh: number) {
    let ppb = 20; // parent padding
    let pbb = 1; // parent border
    let pmb = 8; // parent margin
    let p = ppb + pbb + pmb;
    let height = Math.max(vh * 0.2, vh - window.scrollY - top - p);
    return {
        height: "" + height + "px",
    };
}
</script>

<template>
    <n-card
        title="计算明细"
        class="text-center mb-2"
        :style="{ '--n-padding-left': 0 }"
    >
        <use-window-size v-slot="{ height }">
            <use-element-bounding v-slot="{ top }">
                <n-scrollbar :style="calcStyle(top, height)">
                    <div class="pr-6">
                        <template v-for="item in data">
                            <n-divider
                                v-if="item.divider"
                                title-placement="left"
                                class="!mt-0 !mb-0"
                            >
                                {{ item.date }}
                                <n-divider vertical />
                                {{ format(item.previous) }}
                            </n-divider>
                            <div class="text-right">
                                <div class="inline-block font-bold">
                                    {{ item.reason }}
                                </div>
                                <n-divider vertical />
                                <div class="inline-block font-bold">
                                    {{ format(item.offset) }}
                                </div>
                            </div>
                        </template>
                    </div>
                </n-scrollbar>
            </use-element-bounding>
        </use-window-size>
    </n-card>
</template>
