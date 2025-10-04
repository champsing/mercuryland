<script setup lang="ts">
document.title = "歡迎來到水星人的夢幻樂園";

// const emit = defineEmits<{
//     (e: "toTab", tab: string): void;
// }>();
</script>

<template>
    <div class="relative overflow-hidden" style="height: calc(100vh - 48px)">
        <NextPageButton :page="1" />
        <div class="absolute x-center y-center text-center">
            <div class="text-white font-bold text-8xl">水星乐园</div>
            <VaDivider class="mt-6 mb-6 opacity-70" />
            <div class="text-white italic text-3xl">The Mercury Land</div>
        </div>
        <img
            src="/images/welcome/welcome.webp"
            alt="Welcome"
            class="-z-10 w-full h-full object-cover absolute top-0 left-0"
        />
    </div>
</template>

<style>
@tailwind utilities;
@layer utilities {
    .x-center {
        @apply left-1/2;
        @apply -translate-x-1/2;
    }

    .y-center {
        @apply top-1/2;
        @apply -translate-y-1/2;
    }
}
</style>
