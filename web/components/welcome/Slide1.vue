<script setup lang="ts">
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
    <div class="h-[calc(100vh-48px)] overflow-hidden relative">
        <NextPageButton :page="1" />

        <!-- Background image with dark overlay -->
        <img
            src="/images/welcome/welcome.webp"
            alt="Welcome"
            class="-z-10 w-full h-full object-cover absolute top-0 left-0"
        />
        <div class="absolute inset-0 bg-neutral-900/60" />

        <!-- Hero title: centered -->
        <div
            class="absolute x-center y-center text-center flex flex-col items-center gap-3"
        >
            <div
                class="text-8xl font-black tracking-[0.12em] bg-gradient-to-r from-amber-200 via-yellow-300 to-amber-400 bg-clip-text text-transparent uppercase"
            >
                水星樂園
            </div>
            <div
                class="text-white/90 font-thin text-4xl tracking-[0.25em] uppercase"
            >
                The Mercury Land
            </div>
        </div>

        <!-- Bottom text column: just above the NextPageButton -->
        <div
            class="absolute bottom-16 left-1/2 -translate-x-1/2 text-center flex flex-col items-center gap-2"
        >
            <div
                class="text-amber-400/60 text-xs font-bold tracking-[0.4em] uppercase"
            >
                Welcome · Bienvenue · ようこそ
            </div>
        </div>
    </div>
</template>
