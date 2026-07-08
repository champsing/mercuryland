<script setup lang="ts">
import { PenItem, stateColor, stateString } from "@/composables/penalty";
import { computed } from "vue";
import { VaCard, VaCardContent, VaChip, VaIcon } from "vuestic-ui";

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
    <VaCard
        class="overflow-hidden rounded-lg border border-[rgba(255,255,255,0.08)] !bg-[linear-gradient(135deg,rgba(229,9,20,0.14),transparent_42%),rgba(18,21,27,0.92)]"
        style="--va-card-padding: 0"
    >
        <VaCardContent class="flex flex-col gap-4 !p-4">
            <div class="flex items-center justify-between gap-3">
                <div>
                    <span
                        class="text-[0.72rem] font-extrabold uppercase tracking-normal text-[#ff6978]"
                    >
                        Latest penalty
                    </span>
                    <h2
                        class="mt-[0.1rem] text-base font-extrabold tracking-normal text-[#f7f7f8]"
                    >
                        最新懲罰
                    </h2>
                </div>
                <div
                    class="grid h-11 w-11 place-items-center rounded-lg bg-[rgba(229,9,20,0.14)] text-[#ff6978]"
                    aria-hidden="true"
                >
                    <VaIcon name="campaign" size="large" />
                </div>
            </div>

            <div v-if="latestPenalty" class="block">
                <div
                    class="flex items-center justify-between gap-3 text-[0.82rem] text-[#aeb7c7]"
                >
                    <div class="inline-flex min-w-0 items-center gap-[0.45rem]">
                        <strong
                            class="text-[0.9rem] font-black leading-none text-[#f7f7f8]"
                        >
                            #{{ latestPenalty.id }}
                        </strong>
                        <span>{{ latestPenalty.date }}</span>
                        <VaChip
                            readonly
                            outline
                            size="small"
                            :color="stateColor(latestPenalty.state, 'raw')"
                            class="min-w-[5rem] justify-center"
                        >
                            ● {{ stateString(latestPenalty.state) }}
                        </VaChip>
                        <p
                            class="ml-2 line-clamp-4 text-[1.12rem] font-extrabold leading-[1.45] text-[#f7f7f8]"
                        >
                            {{ latestPenalty.name }}
                        </p>
                    </div>
                </div>
            </div>
            <div
                v-else
                class="grid flex-1 place-items-center gap-2 text-center text-[#8f98a8]"
            >
                <VaIcon name="inbox" size="large" />
                <span>暫無懲罰</span>
            </div>
        </VaCardContent>
    </VaCard>
</template>
