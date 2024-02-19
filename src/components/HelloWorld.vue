<script setup>
import { ref } from "vue";
import * as d3 from "d3";

defineProps({
    msg: String,
});

var filterBegDate = ref("1970-01-01");
var filterEndDate = ref(new Date(Date.now()).toISOString().slice(0, 10));
var filterFinish = ref(2);
var filterSearch = ref("");

var dataSource = [];
var dataDisplay = ref([]);
d3.csv("public/data.csv", function (d) {
    dataSource.push(d);
    refresh();
});

function refresh() {
    dataDisplay.value = dataSource
        .filter(
            (v) =>
                v.date >= filterBegDate.value && v.date <= filterEndDate.value
        )
        .filter((v) => filterFinish.value == 2 || filterFinish.value == v.done)
        .filter(
            (v) =>
                filterSearch.value == "" || v.name.includes(filterSearch.value)
        )
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

    drawPieChart(dataDisplay.value);
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

function numToYN(i) {
    if (i == 0) {
        return "N";
    } else {
        return "Y";
    }
}

function numToColor(i) {
    if (i == 0) {
        return "#922B21";
    } else {
        return "#196F3D";
    }
}

function truncateStr(s) {
    if (s.length > 12) {
        return s.substring(0, 10) + "...";
    } else {
        return s;
    }
}

function drawPieChart(dataIn) {
    const positive = dataIn.filter((v) => v.done == 1).length;
    const negative = dataIn.filter((v) => v.done == 0).length;

    const pieFn = d3.pie();
    const arcFn = d3.arc().innerRadius(0).outerRadius(0.9);
    const arc = pieFn([negative, positive]);

    d3.select("#pieChart").selectAll("*").remove();

    const svg = d3.select("#pieChart");

    svg.selectAll("arc")
        .data(arc)
        .enter()
        .append("g")
        .append("path")
        .attr("fill", (_, i) => numToColor(i))
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

    console.log(arc);
}

function drawBarChart() {}
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
                <option value="2"></option>
                <option value="1">已完成</option>
                <option value="0">未完成</option>
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
                            'background-color': numToColor(item.done),
                        }"
                    >
                        <td>
                            {{ item.date }}
                        </td>
                        <td>
                            {{ truncateStr(item.name) }}
                        </td>
                        <td>
                            {{ numToYN(item.done) }}
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="pie">
            <svg id="pieChart" viewBox="-1 -1 2 2"></svg>
        </div>
    </div>

    <div class="time"></div>
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
    width: 100vw;
    display: grid;
    grid-template-columns: 70% 30%;
}

.detail {
    max-height: 60vh;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    overflow-y: scroll;
}

.pie {
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
}

.time {
    height: 20vh;
    width: 100%;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    overflow-x: scroll;
}

table {
    width: 100%;
}

.read-the-docs {
    color: #888;
}
</style>
