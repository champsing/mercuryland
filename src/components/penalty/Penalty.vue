<script setup lang="ts">
import { ref, Ref } from "vue";
import {
    NButton,
    NCard,
    NDatePicker,
    NDivider,
    NGrid,
    NGi,
    NSelect,
    NIcon,
    NInput,
    NModal,
    NTable,
} from "naive-ui";
import { VaButton, VaButtonGroup, VaChip } from "vuestic-ui";
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
import { openLink, openLinks, ofId } from "@composables/utils.ts";
import penaltyData from "@assets/data/penalty.json";
import penaltyStatus from "@assets/data/penalty_status.json";
import vodData from "@assets/data/vod.json";
import PenaltySyntax from "./PenaltySyntax.vue";
import "chartjs-adapter-date-fns";
import { InfoCircle } from "@vicons/tabler";

ChartJS.register(
    Title,
    Tooltip,
    BarElement,
    ArcElement,
    CategoryScale,
    LinearScale,
    TimeScale
);

let filterDate: Ref<[number, number]> = defineModel("filterDate", {
    default: [1672502400000, Date.now()] as const,
    set(value: [number, number]) {
        filteredData.value = filterPenaltyData(
            value,
            filterStatus.value,
            filterSearch.value
        );
        return value;
    },
});

let filterStatus = defineModel("filterStatus", {
    default: null,
    set(value) {
        filteredData.value = filterPenaltyData(
            filterDate.value,
            value,
            filterSearch.value
        );
        return value;
    },
});

let filterSearch = defineModel("filterSearch", {
    default: "",
    set(value) {
        filteredData.value = filterPenaltyData(
            filterDate.value,
            filterStatus.value,
            value
        );
        return value;
    },
});

let finishOptions = [{ label: "", value: null }].concat(
    penaltyStatus
        .map((x) => x.name)
        .sort()
        .map((x) => {
            return { label: x, value: x };
        })
);

let filteredData = defineModel("filteredData", {
    default: filterPenaltyData([0, Date.now()], null, ""),
    set(value) {
        doughnutChartData.value = generateDoughnutChartData(value);
        barChartData.value = generateBarChartData(value);
        return value;
    },
});

let doughnutChartData: Ref<any> = ref(
    generateDoughnutChartData(filteredData.value)
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

let barChartData: Ref<any> = ref(generateBarChartData(filteredData.value));
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

const showModal = ref(false);
const modalContent: Ref<PenaltyDataEntry> = defineModel("modalContent", {
    default: null,
    set(value) {
        showModal.value = !showModal.value;
        return value;
    },
});
</script>

<script lang="ts">
class PenaltyDataEntry {
    id: number;
    date: string;
    name: string;
    status: string;
    description: { block: string; str?: string; uri?: string }[];
}

function filterPenaltyData(
    date: [number, number],
    status: string,
    search: string
): typeof penaltyData {
    return penaltyData
        .filter(
            (v) =>
                v.date >= new Date(date[0]).toISOString().slice(0, 10) &&
                v.date <= new Date(date[1]).toISOString().slice(0, 10)
        )
        .filter((v) => status == null || status == v.status)
        .filter(
            (v) =>
                search == "" ||
                v.name.toLowerCase().includes(search.toLowerCase())
        )
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));
}

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

function statusOf(status: string): (typeof penaltyStatus)[0] {
    return penaltyStatus.filter((x) => x.name == status)[0];
}

function vodLinkOfDate(date: string): string[] {
    return vodData.filter((x) => x.date == date).map((x) => x.link);
}
</script>

<template>
    <n-grid x-gap="12" :cols="4" class="w-11/12">
        <n-gi :span="2">
            <n-date-picker type="daterange" v-model:value="filterDate" />
        </n-gi>
        <n-gi>
            <n-select
                v-model:value="filterStatus"
                :options="finishOptions"
                placeholder="請選擇一種完成狀態"
                :consistent-menu-width="false"
            />
        </n-gi>
        <n-gi>
            <n-input
                round
                placeholder="輸入懲罰內容來搜尋"
                v-model:value="filterSearch"
                type="text"
            />
        </n-gi>
    </n-grid>

    <n-divider class="!mt-2 !mb-2" />

    <n-grid x-gap="12" :cols="3" class="w-11/12 h-80vh overflow-y-hidden">
        <n-gi :span="2" class="h-40vh w-full p-0 m-0 overflow-y-scroll">
            <n-table :bordered="true" size="small" class="text-center w-full">
                <thead>
                    <tr>
                        <td class="font-bold">日期</td>
                        <td class="font-bold">惩罚内容</td>
                        <td class="font-bold">完成状况</td>
                    </tr>
                </thead>

                <tbody>
                    <!-- !bg-[#b91c1c] !bg-[#4d7c0f] !bg-[#047857] !bg-[#b45309] -->
                    <!-- TAILWIND CSS: DO NOT REMOVE ABOVE COMMENT -->
                    <tr v-for="item in filteredData">
                        <td :class="`!bg-[${statusOf(item.status).color}]`">
                            {{ item.date }}
                        </td>
                        <td :class="`!bg-[${statusOf(item.status).color}]`">
                            <n-button
                                @click="modalContent = item"
                                :text="true"
                                :focusable="false"
                            >
                                {{ item.name }}
                            </n-button>
                        </td>
                        <td :class="`!bg-[${statusOf(item.status).color}]`">
                            {{ item.status }}
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </n-gi>
        <n-gi>
            <Doughnut
                :options="doughnutChartOptions"
                :data="doughnutChartData"
                class="h-40vh w-full p-0 m-0"
            />
        </n-gi>
        <n-gi :span="3">
            <Bar
                :options="barChartOptions"
                :data="barChartData"
                class="h-40vh w-full p-0 m-0"
            />
        </n-gi>
    </n-grid>

    <n-modal v-model:show="showModal">
        <n-card
            style="width: 600px"
            :title="modalContent.name"
            :bordered="false"
            size="huge"
            role="dialog"
            aria-modal="true"
        >
            <template #header-extra>
                <n-button @click="openLinks(vodLinkOfDate(modalContent.date))">
                    直播转盘連結
                </n-button>
            </template>
            <template v-for="block in modalContent.description">
                <span v-if="block.block == 'text'">{{ block.str }}</span>
                <n-button
                    v-if="block.block == 'link'"
                    @click="openLink(block.uri)"
                    :text="true"
                    :focusable="false"
                >
                    {{ block.str }}
                </n-button>

                <n-button
                    v-if="block.block == 'vod'"
                    @click="openLink(ofId(vodData, parseInt(block.uri)).link)"
                    :text="true"
                    :focusable="false"
                >
                    {{ block.str }}
                </n-button>

                <img
                    v-if="block.block == 'image'"
                    :src="`penalty/${block.uri}`"
                    :alt="block.str"
                />
                <br v-if="block.block == 'br'" />
            </template>
        </n-card>
    </n-modal>

    <div class="left-margin flex flex-wrap gap-5">
        <VaChip class="vachip2" color="#3d807c">
            <n-icon size="25" class="mt-1 mr-2">
                <InfoCircle />
            </n-icon>
            <div class="text-center text-amber-200">
                <div class="text-lg mt-1">將滑鼠移至圖表上可查看數量</div>
            </div>
        </VaChip>
        <div>
            <VaButtonGroup round class="overall-button" disabled>
                <VaButton color="danger">現存</VaButton>
                <VaButton color="success">完成</VaButton>
            </VaButtonGroup>
        </div>
    </div>

    <!-- overall list -->
    <!-- <div v-for="item in penaltyData.filter((x) => x.status == '未開始' || '進行中')">
        {{ item.name }}
    </div>
    <div v-for="item in penaltyData.filter((x) => x.status == '已完成' || '勉強過')">
        {{ item.name }}
    </div> -->

    <PenaltySyntax class="mb-4" />
</template>

<style>
.vachip2 {
    margin-top: 0.5rem;
    margin-bottom: 1rem;
    --va-chip-border: 0.2rem solid transparent;
    --va-chip-font-size: 1.15rem;
    --va-chip-padding: 0 1.2rem;
}
.left-margin {
    margin-left: 25%;
}
.overall-button {
    margin-top: 0.65rem;
    --va-button-group-font-size: 1.15rem;
    --va-button-group-border-radius: 2px;
    --va-button-group-button-padding: 0.3rem;
    --va-button-group-button-width: 90px;
}
</style>
