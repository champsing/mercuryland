<script setup lang="ts">
document.title = "資料公開 - 水星人的夢幻樂園";

import { ref } from "vue";
import { VaIcon, VaTab, VaTabs } from "vuestic-ui";
import Law from "./Law.vue";
// import NewLaw from "./NewLaw.vue"; //如果沒有新版本水星法可以註解起來
import { WindowNew20Filled } from "@vicons/fluent";
import Archive from "./Archive.vue";
import Join from "./Join.vue";

const activeTab = ref<"join" | "publication" | "archive">("join");
</script>

<template>
    <div class="publication-page min-h-[calc(100vh-48px)] text-[#f7f7f8] pt-10 pb-5 px-4 max-md:pt-16 max-md:pb-4 max-md:px-3">
        <div class="w-full max-w-[1400px] mx-auto">
            <VaTabs v-model="activeTab" color="info" center grow>
                <template #tabs>
                    <VaTab name="join" color="info">加入伺服</VaTab>
                    <VaTab name="publication" color="info">法規與資料</VaTab>
                    <VaTab name="archive" color="info">地圖檔</VaTab>
                </template>
            </VaTabs>

            <div class="mt-10">
                <Join v-if="activeTab === 'join'" />

                <div v-if="activeTab === 'publication'">
                    <!-- 如果沒有新版本水星法可以註解起來 -->
                    <!--
                    <NewLaw />
                    <VaDivider />
                    -->
                    <div class="p-10">
                        <Law />
                    </div>
                </div>

                <div v-if="activeTab === 'archive'" class="mt-4 mb-6">
                    <div
                        class="flex flex-row items-center justify-center gap-4 text-neutral-100 mt-4 mb-10"
                    >
                        <div class="flex flex-row items-center gap-1">
                            <VaIcon size="large">
                                <WindowNew20Filled />
                            </VaIcon>
                            <div class="text-base">將開啟新分頁</div>
                        </div>
                    </div>
                    <Archive />
                </div>
            </div>
        </div>
    </div>
</template>

<style>
.publication-page {
    background:
        radial-gradient(
            circle at 50% 0%,
            rgba(142, 120, 255, 0.09),
            transparent 36rem
        ),
        radial-gradient(
            circle at 15% 0%,
            rgba(88, 166, 255, 0.2),
            transparent 32rem
        ),
        linear-gradient(135deg, #0f1117 0%, #16191f 48%, #101318 100%);
}
</style>
