<script setup lang="ts">
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "vue-window-size";

const vh = useWindowSize().height;
function calcStyle(top: number) {
    let height = Math.max(vh.value * 0.2, vh.value - window.scrollY - top);
    return {
        height: "" + height + "px",
    };
}
</script>

<template>
    <UseElementBounding v-slot="{ top }">
        <div :style="calcStyle(top)"></div>
    </UseElementBounding>
</template>
