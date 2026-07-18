<script setup lang="ts">
import { ref } from "vue";
import { VaCarousel } from "vuestic-ui";
import NextPageButton from "./NextPageButton.vue";
import img1 from "/images/welcome/s2_firework_1.webp";
import img2 from "/images/welcome/s2_firework_2.webp";
import img3 from "/images/welcome/s1_close_1.webp";
import img4 from "/images/welcome/s1_close_2.webp";

const currentImg = ref(0);
const imgs = [
    { src: img1, alt: "煙火大會1" },
    { src: img2, alt: "煙火大會2" },
    { src: img3, alt: "第一季關服1" },
    { src: img4, alt: "第一季關服2" },
];
</script>

<template>
    <div class="h-[calc(100vh-48px)] overflow-hidden bg-neutral-900 relative">
        <NextPageButton :page="4" />

        <!-- Outer frame -->
        <div class="absolute inset-6 border border-white/15 hidden md:block" />
        <div class="absolute inset-8 border border-white/5 hidden md:block" />

        <!-- Corner accents -->
        <div class="absolute top-12 left-12 w-10 h-10 border-l-2 border-t-2 border-rose-400/50 hidden md:block" />
        <div class="absolute bottom-12 right-12 w-10 h-10 border-r-2 border-b-2 border-orange-400/50 hidden md:block" />

        <!-- Top label -->
        <div class="absolute top-20 left-28 text-rose-400/70 text-xs font-bold tracking-[0.4em] uppercase hidden md:block">
            Community · Events · Fireworks
        </div>

        <!-- Vertical accent bar - left -->
        <div class="absolute left-16 top-28 bottom-28 w-px bg-gradient-to-b from-transparent via-rose-400/30 to-transparent hidden md:block" />

        <!-- Main content - left side -->
        <div class="absolute left-28 y-center w-5/12 max-w-xl">
            <!-- Gradient title -->
            <div
                class="text-6xl font-black tracking-[0.1em] leading-tight bg-gradient-to-r from-pink-300 via-rose-400 to-orange-300 bg-clip-text text-transparent uppercase"
            >
                來這裡
            </div>
            <div
                class="text-7xl font-black tracking-[0.12em] leading-tight bg-gradient-to-r from-rose-300 via-pink-400 to-orange-300 bg-clip-text text-transparent uppercase mt-1"
            >
                遇到同好
            </div>

            <!-- Decorative rule -->
            <div class="flex items-center gap-5 mt-5 mb-7">
                <div class="w-2 h-2 rotate-45 bg-rose-400/50" />
                <div class="w-2 h-2 rotate-45 bg-pink-400/50" />
                <div class="h-px w-32 bg-gradient-to-r from-rose-400/50 to-transparent" />
            </div>

            <!-- Body -->
            <div class="text-neutral-300 text-lg font-light tracking-[0.08em] leading-relaxed">
                擁有豐富的擴充遊玩內容、<span class="text-rose-400 font-medium">玩家自辦活動</span>、以及關服舉行的<span class="text-orange-400 font-medium">煙火大會</span>。
            </div>
        </div>

        <!-- Image carousel - right side, framed -->
        <div class="absolute right-20 y-center w-5/12 max-w-lg hidden md:block">
            <div class="relative">
                <!-- Frame around carousel -->
                <div class="absolute -inset-3 border border-rose-400/20" />
                <div class="absolute -inset-4 border border-white/5" />
                <VaCarousel
                    v-model="currentImg"
                    :items="imgs"
                    :indicators="false"
                    :ratio="16 / 12"
                    autoscroll
                    stateful
                    infinite
                    class="shadow-2xl"
                />
                <!-- Caption -->
                <div class="absolute -bottom-8 right-0 text-rose-400/50 text-xs tracking-[0.2em] uppercase">
                    Season memories
                </div>
            </div>
        </div>

        <!-- Mobile carousel (hidden on desktop) -->
        <div class="absolute bottom-32 left-4 right-4 md:hidden opacity-60">
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

        <!-- Circle badge -->
        <div
            class="hidden md:flex flex-col items-center justify-center w-24 h-24 rounded-full border border-rose-400/30 absolute bottom-14 left-28"
        >
            <div class="text-rose-400/80 text-[0.6rem] font-bold tracking-[0.35em] uppercase">Events</div>
            <div class="text-rose-300 text-lg font-black mt-0.5">FIRE</div>
        </div>
    </div>
</template>
