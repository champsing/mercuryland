<script setup lang="ts">
import { ref } from "vue";
import {
    NButton,
    NCollapse,
    NCollapseItem,
    NDatePicker,
    NDrawer,
    NDivider,
    NDrawerContent,
    NGrid,
    NGi,
    NSelect,
    NInput,
    NList,
    NListItem,
    NThing,
    NTable,
    NSpace,
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
import { Doughnut } from "vue-chartjs";
import { truncateText, openLink } from "../composables/utils.ts";
import penaltyData from "../assets/penalty.json";
//import vodLinkData from "../assets/vod.json";
import penaltyStatus from "../assets/penalty_status.json";

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
        return value;
    },
});

let doughnutChartData = ref(generateDoughnutChartData(filteredData.value));
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

const activateDrawer = (item) => {
    isDrawerActive.value = true;
    penaltyContent.value = item;
};

const isDrawerActive = ref(false);
const penaltyContent = ref({
    id: "",
    date: "",
    youtube_vod: "", //["",""]
    name: "",
    done: "",
    description: "",
    //youtube_vod_2: ""
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

function queryStatusMetadata(status: string): (typeof penaltyStatus)[0] {
    return penaltyStatus.filter((x) => x.name == status)[0];
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
            <n-select
                v-model:value="filterStatus"
                :options="finishOptions"
                :consistent-menu-width="false"
            />
        </n-gi>
        <n-gi>
            <label style="font-size: 18px">搜索:</label>
            <n-input
                round
                placeholder="輸入懲罰內容來搜尋"
                v-model:value="filterSearch"
                type="text"
            />
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
                        <td
                            :style="{
                                'background-color': queryStatusMetadata(
                                    item.status
                                ).color,
                                color: queryStatusMetadata(item.status)
                                    .textColor,
                            }"
                        >
                            {{ item.date }}
                        </td>
                        <td
                            :style="{
                                'background-color': queryStatusMetadata(
                                    item.status
                                ).color,
                                color: queryStatusMetadata(item.status)
                                    .textColor,
                            }"
                        >
                            <n-button
                                @click="activateDrawer(item)"
                                :text="true"
                                :focusable="false"
                                :text-color="
                                    queryStatusMetadata(item.status).textColor
                                "
                            >
                                {{ truncateText(item.name, 30) }}
                            </n-button>
                        </td>

                        <td
                            :style="{
                                'background-color': queryStatusMetadata(
                                    item.status
                                ).color,
                                color: queryStatusMetadata(item.status)
                                    .textColor,
                            }"
                        >
                            {{ item.status }}
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </n-gi>
        <n-gi>
            <Doughnut
                ref="pieChart"
                :options="doughnutChartOptions"
                :data="doughnutChartData"
                class="pie"
            />
        </n-gi>
    </n-grid>

    <n-drawer v-model:show="isDrawerActive" :width="502" :placement="'right'">
        <n-drawer-content :title="penaltyContent.name">
            {{ penaltyContent.description }}
            <n-button @click="openLink(penaltyContent.youtube_vod)">
                直播連結
            </n-button>
        </n-drawer-content>
    </n-drawer>

    <div class="time">
        <svg id="barChart" height="90%"></svg>
    </div>
    <hr class="rounded" />

    <n-space>
        <br />
    </n-space>

    <n-collapse
        arrow-placement="right"
        style="
            --n-title-font-size: 24px;
            --n-title-text-color: rgb(11, 118, 225);
        "
    >
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

.time {
    height: 30vh;
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
