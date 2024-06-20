<script setup lang="ts">
import { NCard, NDivider, NTag } from "naive-ui";

const props = defineProps<{
    name1: string;
    name2: string;
    tags: string[];
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
            <template v-for="tag in props.tags">
                <n-tag class="m-1"> {{ tag }}</n-tag>
            </template>
        </n-card>
    </div>
</template>
