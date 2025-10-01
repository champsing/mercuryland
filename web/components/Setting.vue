<script setup lang="ts">
import { VaCard, VaCardContent, VaCardTitle, VaDivider } from "vuestic-ui";

document.title = "系统设置 - 水星人的夢幻樂園";
</script>

<template>
    <div
        class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2"
    >
        <h1 class="ml-12 text-2xl font-semibold">系统设置</h1>
        <p class="text-zinc-400 sm:text-right">
            僅供管理員使用的系統設定頁面。請透過 Discord
            驗證登入後再進行相關操作。
        </p>
    </div>
    <VaDivider class="w-full !mt-0 !mb-2" />

    <div class="grid grid-cols-3 gap-2 px-2">
        <VaCard class="rounded-xl border border-zinc-700">
            <VaCardTitle class="px-6 pt-6 text-lg font-medium text-zinc-200">
                使用者與權限
            </VaCardTitle>
            <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
                功能尚未開放，後續更新將於此頁面公布。
            </VaCardContent>
        </VaCard>
        <VaCard class="rounded-xl border border-zinc-700">
            <VaCardTitle class="px-6 pt-6 text-lg font-medium text-zinc-200">
                通知與整合
            </VaCardTitle>
            <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
                功能尚未開放，請持續關注開發進度。
            </VaCardContent>
        </VaCard>
        <VaCard class="rounded-xl border border-zinc-700">
            <VaCardTitle class="px-6 pt-6 text-lg font-medium text-zinc-200">
                系統維護
            </VaCardTitle>
            <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
                功能尚未開放，仍在規畫中，敬請期待。
            </VaCardContent>
        </VaCard>
    </div>
</template>

<style scoped>
section {
    min-height: calc(100vh - 8rem);
}
</style>
