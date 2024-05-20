<script setup>
import { defineComponent, ref } from "vue";
import * as d3 from "d3";
import { NCode, NCollapse, NCollapseItem, NDatePicker, NSelect, NInput, NList, NListItem, NThing, NTable, NSpace } from "naive-ui";
// import { Doughnut, Line } from "vue-chartjs";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    BarElement,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
} from "chart.js";
ChartJS.register(
    Title,
    Tooltip,
    Legend,
    BarElement,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale
);

defineProps({});

// export default defineComponent({
//   setup() {
//     return {
//       value: ref([""]),
//       statuses: [
//         {
//           label: "未開始",
//           value: "0",
//         }, 
//         {
//           label: "已完成",
//           value: "1",
//         }, 
//         {
//           label: "勉強過",
//           value: "2",
//         }, 
//         {
//           label: "進行中",
//           value: "3",
//         },
//       ]
//     };
//   }
// });

var filterBegDate = ref("1970-01-01");
var filterEndDate = ref(new Date(Date.now()).toISOString().slice(0, 10));
var filterFinish = ref(-1);
var filterSearch = ref("");

const dataPath =
    "data.csv?random=" + Math.floor(Math.random() * 1000000).toString();

var dataSource = [];
var dataDisplay = ref([]);


d3.csv(dataPath, function (d) {
    dataSource.push(d);
    refresh();
});

function refresh() {
    dataDisplay.value = dataSource
        .filter(
            (v) =>
                v.date >= filterBegDate.value && v.date <= filterEndDate.value
        )
        .filter((v) => filterFinish.value == -1 || filterFinish.value == v.done)
        .filter(
            (v) =>
                filterSearch.value == "" || v.name.includes(filterSearch.value)
        )
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

    drawPieChart(dataDisplay.value);
    drawBarChart(dataDisplay.value);
}

function updateBegDate(newDate) {
    filterBegDate.value = newDate.toISOString().slice(0, 10);
    refresh();
}

function updateEndDate(newDate) {
    filterEndDate.value = newDate.toISOString().slice(0, 10);
    refresh();
}

function updateFinish(newFinish) {
    filterFinish.value = parseInt(newFinish);
    refresh();
}

function updateSearch(text) {
    filterSearch.value = text;
    refresh();
}

function statusToString(i) {
    if (i == 0) {
        return "未开始";
    } else if (i == 1) {
        return "已完成";
    } else if (i == 2) {
        return "勉强过";
    } else if (i == 3) {
        return "进行中";
    } else {
        return "undefined";
    }
}

//  表格填滿顏色
function statusToColor(i) {
    if (i == 0) {
        return "#922B21";
    } else if (i == 1) {
        return "#196F3D";
    } else if (i == 2) {
        return "#9D9D9D";
    } else if (i == 3) {
        return "#4DFFFF";
    } else {
        return "#000000";
    }
}

// 表格文字顏色
// i = 0 和 1 時返回白色，其他黑色
function statusToColorText(i) {
    if (i == 0 || i == 1) {
        return "#FFFFFF";
    } else {
        return "#000000";
    }
}


function truncateStr(s) {
    if (s.length > 32) {
        return s.substring(0, 30) + "...";
    } else {
        return s;
    }
}

function drawPieChart(dataIn) {
    const s0 = dataIn.filter((v) => v.done == 0).length;
    const s1 = dataIn.filter((v) => v.done == 1).length;
    const s2 = dataIn.filter((v) => v.done == 2).length;
    const s3 = dataIn.filter((v) => v.done == 3).length;

    const pieFn = d3.pie();
    const arcFn = d3.arc().innerRadius(0).outerRadius(0.9);
    const arc = pieFn([s0, s1, s2, s3]);

    d3.select("#pieChart").selectAll("*").remove();

    const svg = d3.select("#pieChart");

    svg.selectAll("arc")
        .data(arc)
        .enter()
        .append("g")
        .append("path")
        .attr("fill", (_, i) => statusToColor(i))
        .attr("d", arcFn);

    svg.selectAll("label")
        .data(arc.filter((d) => d.data != 0))
        .enter()
        .append("g")
        .attr("transform", (d) => "translate(" + arcFn.centroid(d) + ")")
        .append("text")
        .text((d) => d.data.toString())
        .style("text-anchor", "middle")
        .style("font-size", 0.1);
}

function drawBarChart(dataIn) {
    var series = Array.from(
        Map.groupBy(dataIn, (d) => d.date),
        ([key, value]) => [key, value]
    ).map(function (d) {
        return {
            date: d[0],
            s0: d[1].filter((v) => v.done == 0).length,
            s1: d[1].filter((v) => v.done == 1).length,
            s2: d[1].filter((v) => v.done == 2).length,
            s3: d[1].filter((v) => v.done == 3).length,
        };
    });

    let sep = 40;
    let pad = 10;

    // height = 27vh or 220 (10 for axis)
    // width = 50*n/220*27vh or 50*n

    let physicalH = 27;
    let logicalH = 200;
    let diagramH = logicalH - 50;

    let logicalW = sep * series.length;
    let physicalW = (logicalW / logicalH) * physicalH;

    let hScale =
        Math.max(...series.map((d) => d.s0 + d.s1 + d.s2 + d.s3)) / diagramH;

    d3.select("#barChart").selectAll("*").remove();

    const svg = d3
        .select("#barChart")
        .attr("width", physicalW + "vh")
        .attr("viewBox", "0 0 " + logicalW + " " + logicalH);

    svg.selectAll("s0")
        .data(series)
        .enter()
        .append("rect")
        .attr("x", (_, i) => i * sep)
        .attr("y", (d) => diagramH - d.s0 / hScale)
        .attr("width", sep - pad)
        .attr("height", (d) => d.s0 / hScale)
        .attr("fill", statusToColor(0));

    svg.selectAll("s1")
        .data(series)
        .enter()
        .append("rect")
        .attr("x", (_, i) => i * sep)
        .attr("y", (d) => diagramH - (d.s0 + d.s1) / hScale)
        .attr("width", sep - pad)
        .attr("height", (d) => d.s1 / hScale)
        .attr("fill", statusToColor(1));

    svg.selectAll("s2")
        .data(series)
        .enter()
        .append("rect")
        .attr("x", (_, i) => i * sep)
        .attr("y", (d) => diagramH - (d.s0 + d.s1 + d.s2) / hScale)
        .attr("width", sep - pad)
        .attr("height", (d) => d.s2 / hScale)
        .attr("fill", statusToColor(2));

    svg.selectAll("s3")
        .data(series)
        .enter()
        .append("rect")
        .attr("x", (_, i) => i * sep)
        .attr("y", (d) => diagramH - (d.s0 + d.s1 + d.s2 + d.s3) / hScale)
        .attr("width", sep - pad)
        .attr("height", (d) => d.s3 / hScale)
        .attr("fill", statusToColor(3));

    const xAxis = d3
        .scaleBand()
        .domain(series.map((d) => d.date))
        .range([0, logicalW])
        .padding(0.1);

    svg.append("g")
        .attr("transform", "translate(0," + diagramH + ")")
        .call(d3.axisBottom(xAxis))
        .selectAll("text")
        .style("text-anchor", "end")
        .attr("transform", "rotate(-45)");

}
</script>

<template>
    <div class="filter">
        <div>
            <label style="font-size: 18px;">起始日期:</label>
            <n-date-picker type="date" 
            v-model:value="timestamp"
            @input="updateBegDate($event.target.valueAsDate)"/>
            <input
                type="date"
                :value="filterBegDate.valueOf()"
                @input="updateBegDate($event.target.valueAsDate)"
                 
            />
        </div>
        <div>
            <label style="font-size: 18px;">结束日期:</label>
            <n-date-picker v-model:value="timestamp" type="date" />
            <input
                type="date"
                :value="filterEndDate.valueOf()"
                @input="updateEndDate($event.target.valueAsDate)"
            />
        </div>
        <div>
            <label style="font-size: 18px;">完成状态:</label>
            <n-space vertical>
                <n-select :options="statuses" :consistent-menu-width="false" />
            </n-space>    
            <select
                :value="filterFinish.valueOf()"
                @input="updateFinish($event.target.value)"
            >
                <option value="-1"></option>
                <option value="0">未开始</option>
                <option value="1">已完成</option>
                <option value="2">勉强过</option>
                <option value="3">进行中</option>
            </select>
        </div>
        <div>
            <label style="font-size: 18px;">搜索:</label>
            <n-space vertical>
                <n-input round placeholder="輸入懲罰內容來搜尋" 
                :value="filterSearch.valueOf()"
                @input="updateSearch($event.target.value)" />
                <input
                    :value="filterSearch.valueOf()"
                    @input="updateSearch($event.target.value)"
                />
            </n-space>
        </div>
    </div>

    <hr class="rounded" />

    <div class="main">
        <div class="detail">
            <n-table :bordered="true" size="small" style="text-align: center;">
                <thead>
                    <tr>
                        <td style="font-size: 18px;">日期</td>
                        <td style="font-size: 18px;">惩罚内容</td>
                        <td style="font-size: 18px;">完成状况</td>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="item in dataDisplay"
                    >
                        <td 
                        :style="{
                            'background-color': statusToColor(item.done),
                            'color': statusToColorText(item.done)
                        }"
                        >
                            {{ item.date }}
                        </td>
                        <td
                        :style="{
                            'background-color': statusToColor(item.done),
                            'color': statusToColorText(item.done)
                        }"
                        >
                        {{ truncateStr(item.name) }}
                        <!-- 顯示右側拉匣，標題為項目名稱，內容為項目敘述 -->
                        <!-- <n-button @click="activate('right')">
                            {{ truncateStr(item.name) }}
                        </n-button>
                        <n-drawer v-model:show="active" :width="502" :placement="placement">
                            <n-drawer-content title={{ truncateStr(item.name) }}>
                                {{ truncateStr(item.description) }}
                            </n-drawer-content>
                        </n-drawer> -->
                        </td>
                        <td
                        :style="{
                            'background-color': statusToColor(item.done),
                            'color': statusToColorText(item.done)
                        }"
                        >
                            {{ statusToString(item.done) }}
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </div>
        <div class="pie">
            <svg
                id="pieChart"
                width="100%"
                height="100%"
                viewBox="-1 -1 2 2"
            ></svg>
        </div>
    </div>

    <div class="time">
        <svg id="barChart" height="90%"></svg>
    </div>
    <hr class="rounded" />
    
    <n-space>
        <br />
    </n-space>

    <n-collapse arrow-placement="right" style="
    --n-title-font-size: 24px;
    --n-title-text-color: rgb(11,118,225); 
    ">
        <n-collapse-item title="懲罰語法" name="punish_syntax">
            <div style="overflow: auto">
                <n-list bordered>
                    <n-list-item>
                        <n-thing style="text-align: left; font-size: 18px">
                            <日期>: Unix Timestamp<br />
                            <編號>: int <懲罰主文>: string 〔詳細資料〕: additionalMetaData（執行狀態）: statusMetaData
                        </n-thing>
                    </n-list-item>
                </n-list>
            </div>
        </n-collapse-item>
        <n-collapse-item title="詳細資料" name="more_information">
            <div>
                <n-list bordered>
                    <n-list-item>
                        <n-thing style="font-size: 18px;">
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
                        <n-thing style="font-size: 18px;">
                            ✅完成、✅已抽、🏁原主人或投票給過、⏲️ ⚔️目前已完成進度
                        </n-thing>
                    </n-list-item>
                </n-list>
            </div>
        </n-collapse-item>
    </n-collapse>
    
</template>

<style scoped>
.filter {
    display: grid;
    align-content: space-evenly;
    grid-template-columns: auto auto auto auto;
    gap: 10px;
}

hr.rounded {
    border-top: 4px solid #bbb;
    border-radius: 2px;
}

.main {
    width: 90vw;
    display: grid;
    grid-template-columns: 80% 20%;
}

.detail {
    max-height: 50vh;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    overflow-y: scroll;
}

.pie {
    max-height: 50vh;
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
