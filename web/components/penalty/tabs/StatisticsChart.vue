<script setup lang="ts">
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { parseDate } from "@/composables/utils";
import { BarChart, PieChart } from "echarts/charts";
import {
    GridComponent,
    LegendComponent,
    TitleComponent,
    TooltipComponent,
} from "echarts/components";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { computed, ref } from "vue";
import VChart from "vue-echarts";

use([
    CanvasRenderer,
    BarChart,
    PieChart,
    GridComponent,
    TooltipComponent,
    TitleComponent,
    LegendComponent,
]);

type TimeRange = "1d" | "1w" | "1m" | "6m" | "1y" | "max";
const FINISH_STATES = [1, 2, 3, 4] as const;

const props = defineProps<{ penalties: PenItem[] }>();

const timeRange = ref<TimeRange>("1m");
const rangeOptions: { value: TimeRange; label: string }[] = [
    { value: "1d", label: "1 天" },
    { value: "1w", label: "1 週" },
    { value: "1m", label: "1 個月" },
    { value: "6m", label: "6 個月" },
    { value: "1y", label: "1 年" },
    { value: "max", label: "∞" },
];

function getCutoff(range: TimeRange): Date {
    const days = {
        "1d": 1,
        "1w": 7,
        "1m": 30,
        "6m": 180,
        "1y": 365,
        max: 999999,
    };
    const now = new Date();
    return new Date(
        now.getFullYear(),
        now.getMonth(),
        now.getDate() - days[range],
    );
}

const filteredPenalties = computed(() => {
    const cutoff = getCutoff(timeRange.value);
    return props.penalties.filter((p) => {
        const d = parseDate(p.date);
        return d >= cutoff;
    });
});

const totalCount = computed(() => filteredPenalties.value.length);

/* ── 柱狀圖 ── */
const columnOption = computed(() => {
    const penalties = filteredPenalties.value;
    const dates = [...new Set(penalties.map((p) => p.date))].sort((a, b) =>
        a.localeCompare(b),
    );

    const series = FINISH_STATES.map((state, idx) => {
        const data = dates.map(
            (date) =>
                penalties.filter((p) => p.date === date && p.state === state)
                    .length,
        );
        const isLast = idx === FINISH_STATES.length - 1;
        return {
            name: stateString(state),
            type: "bar" as const,
            stack: "total",
            data,
            color: stateColor(state, "raw"),
            barWidth: dates.length > 15 ? "46%" : "58%",
            itemStyle: {
                // 修復 TS：部分型別只接受 number
                borderRadius: (isLast ? [8, 8, 0, 0] : 0) as [
                    number,
                    number,
                    number,
                    number,
                ],
            },
            emphasis: {
                focus: "series" as const,
                itemStyle: {
                    shadowBlur: 16,
                    shadowColor: "rgba(0,0,0,0.45)",
                },
            },
        };
    });

    return {
        backgroundColor: "transparent",
        title: {
            text: "每日懲罰數量",
            subtext: "依完成狀態堆疊",
            left: "center",
            top: 8,
            textStyle: {
                color: "#f4f4f5",
                fontSize: 20,
                fontWeight: 600,
                letterSpacing: 0.2,
            },
            subtextStyle: {
                color: "#a4a4a8",
                fontSize: 14,
                fontWeight: 400,
            },
        },
        tooltip: {
            trigger: "axis" as const,
            backgroundColor: "rgba(24, 24, 27, 0.94)",
            borderColor: "rgba(255,255,255,0.1)",
            borderWidth: 1,
            padding: [10, 14],
            extraCssText:
                "border-radius:12px;box-shadow:0 12px 32px rgba(0,0,0,0.4);",
            textStyle: { color: "#e4e4e7", fontSize: 13 },
            axisPointer: {
                type: "shadow" as const,
                shadowStyle: { color: "rgba(255,255,255,0.04)" },
            },
            formatter: (
                params: {
                    seriesName: string;
                    value: number;
                    marker: string;
                    name?: string;
                }[],
            ) => {
                const date = params[0]?.name ?? "";
                let html = `<div style="font-weight:600;margin-bottom:8px;font-size:13px;color:#fafafa">${date}</div>`;
                let total = 0;
                for (const p of params) {
                    if (p.value > 0) {
                        html += `<div style="display:flex;align-items:center;gap:8px;margin:4px 0;color:#d4d4d8">
                            ${p.marker}<span style="flex:1">${p.seriesName}</span>
                            <strong style="color:#fafafa">${p.value}</strong>
                        </div>`;
                        total += p.value;
                    }
                }
                html += `<div style="border-top:1px solid rgba(255,255,255,0.08);margin-top:8px;padding-top:8px;display:flex;justify-content:space-between;color:#a1a1aa;font-size:12px">
                    <span>合計</span><strong style="color:#fafafa">${total}</strong>
                </div>`;
                return html;
            },
        },
        legend: {
            bottom: 4,
            textStyle: {
                color: "#a1a1aa",
                fontSize: 14,
                fontWeight: 500,
            },
            itemWidth: 10,
            itemHeight: 10,
            itemGap: 18,
            icon: "circle",
            inactiveColor: "#3f3f46",
        },
        grid: {
            left: "3%",
            right: "3%",
            bottom: "14%",
            top: "30%",
            containLabel: true,
        },
        xAxis: {
            type: "category" as const,
            data: dates,
            axisLabel: {
                color: "#71717a",
                rotate: dates.length > 10 ? 40 : 0,
                fontSize: 11,
                fontWeight: 500,
                margin: 10,
                formatter: (val: string) => val.slice(5),
            },
            axisLine: {
                lineStyle: { color: "rgba(255,255,255,0.08)" },
            },
            axisTick: { show: false },
        },
        yAxis: {
            type: "value" as const,
            minInterval: 1,
            axisLabel: {
                color: "#71717a",
                fontSize: 14,
                fontWeight: 500,
            },
            splitLine: {
                lineStyle: {
                    color: "rgba(255,255,255,0.05)",
                    type: "dashed" as const,
                },
            },
        },
        animationDuration: 550,
        animationEasing: "cubicOut" as const,
        series,
    };
});

/* ── 圓餅圖 ── */
const pieOption = computed(() => {
    const penalties = filteredPenalties.value;
    const stateCounts: Record<number, number> = {};
    for (const s of FINISH_STATES) stateCounts[s] = 0;
    for (const p of penalties) {
        if (p.state >= 1 && p.state <= 4) stateCounts[p.state]++;
    }

    const data = FINISH_STATES.map((state) => ({
        name: stateString(state),
        value: stateCounts[state],
        itemStyle: {
            color: stateColor(state, "raw"),
            shadowBlur: 12,
            shadowColor: `${stateColor(state, "raw")}40`,
        },
    })).filter((d) => d.value > 0);

    return {
        backgroundColor: "transparent",
        title: {
            text: "懲罰狀態圓餅圖",
            left: "center",
            top: 8,
            textStyle: {
                color: "#f4f4f5",
                fontSize: 20,
                fontWeight: 600,
                letterSpacing: 0.2,
            },
            subtextStyle: {
                color: "#a4a4a8",
                fontSize: 14,
                fontWeight: 400,
            },
        },
        tooltip: {
            trigger: "item" as const,
            backgroundColor: "rgba(24, 24, 27, 0.94)",
            borderColor: "rgba(255,255,255,0.1)",
            borderWidth: 1,
            padding: [10, 14],
            extraCssText:
                "border-radius:12px;box-shadow:0 12px 32px rgba(0,0,0,0.4);",
            textStyle: { color: "#e4e4e7", fontSize: 13 },
            formatter: (params: {
                name: string;
                value: number;
                percent: number;
                marker: string;
            }) => {
                return `<div style="display:flex;align-items:center;gap:10px">
                    ${params.marker}
                    <div>
                        <div style="font-weight:600;color:#fafafa">${params.name}</div>
                        <div style="color:#a1a1aa;font-size:12px;margin-top:2px">${params.value} 筆 · ${params.percent}%</div>
                    </div>
                </div>`;
            },
        },
        legend: {
            bottom: 4,
            textStyle: {
                color: "#a1a1aa",
                fontSize: 14,
                fontWeight: 500,
            },
            itemWidth: 10,
            itemHeight: 10,
            itemGap: 18,
            icon: "circle",
            inactiveColor: "#3f3f46",
        },
        series: [
            {
                type: "pie" as const,
                radius: ["44%", "70%"],
                center: ["50%", "48%"],
                avoidLabelOverlap: true,
                padAngle: 2.5,
                itemStyle: {
                    borderColor: "#0f0f11",
                    borderWidth: 3,
                    borderRadius: 5,
                },
                label: {
                    show: true,
                    color: "#a1a1aa",
                    fontSize: 12,
                    fontWeight: 500,
                    lineHeight: 16,
                    formatter: "{b}\n{d}%",
                },
                labelLine: {
                    length: 14,
                    length2: 10,
                    lineStyle: { color: "rgba(255,255,255,0.18)" },
                },
                emphasis: {
                    scale: true,
                    scaleSize: 6,
                    label: {
                        fontSize: 12,
                        fontWeight: "bold",
                        color: "#f4f4f5",
                    },
                    itemStyle: {
                        shadowBlur: 22,
                        shadowOffsetX: 0,
                        shadowColor: "rgba(0,0,0,0.55)",
                    },
                },
                data,
            },
        ],
        animationDuration: 700,
        animationEasing: "cubicOut" as const,
    };
});
</script>

<template>
    <section class="flex flex-col gap-4" aria-label="懲罰圖表">
        <!-- Header row: title + time-range selector -->
        <div class="flex flex-wrap items-center justify-between gap-3">
            <div class="flex flex-wrap items-center gap-2">
                <h3 class="text-base font-extrabold text-[#f7f7f8]">圖表</h3>
                <span
                    class="rounded-full bg-white/[0.06] px-2 py-0.5 text-xs font-normal text-zinc-400"
                >
                    {{ totalCount }} 筆
                </span>
            </div>

            <!-- Mutual time-range selector, styled like the page tabs -->
            <div
                class="flex max-w-full items-center gap-1 overflow-x-auto rounded-full border border-white/10 bg-white/5 p-1 shadow-[0_2px_12px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.08)] backdrop-blur-md"
                role="group"
                aria-label="時間範圍"
            >
                <button
                    v-for="opt in rangeOptions"
                    :key="opt.value"
                    type="button"
                    :aria-pressed="timeRange === opt.value"
                    class="shrink-0 rounded-full px-3 py-1 text-xs font-medium text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                    :class="
                        timeRange === opt.value
                            ? 'bg-gradient-to-b from-white/20 to-white/10 text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.2),0_1px_3px_rgba(0,0,0,0.25)]'
                            : ''
                    "
                    @click="timeRange = opt.value"
                >
                    {{ opt.label }}
                </button>
            </div>
        </div>

        <!-- Empty state: no penalties in range -->
        <div
            v-if="totalCount === 0"
            class="flex flex-col items-center justify-center gap-2 rounded-2xl border border-white/[0.06] bg-[rgba(18,21,27,0.6)] py-16 text-zinc-400"
        >
            <span class="text-4xl">📊</span>
            <span class="text-sm">此時間範圍內暫無懲罰記錄</span>
        </div>

        <!-- Charts grid -->
        <div v-else class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div
                class="chart-stat__card rounded-xl border border-white/[0.06] bg-[rgba(18,21,27,0.6)] p-3"
            >
                <VChart
                    :option="columnOption"
                    autoresize
                    class="h-[280px] w-full md:h-[320px]"
                />
            </div>
            <div
                class="chart-stat__card rounded-xl border border-white/[0.06] bg-[rgba(18,21,27,0.6)] p-3"
            >
                <VChart
                    :option="pieOption"
                    autoresize
                    class="h-[280px] w-full md:h-[320px]"
                />
            </div>
        </div>
    </section>
</template>

<style scoped>
/* Only show empty state graphic when needed */
.chart-stat__card .echarts {
    min-height: 280px;
}

/* 卡片 hover 更柔和 */
.chart-stat__card {
    position: relative;
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.025);
    border: 1px solid rgba(255, 255, 255, 0.06);
    padding: 14px 12px 8px;
    transition:
        border-color 0.25s,
        box-shadow 0.25s;
}
.chart-stat__card:hover {
    border-color: rgba(255, 255, 255, 0.1);
    box-shadow: 0 8px 28px rgba(0, 0, 0, 0.28);
}
</style>
