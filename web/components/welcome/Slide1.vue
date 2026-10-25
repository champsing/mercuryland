<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import NextPageButton from "./NextPageButton.vue";

const props = withDefaults(
    defineProps<{
        showScrollHint?: boolean;
    }>(),
    { showScrollHint: true },
);

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
    <div
        ref="slideEl"
        class="relative min-h-svh overflow-hidden md:min-h-0 md:h-[calc(100vh-48px)]"
    >
        <!-- Desktop -->
        <div class="hidden md:block">
            <NextPageButton v-if="props.showScrollHint" :page="1" />

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

        <!-- Mobile -->
        <div class="md:hidden absolute inset-0 overflow-hidden bg-[#05050a]">
            <!--
                The artwork is 16:9 with the pixel wordmark in the middle. Covering
                the whole portrait viewport would crop it down to ~26% of its width
                and cut the wordmark, so mobile uses a square crop of the centred
                artwork instead and lets the page background carry the rest.
            -->
            <img
                src="/images/welcome/welcome.webp"
                alt="Welcome"
                class="absolute left-0 top-0 w-full aspect-square object-cover"
            />
            <div
                class="absolute left-0 top-0 w-full aspect-square bg-neutral-900/40"
            />
            <div
                class="absolute left-0 top-0 w-full aspect-square bg-gradient-to-b from-transparent via-transparent to-[#05050a]"
            />
            <div
                class="relative flex h-full flex-col items-center justify-center px-6"
            >
                <div class="flex flex-col items-center gap-3 text-center">
                    <div
                        class="anim-fade-up text-4xl font-black tracking-[0.12em] bg-gradient-to-r from-amber-200 via-yellow-300 to-amber-400 bg-clip-text text-transparent uppercase"
                    >
                        水星樂園
                    </div>
                    <div
                        class="anim-fade-up anim-delay-200 text-white/90 font-bold italic text-xl tracking-[0.25em] uppercase"
                    >
                        The Mercury Land
                    </div>
                    <div
                        class="anim-fade-up anim-delay-400 text-amber-400/60 text-xs font-bold tracking-[0.4em] uppercase mt-4"
                    >
                        歡迎 · 欢迎 · Welcome · Bienvenue · ようこそ ·
                        환영합니다
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
