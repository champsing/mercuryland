<script setup lang="ts">
import { NCard, NDivider } from "naive-ui";

const props = defineProps<{
    name1: string;
    name2: string;
    title: string;
    discord: string;
}>();
</script>

<template>
    <div class="w-2/3 max-w-72 h-full m-auto">
        <n-card :title="props.name1" class="h-full">
            <template #header-extra>{{ props.name2 }}</template>
            <template #footer>discord@{{ props.discord }}</template>
            <div class="w-full aspect-square">
                <slot></slot>
            </div>
            <n-divider />
            {{ props.title }}
        </n-card>
    </div>
</template>
