<script setup lang="ts">
import serverInfo from "@assets/data/server_info.json";
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
    <div class="h-[calc(100vh-48px)] overflow-hidden bg-neutral-900 relative">
        <NextPageButton :page="2" />

        <!-- Outer frame -->
        <div
            class="anim-frame-in absolute inset-6 border border-white/15 hidden md:block"
        />
        <div
            class="anim-frame-in anim-delay-100 absolute inset-8 border border-white/5 hidden md:block"
        />

        <!-- Corner accents -->
        <div
            class="anim-fade-up anim-delay-200 absolute top-12 left-12 w-10 h-10 border-l-2 border-t-2 border-emerald-400/50 hidden md:block"
        />
        <div
            class="anim-fade-up anim-delay-200 absolute bottom-12 right-12 w-10 h-10 border-r-2 border-b-2 border-cyan-400/50 hidden md:block"
        />

        <!-- Vertical accent bar - left -->
        <div
            class="anim-reveal-x absolute left-16 top-28 bottom-28 w-px bg-gradient-to-b from-transparent via-emerald-400/30 to-transparent hidden md:block"
        />

        <!-- Top label -->
        <div
            class="anim-fade-down absolute top-20 left-28 text-emerald-400/70 text-xs font-bold tracking-[0.4em] uppercase hidden md:block"
        >
            Minecraft Server · Season {{ serverInfo.season }}
        </div>

        <!-- Main content block -->
        <div class="absolute x-center y-center w-9/12 max-w-3xl">
            <!-- Title with gradient -->
            <div
                class="anim-slide-left text-7xl font-black tracking-[0.15em] bg-gradient-to-r from-emerald-300 via-teal-400 to-cyan-300 bg-clip-text text-transparent uppercase"
            >
                水星伺服器
            </div>

            <!-- Decorative rule -->
            <div
                class="anim-fade-up anim-delay-200 flex items-center gap-5 mt-5 mb-7"
            >
                <div
                    class="h-px flex-1 bg-gradient-to-r from-transparent via-emerald-400/50 to-emerald-400/50"
                />
                <div
                    class="anim-scale-in anim-delay-300 w-2 h-2 rotate-45 bg-cyan-400/50"
                />
                <div
                    class="anim-scale-in anim-delay-350 w-2 h-2 rotate-45 bg-emerald-400/50"
                />
                <div
                    class="anim-scale-in anim-delay-400 w-2 h-2 rotate-45 bg-teal-400/50"
                />
                <div
                    class="h-px flex-1 bg-gradient-to-l from-transparent via-cyan-400/50 to-cyan-400/50"
                />
            </div>

            <!-- Headline -->
            <div
                class="anim-fade-up anim-delay-300 text-white font-thin text-5xl leading-tight tracking-[0.08em]"
            >
                為粉絲們打造的
            </div>
            <div
                class="anim-fade-up anim-delay-400 text-white font-black text-8xl leading-tight tracking-[0.06em] mt-1"
            >
                生存伺服器
            </div>
        </div>

        <!-- Bottom info strip -->
        <div
            class="absolute bottom-14 left-28 right-20 flex items-end justify-between"
        >
            <div class="space-y-2">
                <div
                    class="anim-fade-up anim-delay-500 text-neutral-300 text-lg font-light tracking-[0.1em] leading-relaxed"
                >
                    YouTuber oreki惡靈的粉絲專屬生存Minecraft伺服器
                </div>
                <div
                    class="anim-fade-up anim-delay-600 text-neutral-500 text-base font-light tracking-[0.08em] leading-relaxed"
                >
                    只需加入Discord伺服器和正版Minecraft帳號。
                    <span class="text-cyan-400 font-medium"
                        >一切完全免費。</span
                    >
                </div>
            </div>

            <!-- Circle badge -->
            <div
                class="anim-scale-in anim-delay-300 anim-float hidden md:flex flex-col items-center justify-center w-24 h-24 rounded-full border border-emerald-400/30 flex-shrink-0 ml-12"
            >
                <div
                    class="text-emerald-400/80 text-[0.6rem] font-bold tracking-[0.35em] uppercase"
                >
                    Free
                </div>
                <div class="text-emerald-300 text-xl font-black mt-0.5">
                    JOIN
                </div>
            </div>
        </div>
    </div>
</template>
