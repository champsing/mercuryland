<script setup lang="ts">
import { NGrid, NGi, NDivider } from "naive-ui";
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
    <div class="h-screen overflow-hidden bg-black relative">
        <NextPageButton :page="4" />
        <n-grid
            x-gap="0"
            :cols="2"
            item-responsive
            class="absolute x-center w-10/12 h-full"
        >
            <n-gi span="2 768:1" class="relative">
                <div class="absolute y-center x-center">
                    <div class="text-green-400 font-bold text-6xl">
                        來這裡遇到同好。
                    </div>
                    <n-divider />
                    <div class="font-bold text-green-600 text-3xl">
                        擁有豐富的擴充遊玩內容、玩家自辦活動、以及關服舉行的煙火大會。
                    </div>
                </div>
            </n-gi>
            <n-gi span="0 768:1" class="relative">
                <div class="absolute y-center x-center h-5/6 w-full">
                    <div class="h-1/2 relative">
                        <img
                            src="@assets/images/welcome/s2_firework_1.png"
                            class="absolute x-center y-center h-5/6 object-contain"
                            alt="Welcome"
                        />
                    </div>
                    <div class="h-1/2 relative">
                        <img
                            src="@assets/images/welcome/s2_firework_2.png"
                            class="absolute x-center y-center h-5/6 object-contain"
                            alt="Welcome"
                        />
                    </div>
                </div>
            </n-gi>
        </n-grid>
    </div>
</template>
