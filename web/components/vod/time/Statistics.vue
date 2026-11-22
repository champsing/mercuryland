<script setup lang="ts">
import { parseDate } from "@composables/utils";
import {
    durationHistogram,
    formatHMS,
    movingAverage,
    parseHMS,
    summarizeDurations,
    VodItem,
} from "@composables/vod";
import { BarChart, LineChart } from "echarts/charts";
import {
    GridComponent,
    LegendComponent,
    TooltipComponent,
} from "echarts/components";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { computed, ref } from "vue";
import { VaIcon } from "vuestic-ui";
import VChart from "vue-echarts";

use([
    CanvasRenderer,
    BarChart,
    LineChart,
    GridComponent,
    TooltipComponent,
    LegendComponent,
]);

type TimeRange = "1m" | "3m" | "6m" | "1y" | "max";

interface DurationPoint {
    date: string;
    title: string;
    seconds: number;
}

const props = defineProps<{ vodData: VodItem[] }>();

const rangeOptions: { value: TimeRange; label: string }[] = [
    { value: "1m", label: "1 個月" },
    { value: "3m", label: "3 個月" },
    { value: "6m", label: "6 個月" },
    { value: "1y", label: "1 年" },
    { value: "max", label: "∞" },
];

const RANGE_DAYS: Record<Exclude<TimeRange, "max">, number> = {
    "1m": 30,
    "3m": 90,
    "6m": 180,
    "1y": 365,
};

const timeRange = ref<TimeRange>("6m");

const allPoints = computed<DurationPoint[]>(() =>
    props.vodData
        .map((vod) => ({
            date: vod.date,
            title: vod.title,
            seconds: parseHMS(vod.duration),
        }))
        .filter((point) => point.seconds > 0)
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date)),
);

function startOfToday(): Date {
    const now = new Date();
    return new Date(now.getFullYear(), now.getMonth(), now.getDate());
}

function daysBefore(date: Date, days: number): Date {
    const result = new Date(date);
    result.setDate(result.getDate() - days);
    return result;
}

const points = computed(() => {
    if (timeRange.value === "max") return allPoints.value;

    const cutoff = daysBefore(startOfToday(), RANGE_DAYS[timeRange.value]);
    return allPoints.value.filter((point) => parseDate(point.date) >= cutoff);
});

// The equally long window right before the selected one, for the delta tile.
const previousPoints = computed(() => {
    if (timeRange.value === "max") return [];

    const days = RANGE_DAYS[timeRange.value];
    const cutoff = daysBefore(startOfToday(), days);
    const previousStart = daysBefore(cutoff, days);

    return allPoints.value.filter((point) => {
        const date = parseDate(point.date);
        return date >= previousStart && date < cutoff;
    });
});

const summary = computed(() =>
    summarizeDurations(points.value.map((point) => point.seconds)),
);

const previousSummary = computed(() =>
    summarizeDurations(previousPoints.value.map((point) => point.seconds)),
);

const averageDelta = computed(() => {
    if (
        timeRange.value === "max" ||
        summary.value.count === 0 ||
        previousSummary.value.count === 0
    ) {
        return null;
    }

    const delta = summary.value.average - previousSummary.value.average;
    const percent =
        previousSummary.value.average > 0
            ? (delta / previousSummary.value.average) * 100
            : null;

    return { delta, percent };
});

const deltaClass = computed(() => {
    if (!averageDelta.value) return "text-zinc-500";
    return averageDelta.value.delta >= 0 ? "text-emerald-300" : "text-rose-300";
});

const deltaLabel = computed(() => {
    if (averageDelta.value === null) {
        return timeRange.value === "max" ? "全期間" : "前期暫無資料";
    }

    const { delta, percent } = averageDelta.value;
    const sign = delta >= 0 ? "+" : "−";

    if (percent === null) {
        return `較前期 ${sign} ${formatHMS(Math.abs(delta))}`;
    }

    return `較前期 ${sign} ${Math.abs(percent).toFixed(1)}%`;
});

const extremes = computed(() => {
    if (points.value.length === 0) return null;

    let longest = points.value[0];
    let shortest = points.value[0];

    for (const point of points.value) {
        if (point.seconds > longest.seconds) longest = point;
        if (point.seconds < shortest.seconds) shortest = point;
    }

    return { longest, shortest };
});

const rollingSeconds = computed(() =>
    movingAverage(
        points.value.map((point) => point.seconds),
        5,
    ),
);

const HTML_ESCAPES: Record<string, string> = {
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "'": "&#39;",
};

/** Tooltips render HTML, and stream titles come from the API. */
function escapeHtml(text: string): string {
    return text.replace(/[&<>"']/g, (char) => HTML_ESCAPES[char]);
}

function timestamp(date: string): number {
    return parseDate(date).getTime();
}

/* ── 時長走勢 ── */
const trendOption = computed(() => {
    const data = points.value;
    const rolling = rollingSeconds.value;

    return {
        backgroundColor: "transparent",
        color: ["#5bc6a1", "#fbbf24"],
        grid: {
            left: 8,
            right: 16,
            top: 38,
            bottom: 8,
            containLabel: true,
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
                type: "line" as const,
                lineStyle: { color: "rgba(255,255,255,0.18)" },
            },
            formatter: (params: { dataIndex: number }[]) => {
                const index = params[0]?.dataIndex ?? -1;
                const point = data[index];
                if (!point) return "";

                return `<div style="font-weight:600;color:#fafafa;margin-bottom:2px">${escapeHtml(point.date)}</div>
                    <div style="color:#a1a1aa;font-size:12px;margin-bottom:8px;max-width:220px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${escapeHtml(point.title)}</div>
                    <div style="display:flex;align-items:center;gap:8px;color:#d4d4d8">
                        <span style="width:8px;height:8px;border-radius:999px;background:#5bc6a1"></span>
                        <span style="flex:1">本場時長</span>
                        <strong style="color:#fafafa">${formatHMS(point.seconds)}</strong>
                    </div>
                    <div style="display:flex;align-items:center;gap:8px;color:#d4d4d8;margin-top:4px">
                        <span style="width:8px;height:8px;border-radius:999px;background:#fbbf24"></span>
                        <span style="flex:1">近 5 場平均</span>
                        <strong style="color:#fafafa">${formatHMS(rolling[index] ?? point.seconds)}</strong>
                    </div>`;
            },
        },
        legend: {
            top: 4,
            right: 4,
            icon: "roundRect",
            itemWidth: 10,
            itemHeight: 10,
            itemGap: 16,
            textStyle: { color: "#a1a1aa", fontSize: 12 },
            inactiveColor: "#3f3f46",
        },
        xAxis: {
            type: "time" as const,
            axisLabel: {
                color: "#71717a",
                fontSize: 11,
                hideOverlap: true,
                formatter: (value: number) => {
                    const date = new Date(value);
                    return `${date.getMonth() + 1}/${date.getDate()}`;
                },
            },
            axisLine: {
                lineStyle: { color: "rgba(255,255,255,0.08)" },
            },
            axisTick: { show: false },
            splitLine: { show: false },
        },
        yAxis: {
            type: "value" as const,
            axisLabel: {
                color: "#71717a",
                fontSize: 11,
                formatter: "{value} h",
            },
            splitLine: {
                lineStyle: {
                    color: "rgba(255,255,255,0.05)",
                    type: "dashed" as const,
                },
            },
        },
        series: [
            {
                name: "每場時長",
                type: "line" as const,
                data: data.map((point) => [
                    timestamp(point.date),
                    point.seconds / 3600,
                ]),
                symbol: "circle",
                symbolSize: 4,
                lineStyle: { width: 1.5, color: "#5bc6a1", opacity: 0.6 },
                itemStyle: { color: "#5bc6a1", opacity: 0.8 },
                areaStyle: {
                    color: {
                        type: "linear",
                        x: 0,
                        y: 0,
                        x2: 0,
                        y2: 1,
                        colorStops: [
                            { offset: 0, color: "rgba(91,198,161,0.16)" },
                            { offset: 1, color: "rgba(91,198,161,0)" },
                        ],
                    },
                },
                emphasis: { focus: "series" as const },
            },
            {
                name: "近 5 場平均",
                type: "line" as const,
                data: data.map((point, index) => [
                    timestamp(point.date),
                    (rolling[index] ?? point.seconds) / 3600,
                ]),
                smooth: true,
                showSymbol: false,
                lineStyle: { width: 2.5, color: "#fbbf24" },
                itemStyle: { color: "#fbbf24" },
                emphasis: { focus: "series" as const },
            },
        ],
        animationDuration: 550,
        animationEasing: "cubicOut" as const,
    };
});

/* ── 時長區間分布 ── */
const BUCKET_EDGES = [3600, 7200, 10800, 14400, 21600];
const BUCKET_LABELS = [
    "少於 1 小時",
    "1–2 小時",
    "2–3 小時",
    "3–4 小時",
    "4–6 小時",
    "6 小時以上",
];

const distributionOption = computed(() => {
    const counts = durationHistogram(
        points.value.map((point) => point.seconds),
        BUCKET_EDGES,
    );
    const maxCount = Math.max(0, ...counts);
    const total = summary.value.count;

    return {
        backgroundColor: "transparent",
        grid: {
            left: 8,
            right: 36,
            top: 8,
            bottom: 8,
            containLabel: true,
        },
        tooltip: {
            trigger: "axis" as const,
            axisPointer: { type: "shadow" as const },
            backgroundColor: "rgba(24, 24, 27, 0.94)",
            borderColor: "rgba(255,255,255,0.1)",
            borderWidth: 1,
            padding: [10, 14],
            extraCssText:
                "border-radius:12px;box-shadow:0 12px 32px rgba(0,0,0,0.4);",
            textStyle: { color: "#e4e4e7", fontSize: 13 },
            formatter: (params: { name: string; value: number }[]) => {
                const item = params[0];
                if (!item) return "";

                const percent =
                    total > 0 ? ((item.value / total) * 100).toFixed(1) : "0.0";

                return `<div style="font-weight:600;color:#fafafa">${item.name}</div>
                    <div style="color:#a1a1aa;font-size:12px;margin-top:4px">${item.value} 場 · ${percent}%</div>`;
            },
        },
        xAxis: {
            type: "value" as const,
            minInterval: 1,
            axisLabel: { color: "#71717a", fontSize: 11 },
            splitLine: {
                lineStyle: {
                    color: "rgba(255,255,255,0.05)",
                    type: "dashed" as const,
                },
            },
        },
        yAxis: {
            type: "category" as const,
            inverse: true,
            data: BUCKET_LABELS,
            axisLabel: { color: "#a1a1aa", fontSize: 12 },
            axisTick: { show: false },
            axisLine: { show: false },
        },
        series: [
            {
                type: "bar" as const,
                data: counts.map((count) => ({
                    value: count,
                    itemStyle: {
                        color:
                            count === maxCount && count > 0
                                ? "#5bc6a1"
                                : "rgba(91,198,161,0.35)",
                        borderRadius: [0, 6, 6, 0] as [
                            number,
                            number,
                            number,
                            number,
                        ],
                    },
                })),
                barMaxWidth: 18,
                label: {
                    show: true,
                    position: "right" as const,
                    color: "#d4d4d8",
                    fontSize: 12,
                    formatter: (params: { value: number }) =>
                        params.value > 0 ? `${params.value} 場` : "",
                },
                emphasis: { itemStyle: { color: "#7ee7c0" } },
            },
        ],
        animationDuration: 550,
        animationEasing: "cubicOut" as const,
    };
});
</script>

<template>
    <section
        aria-labelledby="vod-duration-heading"
        class="flex flex-col gap-4 pb-4"
    >
        <div class="flex flex-wrap items-start justify-between gap-3">
            <div class="min-w-0">
                <div class="flex flex-wrap items-baseline gap-x-2">
                    <h2
                        id="vod-duration-heading"
                        class="text-xl font-bold text-neutral-100"
                    >
                        直播時長區間分析
                    </h2>
                    <span
                        class="text-[0.72rem] font-extrabold uppercase text-[#45d483]"
                    >
                        Stream Duration
                    </span>
                </div>
                <p class="mt-1 text-sm text-zinc-400">
                    觀察一段時間內直播長度的變化，以及各時長區間的場次分布。
                </p>
            </div>

            <div
                class="flex max-w-full items-center gap-1 overflow-x-auto rounded-full border border-white/10 bg-white/5 p-1 shadow-[0_2px_12px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.08)] backdrop-blur-md"
                role="group"
                aria-label="時間範圍"
            >
                <button
                    v-for="option in rangeOptions"
                    :key="option.value"
                    type="button"
                    :aria-pressed="timeRange === option.value"
                    class="shrink-0 rounded-full px-3 py-1 text-xs font-medium text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                    :class="
                        timeRange === option.value
                            ? 'bg-gradient-to-b from-white/20 to-white/10 text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.2),0_1px_3px_rgba(0,0,0,0.25)]'
                            : ''
                    "
                    @click="timeRange = option.value"
                >
                    {{ option.label }}
                </button>
            </div>
        </div>

        <div
            v-if="summary.count === 0"
            class="flex flex-col items-center justify-center gap-2 rounded-2xl border border-white/[0.06] bg-[rgba(18,21,27,0.6)] py-16 text-zinc-400"
        >
            <span class="text-4xl" aria-hidden="true">📊</span>
            <span class="text-sm">此時間範圍內暫無直播紀錄</span>
        </div>

        <template v-else>
            <div class="grid grid-cols-2 gap-3 md:grid-cols-4">
                <article
                    class="rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.65)] p-3 md:p-4"
                >
                    <p class="text-xs font-medium text-zinc-400">直播場次</p>
                    <p
                        class="mt-1.5 text-2xl font-black leading-none tabular-nums text-zinc-100 md:text-3xl"
                    >
                        {{ summary.count }}
                        <span class="ml-1 text-sm font-semibold text-zinc-500"
                            >場</span
                        >
                    </p>
                    <p class="mt-2 text-[0.68rem] text-zinc-500">此區間</p>
                </article>

                <article
                    class="rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.65)] p-3 md:p-4"
                >
                    <p class="text-xs font-medium text-zinc-400">平均時長</p>
                    <p
                        class="mt-1.5 text-2xl font-black leading-none tabular-nums text-zinc-100 md:text-3xl"
                    >
                        {{ formatHMS(summary.average) }}
                    </p>
                    <p
                        class="mt-2 flex items-center gap-1 text-[0.68rem]"
                        :class="deltaClass"
                    >
                        <VaIcon
                            v-if="averageDelta"
                            :name="
                                averageDelta.delta < 0
                                    ? 'trending_down'
                                    : 'trending_up'
                            "
                            size="small"
                            aria-hidden="true"
                        />
                        {{ deltaLabel }}
                    </p>
                </article>

                <article
                    class="rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.65)] p-3 md:p-4"
                >
                    <p class="text-xs font-medium text-zinc-400">中位數</p>
                    <p
                        class="mt-1.5 text-2xl font-black leading-none tabular-nums text-zinc-100 md:text-3xl"
                    >
                        {{ formatHMS(summary.median) }}
                    </p>
                    <p class="mt-2 text-[0.68rem] text-zinc-500">
                        不受極端場次影響
                    </p>
                </article>

                <article
                    class="rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.65)] p-3 md:p-4"
                >
                    <p class="text-xs font-medium text-zinc-400">總直播時長</p>
                    <p
                        class="mt-1.5 text-2xl font-black leading-none tabular-nums text-zinc-100 md:text-3xl"
                    >
                        {{ formatHMS(summary.total) }}
                    </p>
                    <p class="mt-2 text-[0.68rem] text-zinc-500">累計</p>
                </article>
            </div>

            <div class="grid gap-3 lg:grid-cols-3">
                <div
                    class="min-w-0 overflow-hidden rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.65)] p-3 lg:col-span-2"
                >
                    <header
                        class="flex flex-wrap items-start justify-between gap-2 px-1 pb-1"
                    >
                        <div class="min-w-0">
                            <h3 class="text-sm font-semibold text-zinc-100">
                                時長走勢
                            </h3>
                            <p
                                v-if="extremes"
                                class="mt-0.5 text-[0.68rem] text-zinc-500"
                            >
                                最長
                                {{ formatHMS(extremes.longest.seconds) }}（{{
                                    extremes.longest.date
                                }}）・最短
                                {{ formatHMS(extremes.shortest.seconds) }}（{{
                                    extremes.shortest.date
                                }}）
                            </p>
                        </div>
                        <span
                            class="shrink-0 text-[0.65rem] font-bold uppercase tracking-[0.18em] text-[#45d483]/80"
                        >
                            Trend
                        </span>
                    </header>
                    <VChart
                        :option="trendOption"
                        autoresize
                        class="h-[280px] w-full md:h-[340px]"
                    />
                    <p class="sr-only">
                        時長走勢圖：共 {{ summary.count }} 場直播，平均
                        {{ formatHMS(summary.average) }}。
                    </p>
                </div>

                <div
                    class="min-w-0 overflow-hidden rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.65)] p-3"
                >
                    <header
                        class="flex flex-wrap items-start justify-between gap-2 px-1 pb-1"
                    >
                        <h3 class="text-sm font-semibold text-zinc-100">
                            時長區間分布
                        </h3>
                        <span
                            class="shrink-0 text-[0.65rem] font-bold uppercase tracking-[0.18em] text-[#45d483]/80"
                        >
                            Distribution
                        </span>
                    </header>
                    <VChart
                        :option="distributionOption"
                        autoresize
                        class="h-[280px] w-full md:h-[340px]"
                    />
                    <p class="sr-only">各時長區間的直播場次分布圖。</p>
                </div>
            </div>
        </template>
    </section>
</template>
