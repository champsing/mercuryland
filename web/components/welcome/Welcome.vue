<script setup lang="ts">
document.title = "歡迎來到水星人的夢幻樂園";

import serverInfo from "@assets/data/server_info.json";
import { onMounted, onUnmounted, ref } from "vue";

import Slide1 from "./Slide1.vue";
import Slide2 from "./Slide2.vue";
import Slide3 from "./Slide3.vue";
import Slide4 from "./Slide4.vue";
import Slide5 from "./Slide5.vue";
import SlideSection from "./SlideSection.vue";

import "@styles/welcome-animations.css";

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
    <template v-if="serverInfo.online">
        <!-- Desktop: snap-scroll, one slide per page -->
        <div ref="snapContainer" class="welcome-snap-container hidden md:block">
            <SlideSection class="welcome-snap-section"><Slide1 /></SlideSection>
            <SlideSection class="welcome-snap-section"><Slide2 /></SlideSection>
            <SlideSection class="welcome-snap-section"><Slide3 /></SlideSection>
            <SlideSection class="welcome-snap-section"><Slide4 /></SlideSection>
            <SlideSection class="welcome-snap-section"><Slide5 /></SlideSection>
        </div>

        <!-- Mobile: normal stacked scroll, same five slides -->
        <div class="md:hidden">
            <SlideSection :fade="false"><Slide1 /></SlideSection>
            <SlideSection :fade="false"><Slide2 /></SlideSection>
            <SlideSection :fade="false"><Slide3 /></SlideSection>
            <SlideSection :fade="false"><Slide4 /></SlideSection>
            <SlideSection :fade="false"><Slide5 /></SlideSection>
        </div>
    </template>

    <!-- Server Closed: reuse the redesigned Slide 1 hero so it stays in sync -->
    <SlideSection v-else :fade="false">
        <Slide1 :show-scroll-hint="false" />
    </SlideSection>
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
