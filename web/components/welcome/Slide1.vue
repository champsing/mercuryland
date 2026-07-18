<script setup lang="ts">
import { VaDivider } from "vuestic-ui";
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
    <div class="h-[calc(100vh-48px)] overflow-hidden relative">
        <NextPageButton :page="1" />
        <div class="absolute x-center y-center text-center">
            <div class="text-white font-bold text-8xl">水星樂園</div>
            <VaDivider class="mt-6 mb-6 opacity-70" />
            <div class="text-white italic text-3xl">The Mercury Land</div>
        </div>
        <img
            src="/images/welcome/welcome.webp"
            alt="Welcome"
            class="-z-10 w-full h-full object-cover absolute top-0 left-0"
        />
    </div>
</template>
