<script setup lang="ts">
import {
    VaButton,
    VaDivider,
    useColors,
    VaNavbar,
    VaNavbarItem,
} from "vuestic-ui";
import { RouterLink } from "vue-router";
import { computed } from "vue";
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
</script>

<template>
    <VaNavbar
        :class="$route.fullPath == '/' ? `z-20 fixed` : `z-20 sticky`"
        :color="$route.fullPath == '/' ? `rgba(0, 0, 0, 0)` : `rgb(24, 24, 27)`"
    >
        <template #left>
            <VaNavbarItem>
                <router-link to="/" class="ml-4">
                    <img
                        src="/images/hexagon.svg"
                        class="invert h-8 w-8 inline"
                        alt="hexagon"
                    />
                </router-link>
            </VaNavbarItem>
        </template>
        <template #center>
            <VaNavbarItem>
                <router-link
                    v-for="t in tabs"
                    :to="t.path"
                    class="ml-4 text-base text-white"
                    @click="backToTop()"
                >
                    {{ t.label }}
                </router-link>
            </VaNavbarItem>
        </template>
        <template #right>
            <VaNavbarItem>
                <div class="flex flex-row justify-center">
                    <div class="mx-2 mt-1">
                        <Login />
                    </div>
                </div>
            </VaNavbarItem>
        </template>
    </VaNavbar>
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
