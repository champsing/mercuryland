<script setup lang="ts">
import mapArchives from "@assets/data/map_archives.json";

const hostMeta: Record<string, { label: string; color: string; bg: string }> = {
    googledrive: {
        label: "Google Drive",
        color: "text-blue-300",
        bg: "bg-blue-500/10 border border-blue-500/25",
    },
    mediafire: {
        label: "MediaFire",
        color: "text-orange-300",
        bg: "bg-orange-500/10 border border-orange-500/25",
    },
};
</script>

<template>
    <div class="grid grid-cols-4 grid-rows-3 gap-24 px-10">
        <a
            v-for="archive in mapArchives"
            :key="archive.season"
            :href="archive.link"
            target="_blank"
            rel="noopener noreferrer"
            class="flex flex-row gap-4 items-center px-6 py-3 rounded-full transition-all duration-200 cursor-pointer"
            :class="hostMeta[archive.host]?.bg"
        >
            <div class="text-xl text-neutral-100">{{ archive.season }}</div>
            <div class="text-sm" :class="hostMeta[archive.host]?.color">
                {{ hostMeta[archive.host]?.label }}
            </div>
        </a>
    </div>
</template>
