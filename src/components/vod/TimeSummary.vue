<script setup lang="ts">
import { computed } from "vue";
import { UseElementBounding } from "@vueuse/components";
import { NCard } from "naive-ui";
import { formatHMS } from "@composables/utils.ts";

const props = defineProps<{ t: number }>();
const text = computed(() => (props.t > 0 ? formatHMS(props.t) : formatHMS(0)));

function calcStyle(width: number) {
    let p = 24;
    let w = width - p * 2;
    let size = Math.floor(w / 5);
    return {
        "--n-font-size": "" + size + "px",
    };
}
</script>

<template>
    <UseElementBounding v-slot="{ width }">
        <n-card
            title="剩余时间"
            class="font-bold text-center"
            :style="calcStyle(width)"
        >
            {{ text }}
        </n-card>
    </UseElementBounding>
</template>
