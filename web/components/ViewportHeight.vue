<script setup lang="ts">
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "@vueuse/core";

const FOOTNOTE_HEIGHT = 48;
const PADDING_BOTTOM = 4;
const vh = useWindowSize().height;

function height(top: number) {
    let height =
        vh.value - window.scrollY - top - FOOTNOTE_HEIGHT - PADDING_BOTTOM;
    return height;
}
</script>

<template>
    <UseElementBounding v-slot="{ top }">
        <div
            :style="{
                height: `${height(top)}px`,
                paddingBottom: `${PADDING_BOTTOM}px`,
            }"
        >
            <slot></slot>
        </div>
    </UseElementBounding>
</template>
