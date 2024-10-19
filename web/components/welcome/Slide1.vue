<script setup lang="ts">
import NextPageButton from "./NextPageButton.vue";
import { VaDivider } from "vuestic-ui";
</script>

<template>
    <div class="h-screen overflow-hidden relative">
        <NextPageButton :page="1" />
        <div class="absolute x-center y-center text-center">
            <div class="text-white font-bold text-8xl">水星伺服器</div>
            <VaDivider class="mt-6 mb-6 opacity-70" />
            <div class="text-white italic text-3xl">The Mercury Land</div>
        </div>
        <img
            src="@assets/images/welcome/welcome.png"
            alt="Welcome"
            class="-z-10 w-full h-full object-cover absolute top-0 left-0"
        />
    </div>
</template>