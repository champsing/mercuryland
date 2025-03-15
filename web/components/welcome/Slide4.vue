<script setup lang="ts">
import { ref } from "vue";
import { VaCarousel, VaDivider } from "vuestic-ui";
import NextPageButton from "./NextPageButton.vue";
import img1 from "@assets/images/welcome/s2_firework_1.png";
import img2 from "@assets/images/welcome/s2_firework_2.png";
import img3 from "@assets/images/welcome/s1_close_1.png";
import img4 from "@assets/images/welcome/s1_close_2.png";

const currentImg = ref(0);
const imgs = [
    { src: img1, alt: "煙火大會1" },
    { src: img2, alt: "煙火大會2" },
    { src: img3, alt: "第一季關服1" },
    { src: img4, alt: "第一季關服2" },
];
</script>

<template>
    <div class="h-screen overflow-hidden bg-black relative">
        <NextPageButton :page="4" />
        <div class="grid grid-cols-2 gap-20 absolute x-center w-5/6 h-full">
            <div class="relative mt-80">
                <div class="text-green-400 font-bold text-6xl">
                    來這裡遇到同好。
                </div>
                <VaDivider class="mt-6 mb-6" />
                <div class="font-bold text-green-600 text-3xl">
                    擁有豐富的擴充遊玩內容、玩家自辦活動、以及關服舉行的煙火大會。
                </div>
            </div>
            <div class="relative mt-48">
                <VaCarousel
                    v-model="currentImg"
                    :items="imgs"
                    :indicators="false"
                    :ratio="16 / 12"
                    autoscroll
                    stateful
                    infinite
                />
            </div>
        </div>
    </div>
</template>
