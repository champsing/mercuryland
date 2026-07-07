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
    <VaCard class="latest-card">
        <VaCardContent class="latest-card__content">
            <div class="latest-card__header">
                <div>
                    <span class="latest-card__eyebrow">Latest release</span>
                    <h2>最新發布</h2>
                </div>
                <div class="latest-card__icon" aria-hidden="true">
                    <VaIcon name="campaign" size="large" />
                </div>
            </div>

            <div v-if="latestPenalty" class="latest-release">
                <div class="latest-release__meta">
                    <div class="latest-release__identity">
                        <strong class="latest-release__id"
                            >#{{ latestPenalty.id }}</strong
                        >
                        <span>{{ latestPenalty.date }}</span>
                        <VaChip
                            readonly
                            outline
                            size="small"
                            :color="stateColor(latestPenalty.state, 'raw')"
                            class="latest-release__chip"
                        >
                            ● {{ stateString(latestPenalty.state) }}
                        </VaChip>
                    </div>
                </div>

                <p class="latest-release__title">
                    {{ latestPenalty.name }}
                </p>
            </div>
            <div v-else class="latest-empty">
                <VaIcon name="inbox" size="large" />
                <span>暫無懲罰</span>
            </div>
        </VaCardContent>
    </VaCard>
</template>

<style scoped>
.latest-card {
    --va-card-padding: 0;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    background:
        linear-gradient(135deg, rgba(229, 9, 20, 0.14), transparent 42%),
        rgba(18, 21, 27, 0.92) !important;
}

.latest-card__content {
    display: flex;
    min-height: 13rem;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem !important;
}

.latest-card__header,
.latest-release__meta {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
}

.latest-card__eyebrow {
    color: #ff6978;
    font-size: 0.72rem;
    font-weight: 800;
    letter-spacing: 0;
    text-transform: uppercase;
}

.latest-card h2 {
    margin: 0.1rem 0 0;
    color: #f7f7f8;
    font-size: 1rem;
    font-weight: 800;
    letter-spacing: 0;
}

.latest-card__icon {
    display: grid;
    width: 2.75rem;
    height: 2.75rem;
    place-items: center;
    border-radius: 8px;
    background: rgba(229, 9, 20, 0.14);
    color: #ff6978;
}

.latest-release {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: space-between;
    gap: 1rem;
}

.latest-release__meta {
    color: #aeb7c7;
    font-size: 0.82rem;
}

.latest-release__chip {
    min-width: 5rem;
    justify-content: center;
}

.latest-release__title {
    display: -webkit-box;
    margin: 0;
    color: #f7f7f8;
    overflow: hidden;
    font-size: 1.12rem;
    font-weight: 800;
    line-height: 1.45;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 4;
}

.latest-release__identity {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    min-width: 0;
}

.latest-release__id {
    color: #f7f7f8;
    font-size: 0.9rem;
    font-weight: 900;
    line-height: 1;
}

.latest-empty {
    display: grid;
    flex: 1;
    place-items: center;
    gap: 0.5rem;
    color: #8f98a8;
    text-align: center;
}
</style>
