<script setup lang="ts">
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
    <div
        class="relative min-h-svh overflow-hidden bg-neutral-900 md:min-h-0 md:h-[calc(100vh-48px)]"
    >
        <!-- Desktop -->
        <div class="hidden md:block">
            <NextPageButton :page="3" />

            <!-- Outer frame -->
            <div
                class="anim-frame-in absolute inset-6 border border-white/15"
            />
            <div
                class="anim-frame-in anim-delay-100 absolute inset-8 border border-white/5"
            />

            <!-- Corner accents -->
            <div
                class="anim-fade-up anim-delay-200 absolute top-12 left-12 w-10 h-10 border-l-2 border-t-2 border-yellow-400/50"
            />
            <div
                class="anim-fade-up anim-delay-200 absolute bottom-12 right-12 w-10 h-10 border-r-2 border-b-2 border-yellow-400/50"
            />

            <!-- Top label -->
            <div
                class="anim-fade-down absolute top-20 left-28 text-yellow-400/70 text-xs font-bold tracking-[0.4em] uppercase"
            >
                Creative Freedom · No Censorship
            </div>

            <!-- Vertical accent bar - left -->
            <div
                class="anim-reveal-x absolute left-16 top-28 bottom-28 w-px bg-gradient-to-b from-transparent via-yellow-400/30 to-transparent"
            />

            <!-- Statue of Liberty image - as a framed artwork -->
            <div
                class="anim-fade-up anim-delay-100 absolute left-[20%] bottom-16"
            >
                <div class="relative">
                    <div
                        class="absolute -inset-2 border border-yellow-400/20"
                    />
                    <img
                        src="/images/welcome/statue_of_liberty.webp"
                        class="h-[70vh] object-contain grayscale-[30%]"
                        alt="Statue of Liberty"
                    />
                </div>
            </div>

            <!-- Main content - right aligned -->
            <div class="absolute right-20 y-center w-5/12 max-w-xl text-right">
                <!-- Gradient title -->
                <div
                    class="anim-slide-right text-6xl font-black tracking-[0.1em] leading-tight bg-gradient-to-r from-yellow-200 via-amber-300 to-yellow-400 bg-clip-text text-transparent uppercase"
                >
                    發揮您的創意
                </div>

                <!-- Decorative rule -->
                <div
                    class="anim-fade-up anim-delay-100 flex items-center justify-end gap-5 mt-5 mb-7"
                >
                    <div
                        class="h-px w-24 bg-gradient-to-r from-transparent to-yellow-400/50"
                    />
                    <div
                        class="anim-scale-in anim-delay-200 w-2 h-2 rotate-45 bg-yellow-400/50"
                    />
                </div>

                <!-- Body -->
                <div
                    class="anim-slide-right anim-delay-200 text-white font-thin text-3xl tracking-[0.06em] leading-relaxed"
                >
                    建造您<span class="font-black text-yellow-400">夢想中</span
                    >的建築
                </div>

                <div
                    class="anim-slide-right anim-delay-300 mt-8 text-neutral-400 text-lg font-light tracking-[0.08em] leading-relaxed"
                >
                    絕無思想審查制度。復刻世界上的有名建築，或蓋出專屬於您的根據地。
                </div>
            </div>

            <!-- Circle badge -->
            <div
                class="anim-scale-in anim-delay-400 anim-float flex flex-col items-center justify-center w-24 h-24 rounded-full border border-yellow-400/30 absolute bottom-14 left-[8%]"
            >
                <div
                    class="text-yellow-400/80 text-[0.6rem] font-bold tracking-[0.35em] uppercase"
                >
                    100%
                </div>
                <div class="text-yellow-300 text-xl font-black mt-0.5">
                    FREE
                </div>
            </div>
        </div>

        <!-- Mobile -->
        <div
            class="md:hidden absolute inset-0 flex flex-col items-center justify-center gap-5 px-8 text-center"
        >
            <div
                class="anim-slide-right text-4xl font-black tracking-[0.1em] leading-tight bg-gradient-to-r from-yellow-200 via-amber-300 to-yellow-400 bg-clip-text text-transparent uppercase"
            >
                發揮您的創意
            </div>
            <img
                src="/images/welcome/statue_of_liberty.webp"
                class="anim-fade-up anim-delay-100 h-[30vh] object-contain opacity-80"
                alt="Statue of Liberty"
            />
            <div
                class="anim-fade-up anim-delay-200 text-white font-thin text-2xl tracking-[0.06em] leading-relaxed"
            >
                建造您<span class="font-black text-yellow-400">夢想中</span
                >的建築
            </div>
            <div
                class="anim-fade-up anim-delay-300 text-neutral-400 text-sm font-light tracking-[0.08em] leading-relaxed"
            >
                絕無思想審查制度。復刻世界上的有名建築，或蓋出專屬於您的根據地。
            </div>
        </div>
    </div>
</template>
