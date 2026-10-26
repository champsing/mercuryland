<script setup lang="ts">
import { formatHMS } from "@composables/vod";
import { computed } from "vue";
import { VaIcon } from "vuestic-ui";

const props = defineProps<{ t: number }>();
const emit = defineEmits<{ (e: "openCalculation"): void }>();

const text = computed(() =>
    props.t >= 0 ? formatHMS(props.t) : "-" + formatHMS(props.t),
);
</script>

<template>
    <div
        class="flex w-full items-center justify-between gap-3 rounded-t-2xl border border-x-0 border-b-0 border-white/[0.10] bg-zinc-300/12 px-4 py-2.5 shadow-[0_4px_24px_rgba(0,0,0,0.25)] backdrop-blur-xl md:w-auto md:rounded-[2rem] md:border md:px-5 md:py-2"
    >
        <div class="flex items-baseline gap-2 md:items-center md:gap-4">
            <span
                class="text-xs tracking-widest text-zinc-200 uppercase md:text-sm"
            >
                剩餘時間
            </span>
            <span
                class="text-2xl font-bold leading-none tracking-tight text-zinc-100 tabular-nums md:text-4xl"
            >
                {{ text }}
            </span>
        </div>

        <div class="hidden h-5 w-px bg-white/10 md:block" aria-hidden="true" />

        <button
            class="inline-flex cursor-pointer items-center gap-1.5 whitespace-nowrap rounded-full border border-white/25 bg-zinc-400/15 px-3 py-1.5 text-xs text-zinc-200 backdrop-blur-sm transition-all duration-200 hover:border-white/40 hover:bg-zinc-400/25 hover:text-zinc-100 md:px-4"
            @click="emit('openCalculation')"
        >
            <VaIcon name="description" size="small" />
            計算明細
        </button>
    </div>
</template>
