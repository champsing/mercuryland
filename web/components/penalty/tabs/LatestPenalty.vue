<script setup lang="ts">
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { computed } from "vue";
import { VaChip, VaIcon } from "vuestic-ui";

const props = defineProps<{ penalties: PenItem[] }>();

const latestPenalty = computed(() =>
    props.penalties.length > 0
        ? props.penalties
              .slice()
              .sort(
                  (lhs, rhs) =>
                      rhs.date.localeCompare(lhs.date) || rhs.id - lhs.id,
              )[0]
        : null,
);
</script>

<template>
    <section
        aria-labelledby="latest-penalty-heading"
        class="flex min-w-0 flex-wrap items-center gap-x-3 gap-y-1 rounded-xl border border-white/[0.08] bg-[linear-gradient(90deg,rgba(229,9,20,0.12),rgba(18,21,27,0.9)_38%)] px-3 py-2 shadow-[inset_2px_0_0_#e50914] backdrop-blur-sm"
    >
        <h2
            id="latest-penalty-heading"
            class="inline-flex shrink-0 items-center gap-1.5 text-[0.72rem] font-extrabold tracking-wide text-[#ff6978]"
        >
            <VaIcon name="campaign" size="small" />
            最新懲罰
        </h2>

        <template v-if="latestPenalty">
            <strong
                class="shrink-0 text-sm font-black leading-none text-[#f7f7f8] tabular-nums"
            >
                #{{ latestPenalty.id }}
            </strong>
            <span
                class="hidden shrink-0 text-xs font-medium text-[#aeb7c7] tabular-nums sm:inline"
            >
                {{ latestPenalty.date }}
            </span>
            <VaChip
                readonly
                outline
                size="small"
                :color="stateColor(latestPenalty.state, 'raw')"
                class="shrink-0 min-w-[4.5rem] justify-center"
            >
                ● {{ stateString(latestPenalty.state) }}
            </VaChip>
            <p
                class="m-0 w-full min-w-0 truncate text-sm font-bold text-[#f7f7f8] md:w-auto md:flex-1"
                :title="latestPenalty.name"
            >
                {{ latestPenalty.name }}
            </p>
        </template>
        <span v-else class="text-sm text-[#8f98a8]">暫無懲罰</span>
    </section>
</template>
