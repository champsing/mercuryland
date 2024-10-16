<script setup lang="ts">
import NextPageButton from "./NextPageButton.vue";
import { VaDivider } from "vuestic-ui";
</script>

<template>
    <div class="h-screen overflow-hidden bg-neutral-800 relative">
        <NextPageButton :page="2" />
        <div class="absolute x-center top-40 text-center w-10/12 hidden md:block">
            <div class="text-neutral-400 text-3xl">
                YouTuber oreki惡靈的粉絲專屬生存Minecraft伺服器
            </div>
            <VaDivider class="mt-6" />
        </div>
        <div class="absolute x-center y-center text-center w-10/12">
            <div class="text-cyan-600 font-bold text-8xl">
                為粉絲們打造的
                <br />
                生存伺服器。
            </div>
        </div>
        <div class="absolute x-center bottom-40 text-center w-10/12 hidden md:block">
            <VaDivider class="mb-6" />
            <div class="text-neutral-400 text-3xl">
                只需加入Discord伺服器和正版Minecraft帳號。一切完全免費。
            </div>
        </div>
    </div>
</template>
