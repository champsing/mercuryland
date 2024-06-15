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
    <div class="join_intro">
            現在就立刻加入我們。
    </div>
    <n-divider class="!mt-2 !mb-2" />
    <div class="java">
        <img src="@assets/images/java_edition.png">
    </div>
    <div class="copy">
        IP: XXX [複製]
    </div>
    <div class="copy">
        種子碼: XXX [複製]
    </div>
    <div class="ver">
        版本: 正版 1.21
    </div>
    <div class="join_guide">
        1. 加入我們的Discord群組來申請白名單 https://discord.gg/A2cMZRr
    </div>
    <div class="join_guide">
        2. 閱讀伺服器規則
    </div>
    <div class="join_guide">
        包含《水星法》、《水星伺服器破壞舉報獎勵規則》等。
    </div>
    <div class="join_guide">
        3. 在#申請伺服打上Minecraft ID
    </div>
    <div class="join_guide">
        4. 等待白名單申請通過
    </div>
    
    <n-grid x-gap="12" :cols="4" class="w-11/12">
        
    </n-grid>



    <n-grid x-gap="12" :cols="3" class="w-11/12 h-80vh overflow-y-hidden">
        
    </n-grid>
    
    <n-divider />
</template>
