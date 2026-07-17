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
    <div class="pt-16 m-auto w-11/12">
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
                <div class="text-4xl text-neutral-100 text-left mt-4 mb-6">
                    法規與資料
                </div>
                <div class="mt-4">
                    <Law />
                </div>
            </div>

            <div v-if="activeTab === 'archive'" class="mt-4 mb-6">
                <div
                    class="flex flex-row items-center justify-center gap-4 text-neutral-100 mt-4 mb-10"
                >
                    <div class="text-4xl">地圖檔</div>
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
</template>
