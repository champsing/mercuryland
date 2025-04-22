<script setup lang="ts">
import { copyToClipboard } from "@/composables/utils";
import {
    VaButton,
    VaCard,
    VaCardTitle,
    VaCardContent,
    VaChip,
    VaDivider,
} from "vuestic-ui";

const props = defineProps<{
    name1: string;
    name2: string;
    tags: string[];
    discord: string;
}>();
</script>

<template>
    <div class="w-2/3 max-w-72 h-full m-auto">
        <VaCard class="flex flex-col h-full w-full contact-card">
            <VaCardTitle>
                <div class="flex w-full justify-between">
                    <div class="text-2xl">
                        {{ props.name1 }}
                    </div>
                    <div class="text-base mt-1">
                        {{ props.name2 }}
                    </div>
                </div>
            </VaCardTitle>

            <VaCardContent class="flex flex-col flex-grow">
                <div class="w-full aspect-square">
                    <slot name="avatar"></slot>
                </div>
                <VaDivider />
                <div class="flex-grow">
                    <VaChip
                        class="m-1 text-sm"
                        color="textPrimary"
                        outline
                        readonly
                        v-for="tag in props.tags"
                    >
                        {{ tag }}
                    </VaChip>
                </div>
                <div class="text-center mt-6">
                    Discord:
                    <br />
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
                <VaDivider />
                <div class="text-center font-bold mt-2">推薦連結</div>
                <div class="mt-2">
                    <slot name="rcm-link"></slot>
                </div>
            </VaCardContent>
        </VaCard>
    </div>
</template>

<style>
.contact-card {
    --va-card-box-shadow: 0px;
}
</style>
