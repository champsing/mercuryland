<script setup lang="ts">
import { availableHeight } from "@/composables/viewportHeight";
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "@vueuse/core";

const vh = useWindowSize().height;

function height(top: number) {
    return availableHeight({
        viewportHeight: vh.value,
        scrollY: window.scrollY,
        top,
    });
}
</script>

<template>
    <UseElementBounding v-slot="{ top }">
        <div :style="{ height: `${height(top)}px` }">
            <slot></slot>
        </div>
    </UseElementBounding>
</template>
