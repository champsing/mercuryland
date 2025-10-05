<script setup lang="ts">
import { computed } from "vue";
import { VaCard, VaCardTitle, VaCardContent } from "vuestic-ui";
import { formatHMS } from "@composables/utils.ts";

const props = defineProps<{ t: number }>();
const text = computed(() =>
    props.t >= 0 ? formatHMS(props.t) : "-" + formatHMS(props.t),
);
</script>

<template>
    <VaCard class="rounded-xl h-full">
        <VaCardTitle class="!text-xl justify-center"> 剩餘時間 </VaCardTitle>
        <VaCardContent class="text-5xl font-bold text-center mb-2">
            {{ text }}
        </VaCardContent>
    </VaCard>
</template>
