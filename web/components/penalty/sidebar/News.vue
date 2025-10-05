<script setup lang="ts">
import {
    VaDivider,
    VaBadge,
    VaCard,
    VaCardContent,
    VaCardTitle,
} from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import { ref } from "vue";
import { statusOf } from "@/composables/penalty";

const latestPenalty = ref(penaltyData.slice().reverse()[0]);
</script>

<template>
    <VaCard
        style="--va-card-padding: 1rem"
        class="rounded-xl h-full flex flex-col"
    >
        <VaCardTitle class="!text-xl justify-center"> 最新 </VaCardTitle>
        <VaCardContent>
            <div
                class="flex flex-col justify-center gap-2 mb-2"
                item-responsive
            >
                <div
                    :class="`inline !text-[${
                        statusOf(latestPenalty.status).color
                    }] font-bold text-center`"
                >
                    ● {{ latestPenalty.status }}
                </div>

                <div class="text-center text-lg mb-3">
                    {{ latestPenalty.name }}xxxxxxxxx xxxxxx xxxxxxx xxxxxxxxxxx xxxxxxxxxx xxxxxxxxxxxx xxxxx xxxxxx
                </div>
            </div>
        </VaCardContent>
    </VaCard>
</template>
