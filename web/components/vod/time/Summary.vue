<script setup lang="ts">
import { formatHMS } from "@composables/vod";
import { Add24Filled } from "@vicons/fluent";
import { computed } from "vue";

const props = defineProps<{ t: number }>();
const emit = defineEmits<{ (e: "openCalculation"): void }>();

const text = computed(() =>
    props.t >= 0 ? formatHMS(props.t) : "-" + formatHMS(props.t),
);
</script>

<template>
    <button
        type="button"
        class="group flex w-full cursor-pointer items-center justify-between gap-3 rounded-2xl border border-white/[0.10] bg-zinc-300/12 px-4 py-2.5 text-left shadow-[0_4px_24px_rgba(0,0,0,0.25)] backdrop-blur-xl transition-colors hover:border-white/20 hover:bg-zinc-300/20 focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60 md:w-auto md:rounded-[2rem] md:border md:px-5 md:py-2"
        :aria-label="`剩餘時間 ${text}，查看計算明細`"
        title="查看時數計算明細"
        @click="emit('openCalculation')"
    >
        <span class="flex items-baseline gap-2 md:items-center md:gap-4">
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
        </span>

        <Add24Filled
            class="h-5 w-5 shrink-0 text-zinc-300 transition-colors group-hover:text-white md:h-7 md:w-7"
            aria-hidden="true"
        />
    </button>
</template>
