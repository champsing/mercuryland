<script setup lang="ts">
import { ref, Ref } from "vue";
import {
    NButton,
    NCard,
    NCollapse,
    NCollapseItem,
    NDatePicker,
    NDivider,
    NGrid,
    NGi,
    NSelect,
    NInput,
    NModal,
    NList,
    NListItem,
    NThing,
    NTable,
} from "naive-ui";
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
import { openLinks, truncateText } from "../composables/utils.ts";
import penaltyData from "../assets/penalty.json";
import penaltyStatus from "../assets/penalty_status.json";
import vodData from "../assets/vod.json"
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

let filterBegTs = defineModel("filterBegTs", {
    default: 1672502400000,
    set(value) {
        filteredData.value = filterPenaltyData(
            value,
            filterEndTs.value,
            filterStatus.value,
            filterSearch.value
        );
        return value;
    },
});
let filterEndTs = defineModel("filterEndTs", {
    default: Date.now(),
    set(value) {
        filteredData.value = filterPenaltyData(
            filterBegTs.value,
            value,
            filterStatus.value,
            filterSearch.value
        );
        return value;
    },
});
let filterStatus = defineModel("filterFinish", {
    default: "",
    set(value) {
        filteredData.value = filterPenaltyData(
            filterBegTs.value,
            filterEndTs.value,
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
            filterBegTs.value,
            filterEndTs.value,
            filterStatus.value,
            value
        );
        return value;
    },
});
let finishOptions = penaltyStatus
    .map((x) => x.name)
    .concat([""])
    .sort()
    .map((x) => {
        return { label: x, value: x };
    });

let filteredData = defineModel("filteredData", {
    default: filterPenaltyData(0, Date.now(), "", ""),
    set(value) {
        doughnutChartData.value = generateDoughnutChartData(value);
        barChartData.value = generateBarChartData(value);
        return value;
    },
});

let doughnutChartData: Ref<any> = ref(generateDoughnutChartData(filteredData.value));
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
            type:"time",
            time: {
                minUnit: "day"
            }
        }
    }
} as ChartOptions<"bar">;

const activateModal = (item) => {
    isModalActive.value = true;
    penaltyContent.value = item;
};

const isModalActive = ref(false);
const penaltyContent = ref({
    id: "",
    date: "",
    name: "",
    status: "",
    description: "",
});
</script>

<script lang="ts">
function filterPenaltyData(
    begTs: number,
    endTs: number,
    status: string,
    search: string
): typeof penaltyData {
    return penaltyData
        .filter(
            (v) =>
                v.date >= new Date(begTs).toISOString().slice(0, 10) &&
                v.date <= new Date(endTs).toISOString().slice(0, 10)
        )
        .filter((v) => status == "" || status == v.status)
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
        labels: filteredData.map((x) => new Date(x.date).getTime()).filter((e, i, a) => e !== a[i - 1]), // may be wrong
        datasets: penaltyStatus.map(x => {
            return {
                label: x.name,
                data: Array.from(
                    Map.groupBy(filteredData, (d) => d.date)
                )
                    .sort((lhs, rhs) => lhs[0].localeCompare(rhs[0]))
                    .map(([_, v]) => v.filter((y) => x.name == y.status).length),
                backgroundColor: x.color,
                stack: "0",
            }
        })
    };
}

function queryStatusMetadata(status: string): (typeof penaltyStatus)[0] {
    return penaltyStatus.filter((x) => x.name == status)[0];
}

function vodLinkOfDate(date: string): Array<string> {
    return vodData.filter((x) => x.date == date).map((x) => x.link)
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
            <label style="font-size: 18px">完成状态:</label>
            <n-select v-model:value="filterStatus" :options="finishOptions" :consistent-menu-width="false" />
        </n-gi>
        <n-gi>
            <label style="font-size: 18px">搜索:</label>
            <n-input round placeholder="輸入懲罰內容來搜尋" v-model:value="filterSearch" type="text" />
        </n-gi>
    </n-grid>

    <n-divider />

    <n-grid x-gap="12" :cols="3" class="main">
        <n-gi class="detail" :span="2">
            <n-table :bordered="true" size="small" style="text-align: center">
                <thead>
                    <tr>
                        <td style="font-size: 18px">日期</td>
                        <td style="font-size: 18px">惩罚内容</td>
                        <td style="font-size: 18px">完成状况</td>
                    </tr>
                </thead>

                <tbody>
                    <tr v-for="item in filteredData">
                        <td :style="{
                            'background-color': queryStatusMetadata(
                                item.status
                            ).color,
                            color: queryStatusMetadata(item.status)
                                .textColor,
                        }">
                            {{ item.date }}
                        </td>
                        <td :style="{
                            'background-color': queryStatusMetadata(
                                item.status
                            ).color,
                            color: queryStatusMetadata(item.status)
                                .textColor,
                        }">
                            <n-button @click="activateModal(item)" :text="true" :focusable="false" :text-color="queryStatusMetadata(item.status).textColor">
                                {{ truncateText(item.name, 30) }}
                            </n-button>
                        </td>

                        <td :style="{
                            'background-color': queryStatusMetadata(
                                item.status
                            ).color,
                            color: queryStatusMetadata(item.status)
                                .textColor,
                        }">
                            {{ item.status }}
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </n-gi>
        <n-gi>
            <Doughnut ref="pieChart" :options="doughnutChartOptions" :data="doughnutChartData" class="pie" />
        </n-gi>
        <n-gi :span="3">
            <Bar ref="barChart" :options="barChartOptions" :data="barChartData" class="bar" />
        </n-gi>
    </n-grid>

    <n-modal v-model:show="isModalActive">
        <n-card
            style="width: 600px"
            :title="penaltyContent.name"
            :bordered="false"
            size="huge"
            role="dialog"
            aria-modal="true"
        >
            {{ penaltyContent.description }}
            <n-divider v-if="penaltyContent.description != ''" />
            <n-button @click="openLinks(vodLinkOfDate(penaltyContent.date))">
                直播转盘連結
            </n-button>
        </n-card>
    </n-modal>
  

    <n-divider />

    <n-collapse arrow-placement="right" style="
            --n-title-font-size: 24px;
            --n-title-text-color: rgb(11, 118, 225);
        ">
        <n-collapse-item title="懲罰語法" name="punish_syntax">
            <div style="overflow: auto">
                <n-list bordered>
                    <n-list-item>
                        <n-thing style="text-align: left; font-size: 18px">
                            &lt;日期&gt;: Unix Timestamp<br />
                            &lt;編號&gt;: int &lt;懲罰主文&gt;: string
                            〔詳細資料〕: additionalMetaData（執行狀態）:
                            statusMetaData
                        </n-thing>
                    </n-list-item>
                </n-list>
            </div>
        </n-collapse-item>
        <n-collapse-item title="詳細資料" name="more_information">
            <div>
                <n-list bordered>
                    <n-list-item>
                        <n-thing style="font-size: 18px">
                            🆙增加、🔁重抽、2️⃣備案、📝原主人修改n次、➕其他後來增加的條件
                        </n-thing>
                    </n-list-item>
                </n-list>
            </div>
        </n-collapse-item>
        <n-collapse-item title="完成狀態" name="punish_status">
            <div>
                <n-list bordered>
                    <n-list-item>
                        <n-thing style="font-size: 18px">
                            ✅完成、✅已抽、🏁原主人或投票給過、⏲️
                            ⚔️目前已完成進度
                        </n-thing>
                    </n-list-item>
                </n-list>
            </div>
        </n-collapse-item>
    </n-collapse>
</template>

<style scoped>
.main {
    width: 90vw;
}

.detail {
    height: 45vh;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    overflow-y: scroll;
}

.pie {
    height: 45vh;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
}

.bar {
    height: 40vh;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    overflow-x: scroll;
}

table {
    width: 100%;
}

.axis {
    font: 1px;
}

.read-the-docs {
    color: #888;
}
</style>
