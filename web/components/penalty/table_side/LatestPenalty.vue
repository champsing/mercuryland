<script setup lang="ts">
import { VaDivider, VaBadge } from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import { ref } from "vue";
import { statusOf } from "@/composables/penalty";

const latestPenalty = ref(penaltyData.reverse()[0]);
</script>

<template>
    <div class="flex flex-col m-auto">
        <div class="flex flex-row justify-center">
            <VaBadge text="NEW" color="#B3D943" class="mr-2" />
            <div class="text-center text-xl">最新懲罰</div>
        </div>

        <VaDivider class="m-4" />

        <div class="flex flex-col justify-center gap-3 mb-0" item-responsive>
            <div class="text-center text-base">
                {{ latestPenalty.date }} 抽出
            </div>

            <div
                :class="`inline !text-[${
                    statusOf(latestPenalty.status).color
                }] font-bold text-center`"
            >
                ● {{ latestPenalty.status }}
            </div>

            <div class="text-center text-2xl mb-3">
                {{ latestPenalty.name }}
            </div>
        </div>

        <VaDivider class="m-3" />
    </div>
</template>
