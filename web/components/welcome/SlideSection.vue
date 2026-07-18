<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { useIntersectionObserver } from "@vueuse/core";

const target = ref<HTMLElement | null>(null);
const isVisible = ref(false);
const rawProgress = ref(0);

// ---- IntersectionObserver: one-shot entrance trigger (staggered children) ----
useIntersectionObserver(
    target,
    ([{ isIntersecting }]) => {
        if (isIntersecting) isVisible.value = true;
    },
    { threshold: 0.08 },
);

// ---- Scroll-progress: continuous 0→1→0 as slide passes viewport center ----
// We find the nearest scrollable ancestor (the snap-container) and listen to
// its scroll events, then read position via getBoundingClientRect (viewport-
// relative, works regardless of which element scrolls).
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

function updateProgress() {
    if (!target.value) return;
    const rect = target.value.getBoundingClientRect();
    const viewH = window.innerHeight;
    const viewCenter = viewH / 2;
    const elCenter = rect.top + rect.height / 2;
    const dist = Math.abs(elCenter - viewCenter);
    const maxDist = viewH * 0.7;
    rawProgress.value = Math.max(0, 1 - dist / maxDist);
}

function onScroll() {
    cancelAnimationFrame(raf);
    raf = requestAnimationFrame(updateProgress);
}

onMounted(() => {
    if (target.value) {
        scrollAncestor = findScrollAncestor(target.value);
    }
    scrollAncestor.addEventListener("scroll", onScroll, { passive: true });
    updateProgress();
});

onUnmounted(() => {
    scrollAncestor.removeEventListener("scroll", onScroll);
    cancelAnimationFrame(raf);
});

// Derive a smooth opacity value from raw progress:
// 1.0 at center, plateaus for the middle 40 % of the zone, then fades out
const slideOpacity = computed(() => {
    const p = rawProgress.value;
    if (p >= 0.55) return 1;
    if (p <= 0.0) return 0;
    const t = p / 0.55;
    return t < 0.5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2;
});

const sectionStyle = computed(
    () =>
        ({
            "--slide-opacity": slideOpacity.value,
        }) as Record<string, string>,
);
</script>

<template>
    <section
        ref="target"
        :class="{ 'is-visible': isVisible }"
        :style="sectionStyle"
    >
        <div class="slide-blend" :style="{ opacity: slideOpacity }">
            <slot />
        </div>
    </section>
</template>

<style scoped>
.slide-blend {
    /* Fast CSS transition blends the opacity change when the slide switches,
       so the visual crossfade happens even when the scroll itself is instant. */
    transition: opacity 0.2s ease-out;
    will-change: opacity;
}
</style>
