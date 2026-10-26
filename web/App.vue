<script setup lang="ts">
import Login from "@/components/login/Login.vue";
import { APP_FOOTER_HEIGHT } from "@/composables/appFooter";
import { useElementSize } from "@vueuse/core";
import { Github } from "@vicons/fa";
import { computed, onBeforeUnmount, onMounted, provide, ref } from "vue";
import { RouterLink } from "vue-router";
import { VaButton, VaDivider, VaIcon, useColors } from "vuestic-ui";
import { useAuthState } from "./composables/authState";
import { backToTop } from "./composables/utils";

useColors().applyPreset("dark");

const authState = useAuthState();

// The footer height differs between desktop (48px) and mobile (two rows), so
// pages that manage their own scroll area measure it instead of hard-coding it.
const footerRef = ref<HTMLElement | null>(null);
const { height: footerHeight } = useElementSize(footerRef);
provide(APP_FOOTER_HEIGHT, footerHeight);

const baseTabs = [
    { path: "/publication", label: "水星伺服器" },
    { path: "/vod", label: "直播隨選" },
    { path: "/penalty", label: "直播懲罰" },
    { path: "/wheel", label: "幸運轉盤" },
    { path: "/leaderboard", label: "水星排行" },
    { path: "/contact", label: "聯絡我們" },
    { path: "/setting", label: "系統設定", requiresAuth: true },
];

const tabs = computed(() =>
    baseTabs.filter((tab) => !tab.requiresAuth || authState.isAuthenticated),
);

const isMenuOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);
const menuTriggerRef = ref<HTMLButtonElement | null>(null);
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

// The disclosure keeps keyboard focus on the trigger and its links while it is
// open, so Tab never wanders into the page behind the menu.
function focusableInMenu(): HTMLElement[] {
    if (!dropdownRef.value) return [];
    return Array.from(
        dropdownRef.value.querySelectorAll<HTMLElement>(
            "button:not([disabled]), a[href], [tabindex]:not([tabindex='-1'])",
        ),
    );
}

function onDocumentKeydown(event: KeyboardEvent) {
    if (!isMenuOpen.value) return;

    if (event.key === "Escape") {
        event.preventDefault();
        closeMenu();
        menuTriggerRef.value?.focus();
        return;
    }

    if (event.key !== "Tab") return;

    const focusables = focusableInMenu();
    if (focusables.length === 0) return;

    const first = focusables[0];
    const last = focusables[focusables.length - 1];
    const active = document.activeElement as HTMLElement | null;

    if (!active || !dropdownRef.value?.contains(active)) {
        event.preventDefault();
        (event.shiftKey ? last : first).focus();
        return;
    }

    if (event.shiftKey && active === first) {
        event.preventDefault();
        last.focus();
    } else if (!event.shiftKey && active === last) {
        event.preventDefault();
        first.focus();
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
    document.addEventListener("keydown", onDocumentKeydown);
});

onBeforeUnmount(() => {
    document.removeEventListener("click", onClickOutside);
    document.removeEventListener("keydown", onDocumentKeydown);
});
</script>

<template>
    <header
        class="fixed top-0 left-0 right-0 z-20 w-full pointer-events-none lg:border-b lg:border-white/10 lg:bg-[#0b0d12]/85 lg:backdrop-blur-md"
    >
        <div
            class="relative flex items-center justify-between px-4 py-3 lg:px-6 lg:py-4"
        >
            <div
                ref="dropdownRef"
                class="relative pointer-events-auto lg:hidden"
            >
                <button
                    ref="menuTriggerRef"
                    type="button"
                    class="flex items-center focus:outline-none"
                    aria-label="切換導覽選單"
                    :aria-expanded="isMenuOpen"
                    aria-controls="app-nav-menu"
                    @click.stop="toggleMenu"
                >
                    <img
                        src="/images/icon.webp"
                        class="h-8 w-8 inline"
                        alt="hexagon"
                    />
                </button>
                <div
                    v-show="isMenuOpen"
                    id="app-nav-menu"
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

            <RouterLink
                to="/"
                class="pointer-events-auto hidden items-center lg:flex"
                aria-label="水星樂園"
                @click="backToTop()"
            >
                <img src="/images/icon.webp" class="h-8 w-8" alt="" />
            </RouterLink>

            <nav
                aria-label="主要導覽"
                class="desktop-nav pointer-events-auto absolute left-1/2 top-1/2 hidden -translate-x-1/2 -translate-y-1/2 items-center gap-1 lg:flex"
            >
                <RouterLink
                    v-for="item in tabs"
                    :key="item.path"
                    :to="item.path"
                    class="shrink-0 whitespace-nowrap rounded-full px-3 py-2 text-sm text-zinc-300 transition-colors hover:bg-white/10 hover:text-white"
                    @click="backToTop()"
                >
                    {{ item.label }}
                </RouterLink>
            </nav>

            <button
                type="button"
                class="pointer-events-auto hidden items-center gap-1.5 whitespace-nowrap rounded-full border px-4 py-2 text-sm font-bold shadow-lg backdrop-blur-md transition-all lg:inline-flex"
                :class="
                    authState.isAuthenticated
                        ? 'border-rose-400/30 bg-rose-400/10 text-rose-200 hover:border-rose-300/50 hover:bg-rose-400/15'
                        : 'border-amber-300/30 bg-amber-300/10 text-amber-200 hover:border-amber-200/60 hover:bg-amber-300/15'
                "
                @click="
                    authState.isAuthenticated ? triggerLogout() : triggerLogin()
                "
            >
                <VaIcon
                    name="admin_panel_settings"
                    size="small"
                    aria-hidden="true"
                />
                {{ authState.isAuthenticated ? "結束管理" : "開啟管理" }}
            </button>
        </div>
    </header>
    <Login ref="loginRef" :render-trigger="false" />
    <div
        class="app-shell flex flex-col lg:pt-[4.5rem]"
        :style="
            footerHeight > 0
                ? { '--app-footer-height': `${footerHeight}px` }
                : undefined
        "
    >
        <div class="flex-1">
            <router-view />
        </div>
        <footer ref="footerRef" class="bg-zinc-900 text-base text-zinc-200">
            <div
                class="mx-auto flex min-h-12 w-[95%] flex-col items-center justify-between gap-2 py-2 md:min-h-12 md:flex-row"
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
        </footer>
    </div>
</template>

<style>
/* Fit the app shell to the visible viewport on mobile browsers, where the
   dynamic toolbar makes 100vh taller than the screen. */
.app-shell {
    min-height: 100vh;
    min-height: 100dvh;
}

.desktop-nav a.router-link-active {
    background-color: rgba(255, 255, 255, 0.1);
    color: #ffffff;
}

.va-navbar {
    --va-navbar-padding-x: 0.7rem;
    --va-navbar-padding-y: 0.6rem;
}
</style>
