<script setup lang="ts">
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
    <div class="h-[calc(100vh-48px)] overflow-hidden bg-neutral-900 relative">
        <NextPageButton :page="3" />

        <!-- Outer frame -->
        <div class="absolute inset-6 border border-white/15 hidden md:block" />
        <div class="absolute inset-8 border border-white/5 hidden md:block" />

        <!-- Corner accents -->
        <div
            class="absolute top-12 left-12 w-10 h-10 border-l-2 border-t-2 border-yellow-400/50 hidden md:block"
        />
        <div
            class="absolute bottom-12 right-12 w-10 h-10 border-r-2 border-b-2 border-yellow-400/50 hidden md:block"
        />

        <!-- Top label -->
        <div
            class="absolute top-20 left-28 text-yellow-400/70 text-xs font-bold tracking-[0.4em] uppercase hidden md:block"
        >
            Creative Freedom · No Censorship
        </div>

        <!-- Vertical accent bar - left -->
        <div
            class="absolute left-16 top-28 bottom-28 w-px bg-gradient-to-b from-transparent via-yellow-400/30 to-transparent hidden md:block"
        />

        <!-- Statue of Liberty image - as a framed artwork -->
        <div class="absolute left-[20%] bottom-16 hidden md:block">
            <div class="relative">
                <div class="absolute -inset-2 border border-yellow-400/20" />
                <img
                    src="/images/welcome/statue_of_liberty.webp"
                    class="h-[70vh] object-contain grayscale-[30%]"
                    alt="Statue of Liberty"
                />
            </div>
        </div>

        <!-- Mobile image (hidden on desktop) -->
        <div class="absolute top-20 left-1/2 -translate-x-1/2 md:hidden">
            <img
                src="/images/welcome/statue_of_liberty.webp"
                class="h-[30vh] object-contain opacity-40"
                alt="Statue of Liberty"
            />
        </div>

        <!-- Main content - right aligned -->
        <div class="absolute right-20 y-center w-5/12 max-w-xl text-right">
            <!-- Gradient title -->
            <div
                class="text-6xl font-black tracking-[0.1em] leading-tight bg-gradient-to-r from-yellow-200 via-amber-300 to-yellow-400 bg-clip-text text-transparent uppercase"
            >
                發揮您的創意
            </div>

            <!-- Decorative rule -->
            <div class="flex items-center justify-end gap-5 mt-5 mb-7">
                <div
                    class="h-px w-24 bg-gradient-to-r from-transparent to-yellow-400/50"
                />
                <div class="w-2 h-2 rotate-45 bg-yellow-400/50" />
            </div>

            <!-- Body -->
            <div
                class="text-white font-thin text-3xl tracking-[0.06em] leading-relaxed"
            >
                建造您<span class="font-black text-yellow-400">夢想中</span
                >的建築
            </div>

            <div
                class="mt-8 text-neutral-400 text-lg font-light tracking-[0.08em] leading-relaxed"
            >
                絕無思想審查制度。復刻世界上的有名建築，或蓋出專屬於您的根據地。
            </div>
        </div>

        <!-- Circle badge -->
        <div
            class="hidden md:flex flex-col items-center justify-center w-24 h-24 rounded-full border border-yellow-400/30 absolute bottom-14 left-[8%]"
        >
            <div
                class="text-yellow-400/80 text-[0.6rem] font-bold tracking-[0.35em] uppercase"
            >
                100%
            </div>
            <div class="text-yellow-300 text-xl font-black mt-0.5">FREE</div>
        </div>
    </div>
</template>
