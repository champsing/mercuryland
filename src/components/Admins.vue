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
import { openLink, openLinks, ofId } from "../composables/utils.ts";
import penaltyData from "../assets/data/penalty.json";
import penaltyStatus from "../assets/data/penalty_status.json";
import vodData from "../assets/data/vod.json";
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


</script>

<template>
    <div class="admin_intro">
        伺服器管理員
    </div>
    <img src="@assets/images/avatar_oreki20.png" />
    <div class="oreki">
        惡靈 Oreki
    </div>
    <div class="oreki_intro">
        伺服主（CEO）、伺服器總管、伺服器白名單管理、水星伺服器管理委員會主委、爭議協調與排解委員
    </div>
    <div class="youtube_button">
        惡靈YouTube頻道按鈕
    </div>
    <img src="@assets/images/avatar_champsing.png" />
    <div class="champsing">
        香榭 Champsing
    </div>
    <div class="champsing_intro">
        網站設計、伺服器管理、伺服器資料庫維護、伺服器營運總監、創意總監、法務總監、水星伺服器管理委員會委員、爭議協調與排解委員
    </div>
    <div class="logic">
        邏輯 Logic
    </div>
    <img src="@assets/images/avatar_logic.png" />
    <div class="logic_intro">
        網站設計
    </div>

    <n-grid x-gap="12" :cols="4" class="w-11/12">
        
    </n-grid>
</template>
