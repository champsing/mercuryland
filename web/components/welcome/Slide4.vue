<script setup lang="ts">
import { VaDivider } from "vuestic-ui";
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
    <div class="h-screen overflow-hidden bg-black relative">
        <NextPageButton :page="4" />
        <div
            class="grid grid-cols-3 grid-rows-4 gap-4 absolute x-center w-5/6 h-full"
        >
            <div class="relative row-start-2 mt-16">
                <div class="text-green-400 font-bold text-6xl">
                    來這裡遇到同好。
                </div>
                <VaDivider class="mt-6 mb-6" />
                <div class="font-bold text-green-600 text-3xl">
                    擁有豐富的擴充遊玩內容、玩家自辦活動、以及關服舉行的煙火大會。
                </div>
            </div>
            <div class="relative col-start-3 row-start-2">
                <img
                    src="@assets/images/welcome/s2_firework_1.png"
                    class="absolute"
                    alt="Welcome"
                />
            </div>
            <div class="relative col-start-3 row-start-3 mt-4">
                <img
                    src="@assets/images/welcome/s2_firework_2.png"
                    class="absolute"
                    alt="Welcome"
                />
            </div>
        </div>
    </div>
</template>
