<script setup lang="ts">
import { VaDivider, VaBadge, VaCard, VaCardContent } from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import { ref } from "vue";
import { statusOf } from "@/composables/penalty";

const latestPenalty = ref(penaltyData.slice().reverse()[0]);
</script>

<template>
    <VaCard style="--va-card-padding: 0rem" class="rounded-xl">
        <VaCardContent>
            <div class="flex flex-col m-auto">
                <div class="flex flex-row justify-center">
                    <VaBadge text="NEW" color="#B3D943" class="mr-2" />
                    <div class="text-center text-xl">最新懲罰</div>
                </div>

                <VaDivider class="m-4" />

                <div
                    class="flex flex-col justify-center gap-3 mb-0"
                    item-responsive
                >
                    <div class="text-center text-base">
                        {{ latestPenalty.date }}
                        <span v-if="latestPenalty.status == '未生效'"
                            >抽出</span
                        >
                        <span v-else> 生效 </span>
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
        </VaCardContent>
    </VaCard>
</template>
