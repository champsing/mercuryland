<script setup>
import { ref } from "vue";
import * as d3 from "d3";

defineProps({});

var filterBegDate = ref("1970-01-01");
var filterEndDate = ref(new Date(Date.now()).toISOString().slice(0, 10));
var filterFinish = ref(2);
var filterSearch = ref("");

var dataSource = [];
var dataDisplay = ref([]);
d3.csv("/data.csv", function (d) {
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
        return "已抽出";
    } else if (i == 3) {
        return "勉强过";
    } else if (i == 4) {
        return "进行中";
    } else {
        return "undefined";
    }
}

function statusToColor(i) {
    if (i == 0) {
        return "#922B21";
    } else if (i == 1) {
        return "#196F3D";
    } else if (i == 2) {
        return "#9AFF02";
    } else if (i == 3) {
        return "#9D9D9D";
    } else if (i == 4) {
        return "#4DFFFF";
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
    const s4 = dataIn.filter((v) => v.done == 4).length;

    const pieFn = d3.pie();
    const arcFn = d3.arc().innerRadius(0).outerRadius(0.9);
    const arc = pieFn([s0, s1, s2, s3, s4]);

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
            s4: d[1].filter((v) => v.done == 4).length,
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
        Math.max(...series.map((d) => d.s0 + d.s1 + d.s2 + d.s3 + d.s4)) /
        diagramH;

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

    svg.selectAll("s4")
        .data(series)
        .enter()
        .append("rect")
        .attr("x", (_, i) => i * sep)
        .attr(
            "y",
            (d) => diagramH - (d.s0 + d.s1 + d.s2 + d.s3 + d.s4) / hScale
        )
        .attr("width", sep - pad)
        .attr("height", (d) => d.s4 / hScale)
        .attr("fill", statusToColor(4));

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
            <label>起始日期:</label>
            <input
                type="date"
                :value="filterBegDate.valueOf()"
                @input="updateBegDate($event.target.valueAsDate)"
            />
        </div>
        <div>
            <label>结束日期:</label>
            <input
                type="date"
                :value="filterEndDate.valueOf()"
                @input="updateEndDate($event.target.valueAsDate)"
            />
        </div>
        <div>
            <label>完成状态:</label>
            <select
                :value="filterFinish.valueOf()"
                @input="updateFinish($event.target.value)"
            >
                <option value="-1"></option>
                <option value="0">未开始</option>
                <option value="1">已完成</option>
                <option value="2">已抽出</option>
                <option value="3">勉强过</option>
                <option value="4">进行中</option>
            </select>
        </div>
        <div>
            <label>搜索:</label>
            <input
                :value="filterSearch.valueOf()"
                @input="updateSearch($event.target.value)"
            />
        </div>
    </div>

    <hr class="rounded" />

    <div class="main">
        <div class="detail">
            <table>
                <thead>
                    <tr>
                        <td>日期</td>
                        <td>惩罚内容</td>
                        <td>完成状况</td>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="item in dataDisplay"
                        :style="{
                            'background-color': statusToColor(item.done),
                        }"
                    >
                        <td>
                            {{ item.date }}
                        </td>
                        <td>
                            {{ truncateStr(item.name) }}
                        </td>
                        <td>
                            {{ statusToString(item.done) }}
                        </td>
                    </tr>
                </tbody>
            </table>
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
