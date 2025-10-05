<script setup lang="ts">
import { VaCard, VaCardContent, VaCardTitle, VaChip } from "vuestic-ui";
import { computed } from "vue";
import { stateString } from "@/composables/penalty";
import { stateColor, PenItem } from "@/composables/penalty";

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
                class="flex flex-col justify-center items-center gap-2 mb-2"
                item-responsive
            >
                <VaChip
                    readonly
                    outline
                    size="small"
                    :color="stateColor(latestPenalty.state, 'raw')"
                    class="w-24"
                >
                    ● {{ stateString(latestPenalty.state) }}
                </VaChip>

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
