<script setup lang="ts">
import { VaButton, VaDivider } from "vuestic-ui";
import ReturnTopButton from "./ReturnTopButton.vue";
</script>

<template>
    <div class="h-screen overflow-hidden bg-emerald-800 relative">
        <ReturnTopButton />
        <div class="flex flex-col absolute y-center x-center text-center">
            <div class="text-cyan-400 font-bold text-6xl">還在猶豫什麼？</div>
            <VaDivider class="mt-6 mb-6" />
            <div class="text-cyan-600 font-bold text-3xl">
                立即前往其他分頁，了解如何加入。
            </div>
            <div class="mt-8">
                <VaButton
                    preset="primary"
                    to="join"
                    size="large"
                    border-color="#0891b2"
                    color="#0891b2"
                >
                    <div class="text-cyan-200">點擊加入</div>
                </VaButton>
            </div>
        </div>
    </div>
</template>
