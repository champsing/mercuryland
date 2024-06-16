<script setup lang="ts">
import NextPageButton from "./NextPageButton.vue";
import { NDivider } from "naive-ui";
</script>

<template>
    <div class="h-screen overflow-hidden bg-neutral-800 relative">
        <NextPageButton :page="2" />
        <div
            class="absolute top-32 -translate-y-1/2 w-full text-center pl-64 pr-64"
        >
            <div class="text-3xl text-neutral-400">
                YouTuber oreki惡靈 的粉絲專屬生存Minecraft伺服器
            </div>
            <n-divider />
        </div>
        <div class="absolute bottom-1/2 w-full text-center pl-64 pr-64">
            <div class="text-cyan-600 font-bold text-8xl">
                為粉絲打造的生存伺服器。
            </div>
        </div>
        <div class="absolute bottom-32 w-full text-center pl-64 pr-64">
            <n-divider />
            <div class="text-neutral-400 text-3xl">
                只需加入Discord伺服器和正版Minecraft帳號。一切完全免費。
            </div>
        </div>
    </div>
</template>
