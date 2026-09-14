<script setup lang="ts">
import { VaDivider } from "vuestic-ui";
import Backup from "./Backup.vue";
import Channels from "./Channels.vue";
import Anonymous from "./Anonymous.vue";

document.title = "系统设置 - 水星人的夢幻樂園";
</script>

<template>
    <div
        class="flex w-full flex-col md:flex-row items-start md:items-center justify-between gap-2 md:gap-4 px-2 py-2"
    >
        <h1 class="md:ml-12 text-2xl font-semibold">系统设置</h1>
        <p class="text-zinc-400 sm:text-right">
            僅供管理員使用的系統設定頁面。請透過 Discord
            驗證登入後再進行相關操作。
        </p>
    </div>
    <VaDivider class="w-full !mt-0 !mb-2" />

    <div class="grid grid-cols-1 md:grid-cols-3 gap-2 m-4">
        <Backup />
        <Channels />
        <Anonymous />
    </div>
</template>

<style scoped>
section {
    min-height: calc(100vh - 8rem);
}
</style>
