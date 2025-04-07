<script setup lang="ts">
import { VaDivider } from "vuestic-ui";
import Statistics from "./Statistics.vue";
import Syntax from "./Syntax.vue";
import LatestPenalty from "./LatestPenalty.vue";
</script>

<template>
    <div class="flex flex-col m-auto mt-0">
        <Statistics />
        <VaDivider class="m-3" />
        <Syntax />
        <LatestPenalty class="mt-10" />
    </div>

    <!-- <VaChip class="vachip2" color="#3d807c" readonly>
            <VaIcon size="large" class="mt-1 mr-2">
                <InfoCircle />
            </VaIcon>
            <div class="text-center text-amber-200">
                <div class="text-lg mt-1">將滑鼠移至圖表上可查看數量</div>
            </div>
        </VaChip> -->
    <!-- <VaAlert class="mt-4" color="#3d807c" closeable>
            <div class="flex flex-row items-center w-72">
                <VaIcon size="large" class="mr-2">
                    <InfoCircle />
                </VaIcon>
                <div class="text-center text-amber-200">
                    <div class="text-lg">圖表維護中，敬請期待開放</div>
                </div>
            </div>
        </VaAlert> -->
</template>
