<script setup lang="ts">
import { VaButton } from "vuestic-ui";
import ReturnTopButton from "./ReturnTopButton.vue";
</script>

<template>
    <div class="h-[calc(100vh-48px)] overflow-hidden bg-neutral-900 relative">
        <ReturnTopButton />

        <!-- Outer frame -->
        <div
            class="anim-frame-in absolute inset-6 border border-white/15 hidden md:block"
        />
        <div
            class="anim-frame-in anim-delay-100 absolute inset-8 border border-white/5 hidden md:block"
        />

        <!-- Corner accents - all 4 corners -->
        <div
            class="anim-fade-up anim-delay-200 absolute top-12 left-12 w-10 h-10 border-l-2 border-t-2 border-cyan-400/50 hidden md:block"
        />
        <div
            class="anim-fade-up anim-delay-250 absolute top-12 right-12 w-10 h-10 border-r-2 border-t-2 border-cyan-400/50 hidden md:block"
        />
        <div
            class="anim-fade-up anim-delay-200 absolute bottom-12 left-12 w-10 h-10 border-l-2 border-b-2 border-cyan-400/50 hidden md:block"
        />
        <div
            class="anim-fade-up anim-delay-250 absolute bottom-12 right-12 w-10 h-10 border-r-2 border-b-2 border-cyan-400/50 hidden md:block"
        />

        <!-- Top label -->
        <div
            class="anim-fade-down absolute top-20 left-28 text-cyan-400/70 text-xs font-bold tracking-[0.4em] uppercase hidden md:block"
        >
            Your Journey Begins Now
        </div>

        <!-- Vertical accent bars -->
        <div
            class="anim-reveal-x absolute left-16 top-28 bottom-28 w-px bg-gradient-to-b from-transparent via-cyan-400/30 to-transparent hidden md:block"
        />
        <div
            class="anim-reveal-x anim-delay-200 absolute right-16 top-28 bottom-28 w-px bg-gradient-to-b from-transparent via-cyan-400/30 to-transparent hidden md:block"
        />

        <!-- Main content -->
        <div class="absolute x-center y-center text-center w-8/12 max-w-2xl">
            <!-- Hero question -->
            <div
                class="anim-scale-in text-7xl font-black tracking-[0.1em] bg-gradient-to-r from-cyan-200 via-teal-300 to-cyan-400 bg-clip-text text-transparent uppercase"
            >
                還在猶豫什麼？
            </div>

            <!-- Decorative rule -->
            <div
                class="anim-fade-up anim-delay-150 flex items-center justify-center gap-5 mt-6 mb-8"
            >
                <div
                    class="h-px w-20 bg-gradient-to-r from-transparent to-cyan-400/50"
                />
                <div
                    class="anim-scale-in anim-delay-250 w-2 h-2 rotate-45 bg-cyan-400/50"
                />
                <div
                    class="anim-scale-in anim-delay-300 w-2 h-2 rotate-45 bg-teal-400/50"
                />
                <div
                    class="anim-scale-in anim-delay-350 w-2 h-2 rotate-45 bg-cyan-400/50"
                />
                <div
                    class="h-px w-20 bg-gradient-to-l from-transparent to-cyan-400/50"
                />
            </div>

            <!-- Subtitle -->
            <div
                class="anim-fade-up anim-delay-250 text-white/80 font-thin text-2xl tracking-[0.12em] leading-relaxed"
            >
                立即前往水星伺服器專頁，了解如何加入。
            </div>

            <!-- CTA Button - styled like a poster tear-off strip -->
            <div class="anim-fade-up anim-delay-350 mt-12">
                <VaButton
                    preset="primary"
                    to="publication"
                    size="large"
                    border-color="#0891b2"
                    color="#0891b2"
                    class="!px-12 !py-4 !text-lg !tracking-[0.2em] !font-bold !rounded-none hover:!bg-cyan-600 transition-colors"
                >
                    <div class="text-cyan-100">點擊加入</div>
                </VaButton>
            </div>

            <!-- Tear-off strip effect -->
            <div
                class="anim-fade-up anim-delay-450 flex justify-center gap-2 mt-4"
            >
                <div class="w-3 h-3 rounded-full border border-cyan-400/20" />
                <div class="w-3 h-3 rounded-full border border-cyan-400/20" />
                <div class="w-3 h-3 rounded-full border border-cyan-400/20" />
                <div class="w-3 h-3 rounded-full border border-cyan-400/20" />
                <div class="w-3 h-3 rounded-full border border-cyan-400/20" />
                <div class="w-3 h-3 rounded-full border border-cyan-400/20" />
                <div class="w-3 h-3 rounded-full border border-cyan-400/20" />
            </div>
            <div
                class="anim-fade-up anim-delay-500 text-cyan-400/30 text-[0.6rem] tracking-[0.3em] uppercase mt-2"
            >
                ✂ tear here ✂
            </div>
        </div>

        <!-- Top-right accent badge -->
        <div
            class="anim-fade-up anim-delay-600 absolute top-14 right-12 flex items-center gap-3"
        >
            <div class="w-12 h-px bg-cyan-400/30" />
            <div class="text-cyan-400/50 text-xs tracking-[0.3em] uppercase">
                See you in-game
            </div>
            <div class="w-12 h-px bg-cyan-400/30" />
        </div>
    </div>
</template>
