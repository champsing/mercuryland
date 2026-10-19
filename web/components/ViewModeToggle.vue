<script setup lang="ts">
import { BlockViewMode } from "@/composables/viewMode";
import { VaIcon } from "vuestic-ui";

const mode = defineModel<BlockViewMode>({ required: true });

const options: {
    value: BlockViewMode;
    label: string;
    icon: string;
}[] = [
    { value: "grid", label: "卡片檢視", icon: "grid_view" },
    { value: "list", label: "列表檢視", icon: "view_list" },
];
</script>

<template>
    <div
        class="inline-flex items-center gap-1 rounded-lg border border-white/10 bg-zinc-900/60 p-1"
        role="group"
        aria-label="區塊顯示方式"
    >
        <button
            v-for="option in options"
            :key="option.value"
            type="button"
            class="inline-flex h-10 w-10 cursor-pointer items-center justify-center rounded-md border-0 transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#8ea2ff]"
            :class="
                mode === option.value
                    ? 'bg-[#5865f2] text-white shadow-sm shadow-black/30'
                    : 'bg-transparent text-zinc-400 hover:bg-white/10 hover:text-zinc-100'
            "
            :aria-label="option.label"
            :aria-pressed="mode === option.value"
            :title="option.label"
            @click="mode = option.value"
        >
            <VaIcon :name="option.icon" size="large" />
        </button>
    </div>
</template>
