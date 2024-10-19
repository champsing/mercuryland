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
        <VaCard class="h-full w-full ContactCard">
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

            <VaCardContent>
                <div class="w-full aspect-square">
                    <slot></slot>
                </div>
                <VaDivider />
                <div class="flex flex-col h-full !justify-between">
                    <div>
                        <VaChip
                            class="m-1 text-base"
                            color="#d9d9d9"
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
                            color="#d9d9d9"
                            hover-mask-color="#5bc6a1"
                            hover-opacity="1"
                            @click="copyToClipboard(props.discord.toString())"
                        >
                            <div class="font-bold">@{{ props.discord }}</div>
                        </VaButton>
                    </div>
                </div>
            </VaCardContent>
        </VaCard>
    </div>
</template>

<style>
.ContactCard {
    --va-card-overflow: hidden;
}
</style>