<script setup lang="ts">
</script>

<template>
    <div class="z-10 absolute bottom-6 left-1/2 -translate-x-1/2 text-white/30">
        <div class="anim-scroll-hint flex items-center gap-3">
            <svg
                class="w-5 h-5"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <rect x="5" y="2" width="14" height="17" rx="4" />
                <path d="M12 7v5" />
                <path d="M9 10l3 3 3-3" />
            </svg>
            <span class="text-xs tracking-[0.2em] uppercase">Scroll</span>
        </div>
    </div>
</template>
