<script setup lang="ts">
import Login from "@/components/login/Login.vue";
import { Github } from "@vicons/fa";
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { RouterLink } from "vue-router";
import { VaButton, VaDivider, VaIcon, useColors } from "vuestic-ui";
import { useAuthState } from "./composables/authState";
import { backToTop } from "./composables/utils";

useColors().applyPreset("dark");

const authState = useAuthState();

const baseTabs = [
    { path: "/publication", label: "水星伺服器" },
    { path: "/vod", label: "直播隨選" },
    { path: "/penalty", label: "直播懲罰" },
    { path: "/wheel", label: "幸運轉盤" },
    { path: "/leaderboard", label: "水星排行" },
    { path: "/contact", label: "聯絡我們" },
    { path: "/setting", label: "系统设置", requiresAuth: true },
];

const tabs = computed(() =>
    baseTabs.filter((tab) => !tab.requiresAuth || authState.isAuthenticated),
);

const isMenuOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);
const loginRef = ref<InstanceType<typeof Login> | null>(null);

function toggleMenu() {
    isMenuOpen.value = !isMenuOpen.value;
}

function closeMenu() {
    isMenuOpen.value = false;
}

function onClickOutside(event: MouseEvent) {
    if (!dropdownRef.value) return;
    const target = event.target as Node;
    if (!dropdownRef.value.contains(target)) {
        closeMenu();
    }
}

function triggerLogin() {
    closeMenu();
    loginRef.value?.openLoginModal();
}

function triggerLogout() {
    closeMenu();
    loginRef.value?.openLogoutModal();
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
        <div class="flex items-center justify-between px-4 py-3">
            <div ref="dropdownRef" class="relative pointer-events-auto">
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
                    class="absolute left-0 mt-3 w-56 max-w-[calc(100vw-2rem)] rounded-md border border-zinc-700 bg-zinc-900 py-2 shadow-lg"
                >
                    <nav class="flex flex-col">
                        <router-link
                            to="/"
                            class="px-4 py-2 text-left text-base text-zinc-200 hover:bg-zinc-800"
                            @click="
                                backToTop();
                                closeMenu();
                            "
                        >
                            水星樂園
                        </router-link>
                        <router-link
                            v-for="item in tabs"
                            :key="item.path"
                            :to="item.path"
                            class="px-4 py-2 text-left text-base text-zinc-200 hover:bg-zinc-800"
                            @click="
                                backToTop();
                                closeMenu();
                            "
                        >
                            {{ item.label }}
                        </router-link>
                        <button
                            v-if="authState.isAuthenticated"
                            type="button"
                            class="px-4 py-2 text-left text-base text-zinc-200 hover:bg-zinc-800"
                            @click="triggerLogout"
                        >
                            結束管理
                        </button>
                        <button
                            v-else
                            type="button"
                            class="px-4 py-2 text-left text-base text-zinc-200 hover:bg-zinc-800"
                            @click="triggerLogin"
                        >
                            開啟管理
                        </button>
                    </nav>
                </div>
            </div>
        </div>
    </header>
    <Login ref="loginRef" :render-trigger="false" />
    <div class="flex min-h-screen flex-col">
        <div class="flex-1">
            <router-view />
        </div>
        <div class="bg-zinc-900 text-base text-zinc-200">
            <div
                class="mx-auto flex min-h-12 w-[95%] flex-col items-center justify-between gap-2 py-2 md:h-12 md:flex-row"
            >
                <div
                    class="flex flex-row items-center gap-2 text-center"
                    style="font-family: playfair display"
                >
                    <div>Copyright © 2026 The Mercury Land</div>
                    <div class="hidden md:block">保留一切權利。</div>
                </div>
                <div class="flex flex-row items-center">
                    <VaButton
                        preset="secondary"
                        :bordered="false"
                        to="tos"
                        aria-label="使用條款"
                        title="使用條款"
                        @click="backToTop()"
                    >
                        <VaIcon name="description" class="text-red-300" />
                        <span class="text-red-300 max-md:hidden">使用條款</span>
                    </VaButton>
                    <VaDivider vertical class="mx-2" />
                    <VaButton
                        preset="secondary"
                        :bordered="false"
                        to="privacy"
                        aria-label="隱私政策"
                        title="隱私政策"
                        @click="backToTop()"
                    >
                        <VaIcon name="privacy_tip" class="text-sky-300" />
                        <span class="text-sky-300 max-md:hidden">隱私政策</span>
                    </VaButton>
                    <VaDivider vertical class="mx-2" />
                    <VaButton
                        preset="secondary"
                        :bordered="false"
                        href="https://www.youtube.com/watch?v=Yir_XAcccmY"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="使用教學"
                        title="使用教學"
                    >
                        <VaIcon name="play_circle" class="text-lime-300" />
                        <span class="text-lime-300 max-md:hidden"
                            >使用教學</span
                        >
                    </VaButton>
                    <VaDivider vertical class="mx-2" />
                    <VaButton
                        preset="secondary"
                        :bordered="false"
                        href="https://github.com/champsing/mercuryland"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="開源代碼"
                        title="開源代碼"
                    >
                        <VaIcon class="text-orange-300">
                            <Github />
                        </VaIcon>
                        <span class="text-orange-300 max-md:hidden"
                            >開源代碼</span
                        >
                    </VaButton>
                </div>
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
