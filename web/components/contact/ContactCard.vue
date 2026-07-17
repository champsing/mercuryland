<script setup lang="ts">
import { copyToClipboard } from "@/composables/utils";
import { VaButton, VaChip } from "vuestic-ui";

const props = defineProps<{
    name1: string;
    name2: string;
    tags: string[];
    discord: string;
}>();
</script>

<template>
    <div class="w-full max-w-2xl m-auto">
        <div
            class="name-card flex flex-row h-full w-full bg-surface flex-wrap overflow-hidden"
        >
            <div class="avatar-side flex items-center justify-center shrink-0">
                <div class="w-full aspect-square">
                    <slot name="avatar"></slot>
                </div>
            </div>

            <div class="info-side flex flex-col flex-grow justify-center">
                <div class="flex items-baseline gap-3 flex-wrap">
                    <div class="text-3xl font-bold">{{ props.name1 }}</div>
                    <div class="text-lg text-secondary">{{ props.name2 }}</div>
                </div>

                <div class="w-24 h-0.5 bg-primary my-2"></div>

                <div class="flex flex-wrap gap-2 mb-3">
                    <VaChip
                        class="text-xs"
                        color="textPrimary"
                        outline
                        readonly
                        v-for="tag in props.tags"
                        size="small"
                    >
                        {{ tag }}
                    </VaChip>
                </div>

                <div class="flex items-center gap-2 mb-2">
                    <span class="discord-label">Discord</span>
                    <VaButton
                        preset="plain"
                        color="textPrimary"
                        hover-mask-color="#5bc6a1"
                        hover-opacity="1"
                        @click="copyToClipboard(props.discord.toString())"
                    >
                        <div class="font-bold">@{{ props.discord }}</div>
                    </VaButton>
                </div>

                <div class="rcm-section">
                    <div class="rcm-title">推薦連結</div>
                    <div>
                        <slot name="rcm-link"></slot>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.name-card {
    border: 1px solid var(--va-border);
    border-radius: 1rem;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
    transition:
        transform 0.2s ease,
        box-shadow 0.2s ease;
}

.name-card:hover {
    transform: translateY(-3px);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.22);
}

.avatar-side {
    width: clamp(7rem, 40%, 10rem);
    padding: 1.25rem;
    background: linear-gradient(
        135deg,
        var(--va-background-badge),
        var(--va-background-card)
    );
    border-right: 1px solid var(--va-border);
}

.info-side {
    padding: 1.5rem;
    min-width: 200px;
}

.discord-label {
    font-size: 0.8rem;
    color: var(--va-text-secondary);
    letter-spacing: 0.05em;
    text-transform: uppercase;
}

.rcm-section {
    margin-top: 0.5rem;
}

.rcm-title {
    font-size: 0.8rem;
    color: var(--va-text-secondary);
    letter-spacing: 0.05em;
    text-transform: uppercase;
    margin-bottom: 0.5rem;
}

@media (max-width: 480px) {
    .name-card {
        flex-direction: column;
    }

    .avatar-side {
        width: 100%;
        max-width: 10rem;
        border-right: none;
        border-bottom: 1px solid var(--va-border);
    }
}
</style>
