<script setup lang="ts">
import Slide1 from "./Slide1.vue";
import Slide2 from "./Slide2.vue";
import Slide3 from "./Slide3.vue";
import Slide4 from "./Slide4.vue";
import Slide5 from "./Slide5.vue";

const emit = defineEmits<{
    (e: "toTab", tab: string): void;
}>();
</script>

<template>
    <div ref="slide">
        <Slide1 />
        <Slide2 />
        <Slide3 />
        <Slide4 />
        <Slide5 />
    </div>
</template>

<style>
@tailwind utilities;
@layer utilities {
    .x-center {
        @apply left-1/2;
        @apply -translate-x-1/2;
    }

    .y-center {
        @apply top-1/2;
        @apply -translate-y-1/2;
    }
}
</style>
