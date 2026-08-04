<script setup lang="ts">
import { computed, ref } from "vue";
import { PenItem, stateString, stateColor } from "@/composables/penalty";
import { parseDate } from "@/composables/utils";
import { VaButton, VaModal } from "vuestic-ui";
import VChart from "vue-echarts";
import { use } from "echarts/core";
import { BarChart } from "echarts/charts";
import { PieChart } from "echarts/charts";
import {
    GridComponent,
    TooltipComponent,
    TitleComponent,
    LegendComponent,
} from "echarts/components";
import { CanvasRenderer } from "echarts/renderers";

use([
    CanvasRenderer,
    BarChart,
    PieChart,
    GridComponent,
    TooltipComponent,
    TitleComponent,
    LegendComponent,
]);

type TimeRange = "5d" | "1m" | "6m" | "1y";

// ── The four finish states (excludes 0 = 未生效/inactive) ──
const FINISH_STATES = [1, 2, 3, 4] as const;

const props = defineProps<{
    penalties: PenItem[];
    modelValue: boolean;
}>();

const emit = defineEmits<{
    "update:modelValue": [value: boolean];
}>();

const timeRange = ref<TimeRange>("1m");

const rangeOptions: { value: TimeRange; label: string }[] = [
    { value: "5d", label: "5天" },
    { value: "1m", label: "1個月" },
    { value: "6m", label: "6個月" },
    { value: "1y", label: "1年" },
];

function getCutoff(range: TimeRange): Date {
    const days = { "5d": 5, "1m": 30, "6m": 180, "1y": 365 };
    const now = new Date();
    const cutoff = new Date(
        now.getFullYear(),
        now.getMonth(),
        now.getDate() - days[range],
    );
    return cutoff;
}

const filteredPenalties = computed(() => {
    const cutoff = getCutoff(timeRange.value);
    return props.penalties.filter((p) => {
        const d = parseDate(p.date);
        return d >= cutoff;
    });
});

const totalCount = computed(() => filteredPenalties.value.length);

// ── Column chart: stacked bars — one stack per finish state, grouped by date ──
const columnOption = computed(() => {
    const penalties = filteredPenalties.value;

    // collect all unique dates
    const dates = [...new Set(penalties.map((p) => p.date))].sort((a, b) =>
        a.localeCompare(b),
    );

    // for each finish state, build an array aligned with the dates
    const series = FINISH_STATES.map((state) => {
        const data = dates.map((date) => {
            return penalties.filter(
                (p) => p.date === date && p.state === state,
            ).length;
        });
        return {
            name: stateString(state),
            type: "bar" as const,
            stack: "total",
            data,
            color: stateColor(state, "raw"),
            itemStyle: {
                borderRadius: 0,
            },
            emphasis: {
                itemStyle: {
                    opacity: 0.85,
                },
            },
        };
    });

    // round the top of the topmost (last) series per bar
    // we do this by setting borderRadius on the last series
    if (series.length > 0) {
        series[series.length - 1].itemStyle = {
            ...series[series.length - 1].itemStyle,
            borderRadius: [4, 4, 0, 0],
        };
    }

    return {
        title: {
            text: "每日懲罰數量（按狀態）",
            left: "center",
            textStyle: { color: "#d1d5db", fontSize: 14 },
        },
        tooltip: {
            trigger: "axis" as const,
            backgroundColor: "rgba(24,24,27,0.94)",
            borderColor: "rgba(255,255,255,0.1)",
            textStyle: { color: "#d1d5db" },
            formatter: (
                params: { seriesName: string; value: number }[],
            ) => {
                const date = params[0]?.name ?? "";
                let html = `<strong>${date}</strong>`;
                for (const p of params) {
                    if (p.value > 0) {
                        html += `<br/>${p.seriesName}：${p.value}`;
                    }
                }
                return html;
            },
        },
        legend: {
            bottom: "0%",
            textStyle: { color: "#a1a1aa", fontSize: 11 },
            itemWidth: 10,
            itemHeight: 10,
        },
        grid: {
            left: "3%",
            right: "4%",
            bottom: "14%",
            top: "12%",
            containLabel: true,
        },
        xAxis: {
            type: "category" as const,
            data: dates,
            axisLabel: {
                color: "#a1a1aa",
                rotate: 45,
                fontSize: 10,
                formatter: (val: string) => val.slice(5),
            },
            axisLine: { lineStyle: { color: "rgba(255,255,255,0.08)" } },
            axisTick: { show: false },
        },
        yAxis: {
            type: "value" as const,
            minInterval: 1,
            axisLabel: { color: "#a1a1aa", fontSize: 10 },
            splitLine: {
                lineStyle: { color: "rgba(255,255,255,0.06)" },
            },
        },
        series,
    };
});

// ── Pie chart: proportion of the four finish states ──
const pieOption = computed(() => {
    const penalties = filteredPenalties.value;

    const stateCounts: Record<number, number> = {};
    for (const s of FINISH_STATES) stateCounts[s] = 0;

    for (const p of penalties) {
        if (p.state >= 1 && p.state <= 4) {
            stateCounts[p.state]++;
        }
    }

    const data = FINISH_STATES.map((state) => ({
        name: stateString(state),
        value: stateCounts[state],
        itemStyle: { color: stateColor(state, "raw") },
    })).filter((d) => d.value > 0);

    return {
        title: {
            text: "狀態分佈（僅完成狀態）",
            left: "center",
            textStyle: { color: "#d1d5db", fontSize: 14 },
        },
        tooltip: {
            trigger: "item" as const,
            backgroundColor: "rgba(24,24,27,0.94)",
            borderColor: "rgba(255,255,255,0.1)",
            textStyle: { color: "#d1d5db" },
            formatter: "{b}: {c} ({d}%)",
        },
        legend: {
            bottom: "0%",
            textStyle: { color: "#a1a1aa", fontSize: 11 },
        },
        series: [
            {
                type: "pie" as const,
                radius: ["40%", "70%"],
                center: ["50%", "45%"],
                avoidLabelOverlap: false,
                itemStyle: {
                    borderColor: "rgba(18,21,27,0.92)",
                    borderWidth: 2,
                },
                label: {
                    show: true,
                    color: "#a1a1aa",
                    fontSize: 10,
                    formatter: "{b}\n{d}%",
                },
                emphasis: {
                    label: { fontSize: 14, fontWeight: "bold" },
                    itemStyle: {
                        shadowBlur: 10,
                        shadowOffsetX: 0,
                        shadowColor: "rgba(0, 0, 0, 0.5)",
                    },
                },
                data,
            },
        ],
    };
});
</script>

<template>
    <VaModal
        :model-value="modelValue"
        @update:model-value="
            (value: boolean) => emit('update:modelValue', value)
        "
        size="large"
        close-button
        hide-default-actions
        :mobile-fullscreen="false"
        class="chart-stat-modal"
    >
        <div class="chart-stat__shell flex flex-col gap-4">
            <!-- Header row: title + time-range selector -->
            <div
                class="chart-stat__header flex flex-wrap items-center justify-between gap-3 mr-8"
            >
                <div
                    class="text-lg font-semibold text-zinc-300 flex items-center gap-2"
                >
                    懲罰統計圖表
                    <span
                        class="text-xs font-normal text-zinc-400 bg-white/[0.06] px-2 py-0.5 rounded-full"
                    >
                        {{ totalCount }} 筆
                    </span>
                </div>

                <!-- Mutual time-range selector (segmented control) -->
                <div
                    class="chart-stat__range flex rounded-lg bg-white/[0.06] p-0.5 gap-0.5 border border-white/[0.06]"
                    role="radiogroup"
                    aria-label="時間範圍"
                >
                    <VaButton
                        v-for="opt in rangeOptions"
                        :key="opt.value"
                        :color="
                            timeRange === opt.value
                                ? '#53b1b8'
                                : 'secondary'
                        "
                        :preset="
                            timeRange === opt.value
                                ? undefined
                                : 'secondary'
                        "
                        size="small"
                        class="chart-stat__range-btn !text-[0.72rem] !px-2.5 !py-1 !min-h-0 !rounded-md"
                        :class="{
                            '!bg-[#53b1b8]/15 !text-[#53b1b8] !font-bold':
                                timeRange === opt.value,
                        }"
                        @click="timeRange = opt.value"
                    >
                        {{ opt.label }}
                    </VaButton>
                </div>
            </div>

            <!-- Empty state: no penalties in range -->
            <div
                v-if="totalCount === 0"
                class="chart-stat__empty flex flex-col items-center justify-center gap-2 py-16 text-zinc-400"
            >
                <span class="text-4xl">📊</span>
                <span class="text-sm">此時間範圍內暫無懲罰記錄</span>
            </div>

            <!-- Charts grid -->
            <div
                v-else
                class="chart-stat__charts grid gap-4 grid-cols-1 md:grid-cols-2"
            >
                <div
                    class="chart-stat__card rounded-xl border border-white/[0.06] bg-[rgba(18,21,27,0.6)] p-3"
                >
                    <VChart
                        :option="columnOption"
                        autoresize
                        class="h-[320px] w-full"
                    />
                </div>
                <div
                    class="chart-stat__card rounded-xl border border-white/[0.06] bg-[rgba(18,21,27,0.6)] p-3"
                >
                    <VChart
                        :option="pieOption"
                        autoresize
                        class="h-[320px] w-full"
                    />
                </div>
            </div>
        </div>
    </VaModal>
</template>

<style scoped>
/* Wider dialog for two-chart layout */
:deep(.va-modal__dialog) {
    max-width: 960px !important;
    width: 95% !important;
}

@media (min-width: 768px) {
    .chart-stat-modal :deep(.va-modal__dialog) {
        max-width: 900px !important;
    }
}

/* Range button transitions */
.chart-stat__range-btn {
    transition: all 0.2s ease;
}

/* Only show empty state graphic when needed */
.chart-stat__card .echarts {
    min-height: 320px;
}
</style>
