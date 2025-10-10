<script setup lang="ts">
// TODO: Make the wheel password into the setting page
// TODO: Make vote channel, exchange channel & penalty channel into setting page
import { VaDivider } from "vuestic-ui";
import Backup from "./Backup.vue";
import Wheel from "./Wheel.vue";

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
        <Backup />
        <Wheel />
    </div>
</template>

<style scoped>
section {
    min-height: calc(100vh - 8rem);
}
</style>
