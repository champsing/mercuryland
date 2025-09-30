<script setup lang="ts">
import { VaButton, VaDivider, useColors } from "vuestic-ui";
import { RouterLink } from "vue-router";
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import Login from "@/components/login/Login.vue";
import { backToTop } from "./composables/utils";
import { useAuthState } from "./composables/authState";

useColors().applyPreset("dark");

const authState = useAuthState();

const baseTabs = [
    // { path: "/join", label: "加入伺服" },
    // { path: "/publication", label: "資料公開" },
    { path: "/vod", label: "直播隨選" },
    { path: "/penalty", label: "直播懲罰" },
    { path: "/wheel", label: "幸運轉盤" },
    { path: "/leaderboard", label: "水星排行" },
    { path: "/setting", label: "系统设置", requiresAuth: true },
    // { path: "/propose", label: "直播提案" },
    { path: "/contact", label: "聯絡我們" },
];

const tabs = computed(() =>
    baseTabs.filter((tab) => !tab.requiresAuth || authState.isAuthenticated)
);

const dropdownItems = computed(() => [
    { path: "/", label: "首頁" },
    ...tabs.value,
]);

const isMenuOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);

function toggleMenu() {
    isMenuOpen.value = !isMenuOpen.value;
}

function closeMenu() {
    isMenuOpen.value = false;
}

function handleMenuItemClick() {
    backToTop();
    closeMenu();
}

function onClickOutside(event: MouseEvent) {
    if (!dropdownRef.value) return;
    const target = event.target as Node;
    if (!dropdownRef.value.contains(target)) {
        closeMenu();
    }
}

onMounted(() => {
    document.addEventListener("click", onClickOutside);
});

onBeforeUnmount(() => {
    document.removeEventListener("click", onClickOutside);
});
</script>

<template>
    <header class="fixed top-0 left-0 right-0 z-20 w-full pointer-events-none">
        <div class="flex items-center justify-between px-4 py-3 pointer-events-auto">
            <div ref="dropdownRef" class="relative">
                <button
                    type="button"
                    class="flex items-center focus:outline-none"
                    aria-label="切換導覽選單"
                    @click.stop="toggleMenu"
                >
                    <img
                        src="/images/icon.webp"
                        class="h-8 w-8 inline"
                        alt="hexagon"
                    />
                </button>
                <div
                    v-if="isMenuOpen"
                    class="absolute left-0 mt-3 w-56 rounded-md border border-zinc-700 bg-zinc-900 py-2 shadow-lg"
                >
                    <nav class="flex flex-col">
                        <router-link
                            v-for="item in dropdownItems"
                            :key="item.path"
                            :to="item.path"
                            class="px-4 py-2 text-left text-base text-zinc-200 hover:bg-zinc-800"
                            @click="handleMenuItemClick"
                        >
                            {{ item.label }}
                        </router-link>
                    </nav>
                </div>
            </div>
            <div class="flex flex-row justify-center">
                <div class="mx-2 mt-1">
                    <Login />
                </div>
            </div>
        </div>
    </header>
    <div class="min-h-screen">
        <router-view />
    </div>
    <div class="text-base text-zinc-200 pt-4 pb-4 bg-zinc-900">
        <div class="flex flex-row items-center justify-between w-[95%] mx-auto">
            <div class="flex flex-row items-center gap-2" style="font-family: playfair display">
                <div>Copyright © 2025 The Mercury Land</div>
                <div>保留一切權利。</div>
            </div>
            <div class="flex flex-row items-center">
                <VaButton
                    preset="secondary"
                    border-color="#363636"
                    to="tos"
                    @click="backToTop()"
                >
                    <div class="text-zinc-200">使用條款</div>
                </VaButton>
                <VaDivider vertical class="mx-2" />
                <VaButton
                    preset="secondary"
                    border-color="#363636"
                    to="privacy"
                    @click="backToTop()"
                >
                    <div class="text-zinc-200">隱私政策</div>
                </VaButton>
                <VaDivider vertical class="mx-2" />
                <VaButton
                    preset="secondary"
                    border-color="#363636"
                    href="https://www.youtube.com/watch?v=Yir_XAcccmY"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <div class="text-zinc-200">使用教學</div>
                </VaButton>
                <VaDivider vertical class="mx-2" />
                <VaButton
                    preset="secondary"
                    border-color="#363636"
                    href="https://github.com/champsing/mercuryland"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <div class="text-zinc-200">開源代碼</div>
                </VaButton>
            </div>
        </div>
    </div>
</template>

<style>
.va-navbar {
    --va-navbar-padding-x: 0.7rem;
    --va-navbar-padding-y: 0.6rem;
}
</style>
