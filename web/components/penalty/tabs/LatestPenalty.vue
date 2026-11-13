<script setup lang="ts">
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { computed, ref } from "vue";
import { VaChip, VaIcon } from "vuestic-ui";

const props = defineProps<{ penalties: PenItem[] }>();

// 手機上主文預設收合成一行，可用箭頭展開看全文。
const nameExpanded = ref(false);

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
            <div class="flex w-full min-w-0 items-start gap-1 md:contents">
                <p
                    id="latest-penalty-name"
                    class="m-0 min-w-0 flex-1 text-sm font-bold text-[#f7f7f8] md:flex-1"
                    :class="nameExpanded ? 'break-words' : 'truncate'"
                    :title="latestPenalty.name"
                >
                    {{ latestPenalty.name }}
                </p>
                <button
                    type="button"
                    class="inline-flex h-6 w-6 shrink-0 cursor-pointer items-center justify-center rounded-full text-[#aeb7c7] transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60 md:hidden"
                    :aria-expanded="nameExpanded"
                    aria-controls="latest-penalty-name"
                    :aria-label="nameExpanded ? '收合最新懲罰' : '展開最新懲罰'"
                    :title="nameExpanded ? '收合' : '展開全文'"
                    @click="nameExpanded = !nameExpanded"
                >
                    <VaIcon
                        :name="nameExpanded ? 'expand_less' : 'expand_more'"
                        size="small"
                    />
                </button>
            </div>
        </template>
        <span v-else class="text-sm text-[#8f98a8]">暫無懲罰</span>
    </section>
</template>
