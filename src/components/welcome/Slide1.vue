<script setup lang="ts">
import NextPageButton from "./NextPageButton.vue";
import { NDivider } from "naive-ui";
import { ccMix } from "@/composables/utils";
</script>

<template>
    <div class="h-screen overflow-hidden relative">
        <NextPageButton :page="1" />
        <div class="absolute x-center y-center text-center">
            <div class="text-white font-bold text-8xl mt-5">{{ ccMix('水星伺服器') }}</div>
            <n-divider />
            <div class="text-white italic text-3xl">The Mercury Land</div>
        </div>
        <img
            src="@assets/images/welcome/welcome.png"
            alt="Welcome"
            class="-z-10 w-full h-full object-cover absolute top-0 left-0"
        />
    </div>
</template>
