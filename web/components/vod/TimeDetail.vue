<script setup lang="ts">
import { computed, watch } from "vue";
import {
    VaCard,
    VaCardContent,
    VaDivider,
    VaScrollContainer,
    VaList,
    VaListLabel,
    VaListItem,
    VaListItemSection,
} from "vuestic-ui";
import { UseElementBounding, UseWindowSize } from "@vueuse/components";
import { parseHMS, formatHMS } from "@composables/utils.ts";
import vodSchedule from "@assets/data/schedule.json";

class DataType {
    date: string;
    offset: number;
    previous: number;
    reason: string;
    divider: boolean;
}

interface VodItem {
    id?: number | null;
    date: string;
    link: string;
    title: string;
    tags: string[];
    duration: string;
}

const props = defineProps<{
    dateRange: { start: Date; end: Date };
    vodData: VodItem[];
}>();
const emit = defineEmits<{ (e: "computedTime", tag: number): void }>();
const rawData = computed(() => calcRawData(props.vodData));
const data = computed(() => {
    let filtered = rawData.value.filter(
        (v: DataType) =>
            v.date >= props.dateRange.start.toISOString().slice(0, 10) &&
            v.date <=
                new Date(props.dateRange.end.getTime() + 28800000)
                    .toISOString()
                    .slice(0, 10)
    );
    let i0 = filtered.findIndex((x: DataType) => x.divider);

    if (i0 == null) return Array();
    else return filtered.slice(i0);
});

watch(
    rawData,
    (value) => {
        const lastItem = value[value.length - 1];
        const total = lastItem ? lastItem.previous + lastItem.offset : 0;
        emit("computedTime", total);
    },
    { immediate: true }
);

function calcRawData(vodItems: VodItem[]): DataType[] {
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
    let vod = vodItems
        .filter((x) => x.date > date.toISOString().slice(0, 10))
        .slice()
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

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
            reason: "計劃",
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
    let parentPaddingBottom = 20;
    let parentBorderBottom = 1;
    let parentMarginBottom = 8;
    let footnoteHeight = 48;
    let parentPaddingBottom2 = 8;

    let delta = parentPaddingBottom + parentBorderBottom + parentMarginBottom + footnoteHeight + parentPaddingBottom2;
    let height = Math.max(vh * 0.2, vh - window.scrollY - top - delta);
    return {
        height: "" + height + "px",
    };
}
</script>

<template>
    <VaCard style="--va-card-padding: 0rem" class="h-full vod-card rounded-xl">
        <VaCardContent>
            <VaList style="--va-list-label-padding: 1rem">
                <VaListLabel class="text-xl" color="textPrimary">
                    計算明細
                </VaListLabel>
                <use-window-size v-slot="{ height }">
                    <use-element-bounding v-slot="{ top }">
                        <VaScrollContainer
                            vertical
                            color="#e0feb4"
                            size="medium"
                            :style="calcStyle(top, height)"
                        >
                            <template v-for="item in data">
                                <VaListItem class="text-center mb-2">
                                    <VaListItemSection>
                                        <VaDivider
                                            v-if="item.divider"
                                            orientation="left"
                                            class="mb-3"
                                        >
                                            <div class="flex">
                                                <div>
                                                    {{ item.date }}
                                                </div>
                                                <VaDivider
                                                    vertical
                                                    class="-mt-2 -mb-2"
                                                />
                                                <div>
                                                    {{ format(item.previous) }}
                                                </div>
                                            </div>
                                        </VaDivider>

                                        <div
                                            class="flex justify-end text-right mr-2"
                                        >
                                            <div class="font-bold">
                                                {{ item.reason }}
                                            </div>
                                            <VaDivider vertical />
                                            <div class="font-bold">
                                                {{ format(item.offset) }}
                                            </div>
                                        </div>
                                    </VaListItemSection>
                                </VaListItem>
                            </template>
                        </VaScrollContainer>
                    </use-element-bounding>
                </use-window-size>
            </VaList>
        </VaCardContent>
    </VaCard>
</template>
