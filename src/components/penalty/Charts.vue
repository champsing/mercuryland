<script setup lang="ts">
import { Ref, ref } from "vue";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    BarElement,
    ArcElement,
    CategoryScale,
    LinearScale,
    TimeScale,
    ChartOptions,
    ChartData,
} from "chart.js";
import { Bar, Doughnut } from "vue-chartjs";
import penaltyData from "@assets/data/penalty.json";
import penaltyStatus from "@assets/data/penalty_status.json";
import "chartjs-adapter-date-fns";

ChartJS.register(
    Title,
    Tooltip,
    BarElement,
    ArcElement,
    CategoryScale,
    LinearScale,
    TimeScale
);

const props = defineProps<{
    dateRange: [number, number];
    status?: string;
    search: string;
}>();

function filterPenaltyData(
    date: [number, number],
    status: string,
    search: string
): typeof penaltyData {
    return penaltyData
        .filter(
            (v) =>
                v.date >= new Date(date[0]).toISOString().slice(0, 10) &&
                v.date <=
                    new Date(date[1] + 28800000).toISOString().slice(0, 10)
        )
        .filter((v) => status == null || status == v.status)
        .filter(
            (v) =>
                search == "" ||
                v.name.toLowerCase().includes(search.toLowerCase())
        )
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));
}

let filteredData = filterPenaltyData(
    props.dateRange,
    props.status,
    props.search
);

let doughnutChartData: Ref<any> = ref(
    generateDoughnutChartData(filteredData)
);

let doughnutChartOptions = {
    maintainAspectRatio: false,
    plugins: {
        legend: {
            display: false,
        },
    },
    layout: {
        padding: 20,
    },
} as ChartOptions<"doughnut">;

let barChartData: Ref<any> = ref(generateBarChartData(filteredData));
let barChartOptions = {
    maintainAspectRatio: false,
    plugins: {
        legend: {
            display: false,
        },
    },
    scales: {
        x: {
            type: "time",
            grid: {
                display: false,
            },
            time: {
                minUnit: "day",
            },
        },
        y: {
            grid: {
                display: false,
            },
        },
    },
} as ChartOptions<"bar">;

function generateDoughnutChartData(
    filteredData: typeof penaltyData
): ChartData<"doughnut", number[], string> {
    return {
        labels: penaltyStatus.map((x) => x.name),
        datasets: [
            {
                label: null,
                data: penaltyStatus.map(
                    (x) => filteredData.filter((y) => x.name == y.status).length
                ),
                backgroundColor: penaltyStatus.map((x) => x.color),
                borderWidth: 0,
                hoverOffset: 50,
            },
        ],
    };
}

function generateBarChartData(
    filteredData: typeof penaltyData
): ChartData<"bar", number[], number> {
    return {
        labels: filteredData
            .map((x) => new Date(x.date).getTime())
            .filter((e, i, a) => e !== a[i - 1]), // may be wrong
        datasets: penaltyStatus.map((x) => {
            return {
                label: x.name,
                data: Array.from(Map.groupBy(filteredData, (d) => d.date))
                    .sort((lhs, rhs) => lhs[0].localeCompare(rhs[0]))
                    .map(
                        ([_, v]) => v.filter((y) => x.name == y.status).length
                    ),
                backgroundColor: x.color,
                stack: "0",
            };
        }),
    };
}
</script>

<template>
    <Doughnut
        :options="doughnutChartOptions"
        :data="doughnutChartData"
        class="h-40vh w-full p-0 m-0"
    />
    <Bar
        :options="barChartOptions"
        :data="barChartData"
        class="h-40vh w-full p-0 m-0"
    />
</template>
