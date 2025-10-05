<script setup lang="ts">
import { VaCard, VaCardContent, VaCardTitle } from "vuestic-ui";
import { computed } from "vue";
import { stateString } from "@/composables/penalty";
import type { PenItem } from "@/composables/utils";

const props = defineProps<{ penalties: PenItem[] }>();

const latestPenalty = computed(() => props.penalties.slice().reverse()[0]);
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
                    :class="`inline text-penalty-state-${latestPenalty.state} font-bold text-center`"
                >
                    ● {{ stateString(latestPenalty.state) }}
                </div>

                <div class="text-center text-lg mb-3 line-clamp-3">
                    {{ latestPenalty.name }}
                </div>
            </div>
        </VaCardContent>
    </VaCard>
</template>
