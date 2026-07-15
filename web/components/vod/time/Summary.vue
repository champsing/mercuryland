<script setup lang="ts">
import { computed } from "vue";
import { formatHMS } from "@composables/vod";

const props = defineProps<{ t: number }>();
const emit = defineEmits<{ (e: "openCalculation"): void }>();

const text = computed(() =>
    props.t >= 0 ? formatHMS(props.t) : "-" + formatHMS(props.t),
);
</script>

<template>
    <div
        class="inline-flex flex-row items-center gap-3 rounded-[2rem] border border-white/[0.10] bg-zinc-300/12 backdrop-blur-xl px-5 py-2 shadow-[0_4px_24px_rgba(0,0,0,0.25)]"
    >
        <div class="flex items-baseline gap-2">
            <span
                class="text-[10px] tracking-widest text-zinc-500 uppercase md:text-xs"
            >
                剩餘時間
            </span>
            <span
                class="text-xl font-bold tracking-tight text-zinc-100 tabular-nums md:text-2xl"
            >
                {{ text }}
            </span>
        </div>

        <div class="h-5 w-px bg-white/10" aria-hidden="true" />

        <button
            class="whitespace-nowrap rounded-full border border-white/[0.08] bg-zinc-400/12 px-3 py-1 text-[11px] text-zinc-400 backdrop-blur-sm transition-all duration-200 hover:border-white/20 hover:bg-zinc-400/20 hover:text-zinc-200 md:px-4 md:py-1.5 md:text-xs cursor-pointer"
            @click="emit('openCalculation')"
        >
            查看計算明細
        </button>
    </div>
</template>
