<script setup lang="ts">
import { formatHMS } from "@composables/vod";
import { computed } from "vue";

const props = defineProps<{ t: number }>();
const emit = defineEmits<{ (e: "openCalculation"): void }>();

const text = computed(() =>
    props.t >= 0 ? formatHMS(props.t) : "-" + formatHMS(props.t),
);
</script>

<template>
    <div
        class="flex flex-col items-center gap-2 rounded-[2rem] border border-white/[0.10] bg-zinc-300/12 px-5 py-2.5 shadow-[0_4px_24px_rgba(0,0,0,0.25)] backdrop-blur-xl md:flex-row md:gap-3 md:py-2"
    >
        <div class="flex items-center gap-2 md:gap-4">
            <span
                class="text-xs md:text-sm tracking-widest text-zinc-200 uppercase"
            >
                剩餘時間
            </span>
            <span
                class="text-4xl font-bold tracking-tight text-zinc-100 tabular-nums"
            >
                {{ text }}
            </span>
        </div>

        <div class="h-px w-8 bg-white/10 md:h-5 md:w-px" aria-hidden="true" />

        <button
            class="whitespace-nowrap rounded-full border border-white/25 bg-zinc-400/15 px-4 py-1.5 text-xs text-zinc-200 backdrop-blur-sm transition-all duration-200 hover:border-white/40 hover:bg-zinc-400/25 hover:text-zinc-100 cursor-pointer"
            @click="emit('openCalculation')"
        >
            查看計算明細
        </button>
    </div>
</template>
