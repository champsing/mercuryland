<script setup lang="ts">
import { VaCard, VaCardContent, VaCardTitle } from "vuestic-ui";
import { computed } from "vue";
import { stateString } from "@/composables/penalty";
import type { PenItem } from "@/composables/utils";
import { stateColor } from "@/composables/penalty";

const props = defineProps<{ penalties: PenItem[] }>();

const latestPenalty = computed(() =>
    props.penalties.length > 0 ? props.penalties.slice().reverse()[0] : null,
);
</script>

<template>
    <VaCard
        style="--va-card-padding: 1rem"
        class="rounded-xl h-full flex flex-col"
    >
        <VaCardTitle class="!text-xl justify-center"> 最新 </VaCardTitle>
        <VaCardContent>
            <div
                v-if="latestPenalty"
                class="flex flex-col justify-center gap-2 mb-2"
                item-responsive
            >
                <div
                    :class="`inline ${stateColor(latestPenalty.state, 'text')} font-bold text-center`"
                >
                    ● {{ stateString(latestPenalty.state) }}
                </div>

                <div class="text-center text-lg mb-3 line-clamp-3">
                    {{ latestPenalty.name }}
                </div>
            </div>
            <div
                v-else
                class="flex justify-center items-center h-full text-center text-gray-500"
            >
                暫無懲罰
            </div>
        </VaCardContent>
    </VaCard>
</template>
