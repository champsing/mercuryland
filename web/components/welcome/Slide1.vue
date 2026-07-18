<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import NextPageButton from "./NextPageButton.vue";

const slideEl = ref<HTMLElement | null>(null);
const parallaxY = ref(0);
let raf = 0;
let scrollAncestor: HTMLElement | Window = window;

function findScrollAncestor(el: HTMLElement): HTMLElement | Window {
    let parent: HTMLElement | null = el.parentElement;
    while (parent) {
        const style = getComputedStyle(parent);
        if (/(auto|scroll)/.test(style.overflowY)) return parent;
        parent = parent.parentElement;
    }
    return window;
}

function onScroll() {
    cancelAnimationFrame(raf);
    raf = requestAnimationFrame(() => {
        if (!slideEl.value) return;
        // Use the slide's viewport position (works with any scroll container)
        parallaxY.value = -slideEl.value.getBoundingClientRect().top * 0.4;
    });
}

onMounted(() => {
    if (slideEl.value) {
        scrollAncestor = findScrollAncestor(slideEl.value);
    }
    scrollAncestor.addEventListener("scroll", onScroll, { passive: true });
    // initial call so parallax is correct before first scroll
    onScroll();
});

onUnmounted(() => {
    scrollAncestor.removeEventListener("scroll", onScroll);
    cancelAnimationFrame(raf);
});
</script>

<template>
    <div ref="slideEl" class="h-[calc(100vh-48px)] overflow-hidden relative">
        <NextPageButton :page="1" />

        <!-- Background image with parallax -->
        <img
            src="/images/welcome/welcome.webp"
            alt="Welcome"
            class="-z-10 w-full h-[120%] object-cover absolute top-0 left-0"
            :style="{ transform: `translateY(${parallaxY}px)` }"
        />
        <div class="absolute inset-0 bg-neutral-900/60" />

        <!-- Hero title: centered -->
        <div
            class="absolute x-center y-center text-center flex flex-col items-center gap-3"
        >
            <div
                class="anim-fade-up text-8xl font-black tracking-[0.12em] bg-gradient-to-r from-amber-200 via-yellow-300 to-amber-400 bg-clip-text text-transparent uppercase"
            >
                水星樂園
            </div>
            <div
                class="anim-fade-up anim-delay-200 text-white/90 font-bold italic text-4xl tracking-[0.25em] uppercase"
            >
                The Mercury Land
            </div>
        </div>

        <!-- Bottom text column: just above the NextPageButton -->
        <div
            class="absolute bottom-16 left-1/2 -translate-x-1/2 text-center flex flex-col items-center gap-2"
        >
            <div
                class="anim-fade-up anim-delay-400 text-amber-400/60 text-xs font-bold tracking-[0.4em] uppercase"
            >
                歡迎 · 欢迎 · Welcome · Bienvenue · ようこそ · 환영합니다
            </div>
        </div>
    </div>
</template>
