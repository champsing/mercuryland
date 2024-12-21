<script setup lang="ts">
import { VaButton, VaDivider } from "vuestic-ui";
import { NGrid, NGi } from "naive-ui";
import ReturnTopButton from "./ReturnTopButton.vue";

// const emit = defineEmits<{
//     (e: "toTab", tab: string): void;
// }>();
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
            <n-gi span="0 768:1" class="relative">
                <div class="flex flex-col absolute y-center x-center h-5/6 w-full">
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
            <n-gi span="2 768:1" class="relative">
                <div class="flex flex-col absolute y-center x-center">
                    <div class="text-cyan-400 font-bold text-6xl">
                        還在猶豫什麼？
                    </div>
                    <VaDivider class="mt-6 mb-6" />
                    <div class="text-cyan-600 font-bold text-3xl">
                        立即前往其他分頁，了解如何加入。
                    </div>
                    <div class="mt-8">
                        <VaButton preset="primary" size="large" to="join" border-color="#0891b2" color="#0891b2">
                            <div class="text-cyan-200">
                                點擊加入
                            </div>
                        </VaButton>
                    </div>
                </div>
            </n-gi>
        </n-grid>
    </div>
</template>
