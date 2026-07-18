<script setup lang="ts">
document.title = "歡迎來到水星人的夢幻樂園";

import serverInfo from "@assets/data/server_info.json";
import { onMounted, onUnmounted, ref } from "vue";
import { VaDivider } from "vuestic-ui";
import NextPageButton from "./NextPageButton.vue";

import Slide1 from "./Slide1.vue";
import Slide2 from "./Slide2.vue";
import Slide3 from "./Slide3.vue";
import Slide4 from "./Slide4.vue";
import Slide5 from "./Slide5.vue";
import SlideSection from "./SlideSection.vue";

import "@assets/styles/welcome-animations.css";

// ---- Snap-scroll: instant page switching via wheel interception ----
const TOTAL_SLIDES = 5;
const snapContainer = ref<HTMLElement | null>(null);
const currentSlide = ref(0);
let wheelAccum = 0;
let wheelTimer: ReturnType<typeof setTimeout> | null = null;

function slideHeight(): number {
    return window.innerHeight - 48;
}

function goToSlide(index: number) {
    if (!snapContainer.value) return;
    const clamped = Math.max(0, Math.min(TOTAL_SLIDES - 1, Math.round(index)));
    currentSlide.value = clamped;
    snapContainer.value.scrollTo({
        top: clamped * slideHeight(),
        behavior: "auto",
    });
}

// Sync currentSlide when scroll happens via other means (keyboard, touch, etc.)
function onContainerScroll() {
    if (!snapContainer.value) return;
    const idx = Math.round(snapContainer.value.scrollTop / slideHeight());
    currentSlide.value = Math.max(0, Math.min(TOTAL_SLIDES - 1, idx));
}

// Accumulate wheel delta over a 50 ms window, then snap one page in that direction.
// A short accumulation window makes it feel immediate while letting trackpad
// users complete a brief swipe before the snap fires.
function onWheel(e: WheelEvent) {
    // Only intercept vertical scroll
    if (Math.abs(e.deltaY) < Math.abs(e.deltaX)) return;

    e.preventDefault();
    wheelAccum += e.deltaY;

    if (wheelTimer) clearTimeout(wheelTimer);

    wheelTimer = setTimeout(() => {
        const dir = wheelAccum > 0 ? 1 : -1;
        goToSlide(currentSlide.value + dir);
        wheelAccum = 0;
        wheelTimer = null;
    }, 50);
}

onMounted(() => {
    snapContainer.value?.addEventListener("wheel", onWheel, { passive: false });
    snapContainer.value?.addEventListener("scroll", onContainerScroll, {
        passive: true,
    });
});

onUnmounted(() => {
    snapContainer.value?.removeEventListener("wheel", onWheel);
    snapContainer.value?.removeEventListener("scroll", onContainerScroll);
    if (wheelTimer) clearTimeout(wheelTimer);
});
</script>

<template>
    <!-- Server Open: Snap-scroll container with 5 full-screen slides -->
    <div
        v-if="serverInfo.online"
        ref="snapContainer"
        class="welcome-snap-container"
    >
        <SlideSection class="welcome-snap-section"><Slide1 /></SlideSection>
        <SlideSection class="welcome-snap-section"><Slide2 /></SlideSection>
        <SlideSection class="welcome-snap-section"><Slide3 /></SlideSection>
        <SlideSection class="welcome-snap-section"><Slide4 /></SlideSection>
        <SlideSection class="welcome-snap-section"><Slide5 /></SlideSection>
    </div>

    <!-- Server Closed: Show simple welcome -->
    <div
        v-else
        class="relative overflow-hidden"
        style="height: calc(100vh - 48px)"
    >
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
