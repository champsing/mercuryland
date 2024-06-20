<script setup lang="ts">
import { NGrid, NGi, NDivider } from "naive-ui";
import ReturnTopButton from "./ReturnTopButton.vue";
</script>

<template>
    <div class="h-screen overflow-hidden bg-lime-800 relative">
        <ReturnTopButton />
        <n-grid
            x-gap="0"
            :cols="2"
            item-responsive
            class="absolute x-center w-10/12 h-full"
        >
            <n-gi span="0 800:1" class="relative">
                <div class="absolute y-center x-center h-5/6 w-full">
                    <div class="h-1/2 relative">
                        <img
                            src="@assets/images/welcome/s1_close_1.png"
                            class="absolute x-center y-center h-5/6 object-contain"
                            alt="Welcome"
                        />
                    </div>
                    <div class="h-1/2 relative">
                        <img
                            src="@assets/images/welcome/s2_close_1.png"
                            class="absolute x-center y-center h-5/6 object-contain"
                            alt="Welcome"
                        />
                    </div>
                </div>
            </n-gi>
            <n-gi span="2 800:1" class="relative">
                <div class="absolute y-center x-center">
                    <div class="text-cyan-400 font-bold text-6xl">
                        還在猶豫什麼？
                    </div>
                    <n-divider />
                    <div class="text-cyan-600 font-bold text-3xl">
                        立即前往其他分頁，了解如何加入。
                    </div>
                </div>
            </n-gi>
        </n-grid>
    </div>
</template>
