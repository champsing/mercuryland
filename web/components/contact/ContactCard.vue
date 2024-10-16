<script setup lang="ts">
import { copyToClipboard } from "@/composables/utils";
import { NButton, NCard, NTag } from "naive-ui";
import { VaDivider } from "vuestic-ui";

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
            
            <template #footer>
                <div class="text-center">
                    Discord: 
                    <br/>
                    <n-button
                    :text="true"
                    :focusable="false"
                    @click="copyToClipboard(props.discord.toString())">
                        <div class="font-bold">
                            @{{ props.discord }}
                        </div>    
                    </n-button>
                </div>
            </template>

            <div class="w-full aspect-square">
                <slot></slot>
            </div>
            <VaDivider />
            <template v-for="tag in props.tags">
                <n-tag class="m-1"> {{ tag }} </n-tag>
            </template>
        </n-card>
    </div>
</template>
