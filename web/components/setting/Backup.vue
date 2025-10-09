<script setup lang="ts">
import { ref } from "vue";
import axios from "axios";
import { VaButton, VaCard, VaCardContent, VaCardTitle } from "vuestic-ui";
import { BASE_URL } from "@/composables/utils";

const isVodDownloading = ref(false);

async function downloadDatabase() {
    if (isVodDownloading.value) return;

    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        isVodDownloading.value = true;
        const response = await axios.get(
            `${BASE_URL}/api/setting/backup?token=${token}`,
            {
                responseType: "blob",
            },
        );

        const blob = new Blob([response.data], {
            type: "application/octet-stream",
        });
        const url = window.URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = `backup-${new Date().toISOString().slice(0, 19).replace(/T/, "-")}.db`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        window.URL.revokeObjectURL(url);
    } catch (error) {
        console.error("Database backup download failed", error);
    } finally {
        isVodDownloading.value = false;
    }
}
</script>

<template>
    <VaCard class="rounded-xl border border-zinc-700">
        <VaCardTitle
            class="px-6 pt-6 !text-xl font-medium text-zinc-200 justify-center"
        >
            数据备份
        </VaCardTitle>
        <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
            <VaButton
                preset="primary"
                color="info"
                class="w-full"
                :loading="isVodDownloading"
                @click="downloadDatabase"
            >
                下载数据库
            </VaButton>
        </VaCardContent>
    </VaCard>
</template>

<style scoped></style>
