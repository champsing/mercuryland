<script setup lang="ts">
import { useAuthState } from "@composables/authState";
import { VaDivider } from "vuestic-ui";
import Backup from "./Backup.vue";
import Channels from "./Channels.vue";
import Anonymous from "./Anonymous.vue";

const authState = useAuthState();

document.title = "系統設定 - 水星人的夢幻樂園";
</script>

<template>
    <main class="px-4 max-md:px-3">
        <div class="page-content">
            <div
                class="flex w-full flex-col md:flex-row items-start md:items-center justify-between gap-2 md:gap-4 py-2"
            >
                <h1 class="text-2xl font-semibold">系統設定</h1>
                <p class="text-zinc-400 sm:text-right">
                    僅供管理員使用的系統設定頁面。請透過 Discord
                    驗證登入後再進行相關操作。
                </p>
            </div>
            <VaDivider class="w-full !mt-0 !mb-2" />

            <div
                v-if="!authState.isResolved"
                class="py-2 text-sm text-zinc-400"
                role="status"
            >
                正在確認登入狀態…
            </div>
            <div
                v-else-if="!authState.isAuthenticated"
                class="rounded-xl border border-zinc-700 bg-zinc-900/50 p-6 text-center"
                role="status"
            >
                <h2 class="text-lg font-semibold text-zinc-200">
                    需要管理員權限
                </h2>
                <p class="mt-2 text-sm text-zinc-400">
                    此頁面僅供管理員使用，請從左上角選單點選「開啟管理」並完成
                    Discord 驗證登入。
                </p>
            </div>
            <div v-else class="grid grid-cols-1 md:grid-cols-3 gap-2 pb-4">
                <Backup />
                <Channels />
                <Anonymous />
            </div>
        </div>
    </main>
</template>

<style scoped>
section {
    min-height: calc(100vh - 8rem);
}
</style>
