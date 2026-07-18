<script setup lang="ts">
document.title = "歡迎來到水星人的夢幻樂園";

import serverInfo from "@assets/data/server_info.json";
import { VaDivider } from "vuestic-ui";
import NextPageButton from "./NextPageButton.vue";

import SlideSection from "./SlideSection.vue";
import Slide1 from "./Slide1.vue";
import Slide2 from "./Slide2.vue";
import Slide3 from "./Slide3.vue";
import Slide4 from "./Slide4.vue";
import Slide5 from "./Slide5.vue";

import "@assets/styles/welcome-animations.css";
</script>

<template>
    <!-- Server Open: Snap-scroll container with 5 full-screen slides -->
    <div v-if="serverInfo.server_open" class="welcome-snap-container">
        <SlideSection class="welcome-snap-section"><Slide1 /></SlideSection>
        <SlideSection class="welcome-snap-section"><Slide2 /></SlideSection>
        <SlideSection class="welcome-snap-section"><Slide3 /></SlideSection>
        <SlideSection class="welcome-snap-section"><Slide4 /></SlideSection>
        <SlideSection class="welcome-snap-section"><Slide5 /></SlideSection>
    </div>

    <!-- Server Closed: Show simple welcome -->
    <div v-else class="relative overflow-hidden" style="height: calc(100vh - 48px)">
        <NextPageButton :page="1" />
        <div class="absolute x-center y-center text-center">
            <div class="text-white font-bold text-8xl">水星樂園</div>
            <VaDivider class="mt-6 mb-6 opacity-70" />
            <div class="text-white italic text-3xl">The Mercury Land</div>
        </div>
        <img
            src="/images/welcome/welcome.webp"
            alt="Welcome"
            class="-z-10 w-full h-full object-cover absolute top-0 left-0"
        />
    </div>
</template>

<style>
@tailwind utilities;
@layer utilities {
    .x-center {
        @apply left-1/2;
        @apply -translate-x-1/2;
    }

    .y-center {
        @apply top-1/2;
        @apply -translate-y-1/2;
    }
}
</style>
