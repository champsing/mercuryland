<script setup lang="ts">
import { computed } from "vue";
import { UseElementBounding } from "@vueuse/components";
import { VaCard, VaCardTitle, VaCardContent } from "vuestic-ui";
import { formatHMS } from "@composables/utils.ts";

const props = defineProps<{ t: number }>();
const text = computed(() => (props.t >= 0 ? formatHMS(props.t) : "-" + formatHMS(props.t)));

function calcStyle(width: number) {
    let p = 24;
    let w = width - p * 2;
    let size = Math.floor(w / 5);
    return {
        "font-size": "" + size + "px",
    };
}
</script>

<template>
    <use-element-bounding v-slot="{ width }">
        <VaCard :style="calcStyle(width)" class="rounded-xl">
            <VaCardTitle style="font-size: 20px; justify-content: center;">
                剩餘時間
            </VaCardTitle>
            <VaCardContent class="font-bold text-center mb-2">
                {{ text }}
            </VaCardContent>
        </VaCard>
    </use-element-bounding>
</template>
