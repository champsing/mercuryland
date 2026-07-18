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

        <!-- Outer frame -->
        <div class="absolute inset-6 border border-white/20 hidden md:block" />
        <div class="absolute inset-8 border border-white/8 hidden md:block" />

        <!-- Corner accents -->
        <div class="absolute top-12 left-12 w-10 h-10 border-l-2 border-t-2 border-amber-400/60 hidden md:block" />
        <div class="absolute bottom-12 right-12 w-10 h-10 border-r-2 border-b-2 border-amber-400/60 hidden md:block" />

        <!-- Top label -->
        <div class="absolute top-20 left-28 text-amber-400/80 text-xs font-bold tracking-[0.4em] uppercase hidden md:block">
            Welcome · Bienvenue · ようこそ
        </div>

        <!-- Main content -->
        <div class="absolute x-center y-center text-center w-10/12 max-w-3xl">
            <!-- Hero title -->
            <div
                class="text-8xl font-black tracking-[0.12em] bg-gradient-to-r from-amber-200 via-yellow-300 to-amber-400 bg-clip-text text-transparent uppercase"
            >
                水星樂園
            </div>

            <!-- Decorative rule -->
            <div class="flex items-center gap-5 mt-6 mb-8">
                <div class="h-px flex-1 bg-gradient-to-r from-transparent via-amber-400/50 to-amber-400/50" />
                <div class="w-2 h-2 rotate-45 bg-amber-400/60" />
                <div class="w-1.5 h-1.5 rotate-45 bg-yellow-300/60" />
                <div class="w-2 h-2 rotate-45 bg-amber-400/60" />
                <div class="h-px flex-1 bg-gradient-to-l from-transparent via-amber-400/50 to-amber-400/50" />
            </div>

            <!-- Subtitle -->
            <div class="text-white/90 font-thin text-4xl tracking-[0.25em] uppercase">
                The Mercury Land
            </div>
        </div>

        <!-- Bottom accent -->
        <div class="absolute bottom-14 left-1/2 -translate-x-1/2 flex items-center gap-3">
            <div class="w-16 h-px bg-amber-400/40" />
            <div class="text-amber-400/60 text-[0.6rem] font-bold tracking-[0.35em] uppercase">Scroll to explore</div>
            <div class="w-16 h-px bg-amber-400/40" />
        </div>
    </div>
</template>
