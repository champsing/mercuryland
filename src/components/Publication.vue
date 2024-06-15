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
        在這裡可以找到所有規則、伺服器名簿與檔案。
    </div>

    <n-divider class="!mt-2 !mb-2" />

    <div class="law_or_data">
        大標 法規與資料
    </div>
    <div class="embed">
        許多嵌入 maybe iframe...
    </div>
    <div class="embed">
        許多嵌入 maybe iframe...
    </div>
    <div class="embed">
        許多嵌入 maybe iframe...
    </div>

    <n-divider class="!mt-2 !mb-2" />

    <div class="worldsave">
        大標 地圖檔
    </div>
    <div class="embed">
        許多嵌入 maybe iframe... 第一季
    </div>
    <div class="embed">
        許多嵌入 maybe iframe... 第二～五季
    </div>
    <div class="embed">
        許多嵌入 maybe iframe... 第六、七季
    </div>
    <n-divider class="!mt-2 !mb-2" />
    <div class="worldsave">
        大標 活動合影
    </div>
    <div class="embed">
        許多嵌入 maybe iframe... 第五季關服合影
    </div>

    <n-divider class="!mt-2 !mb-2" />


    <n-grid x-gap="12" :cols="4" class="w-11/12">
        
    </n-grid>

    <n-divider class="!mt-2 !mb-2" />

    <n-grid x-gap="12" :cols="3" class="w-11/12 h-80vh overflow-y-hidden">
        
    </n-grid>

</template>
