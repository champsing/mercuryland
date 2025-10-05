<script setup lang="ts">
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "@vueuse/core";

export const FOOTNOTE_HEIGHT = 48;

const vh = useWindowSize().height;
function height(top: number) {
    let height = vh.value - window.scrollY - top - FOOTNOTE_HEIGHT;
    return height;
}
</script>

<template>
    <UseElementBounding v-slot="{ top }">
        <div :style="{ height: `${height(top)}px` }">
            <slot></slot>
        </div>
    </UseElementBounding>
</template>
