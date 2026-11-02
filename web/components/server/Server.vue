<script setup lang="ts">
document.title = "水星伺服器 - 水星人的夢幻樂園";

import { ref } from "vue";
import { VaIcon } from "vuestic-ui";
import Law from "./Law.vue";
// import NewLaw from "./NewLaw.vue"; //如果沒有新版本水星法可以註解起來
import { WindowNew20Filled } from "@vicons/fluent";
import Archive from "./Archive.vue";
import Join from "./Join.vue";

const activeTab = ref<"join" | "publication" | "archive">("join");

const tabs = [
    { name: "join", label: "加入伺服" },
    { name: "publication", label: "法規與資料" },
    { name: "archive", label: "地圖檔" },
] as const;
</script>

<template>
    <main
        class="min-h-[calc(100vh-48px)] bg-page-surface px-4 pb-5 pt-10 text-[#f7f7f8] max-md:px-3 max-md:pb-4 max-md:pt-16"
    >
        <div class="page-content">
            <div class="mb-6 flex flex-wrap items-center gap-x-6 gap-y-3">
                <h1 class="text-2xl font-bold text-neutral-100">水星伺服器</h1>
                <div
                    role="tablist"
                    aria-label="水星伺服器分頁"
                    class="flex items-center gap-1 rounded-full border border-white/10 bg-white/5 p-1 shadow-[0_2px_12px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.08)] backdrop-blur-md"
                >
                    <button
                        v-for="tab in tabs"
                        :key="tab.name"
                        type="button"
                        role="tab"
                        :aria-selected="activeTab === tab.name"
                        class="rounded-full px-4 py-1.5 text-sm font-medium text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                        :class="
                            activeTab === tab.name
                                ? 'bg-gradient-to-b from-white/20 to-white/10 text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.2),0_1px_3px_rgba(0,0,0,0.25)]'
                                : ''
                        "
                        @click="activeTab = tab.name"
                    >
                        {{ tab.label }}
                    </button>
                </div>
            </div>

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
    </main>
</template>
