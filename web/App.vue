<script setup lang="ts">
import { NConfigProvider, darkTheme } from "naive-ui";
import {
    VaButton,
    VaDivider,
    useColors,
    VaNavbar,
    VaNavbarItem,
} from "vuestic-ui";
import { RouterLink } from "vue-router";
import Login from "@components/Login.vue";

useColors().applyPreset("dark");

const tabs = [
    { path: "/join", label: "加入伺服" },
    { path: "/publication", label: "資料公開" },
    { path: "/vod", label: "直播隨選" },
    { path: "/penalty", label: "直播懲罰" },
    { path: "/wheel", label: "幸運轉盤" },
    { path: "/contact", label: "聯絡我們" },
];
</script>

<template>
    <VaNavbar
        :class="$route.fullPath == '/' ? `z-20 fixed` : `z-20 sticky`"
        :color="$route.fullPath == '/' ? `rgba(0, 0, 0, 0)` : `rgb(24, 24, 27)`"
    >
        <template #left>
            <VaNavbarItem class="navbar-item-slot">
                <router-link to="/" class="ml-4">
                    <img
                        src="@assets/images/hexagon.svg"
                        class="invert h-8 w-8 inline"
                        alt="hexagon"
                    />
                </router-link>
            </VaNavbarItem>
            <VaNavbarItem class="navbar-item-slot" v-for="t in tabs">
                <router-link :to="t.path" class="ml-4 text-base text-white">
                    {{ t.label }}
                </router-link>
            </VaNavbarItem>
        </template>
        <template #right>
            <VaNavbarItem class="navbar-item-slot">
                <div class="mr-4">
                    <Login />
                </div>
            </VaNavbarItem>
        </template>
    </VaNavbar>
    <div class="min-h-screen">
        <n-config-provider :theme="darkTheme">
            <router-view />
        </n-config-provider>
    </div>
    <div class="text-center text-base text-zinc-200 pt-4 pb-4 bg-zinc-900">
        <div class="flex justify-center">
            <div style="font-family: playfair display">
                Copyright © 2024 The Mercury Land
            </div>
            <!-- 
                    【&nbsp;】：半角スペースと同じサイズの空白
                    【&thinsp;】：&nbsp;の空白より小さい空白
                    【&ensp;】：半角スペースより間隔がやや広い空白
                    【&emsp;】：全角スペースとほぼ同じサイズの空白 
                -->
            &ensp;保留一切權利。
        </div>
        <!-- <div class="mt-2">使用條款&ensp;|&ensp;隱私權政策&ensp;|&ensp;法律聲明</div> -->
        <div class="flex justify-center mt-2">
            <VaButton preset="secondary" border-color="#363636" to="tos">
                <div class="text-zinc-200">使用條款</div>
            </VaButton>
            <VaDivider vertical />
            <VaButton preset="secondary" border-color="#363636" to="privacy">
                <div class="text-zinc-200">隱私權政策</div>
            </VaButton>
            <VaDivider vertical />
            <VaButton
                preset="secondary"
                border-color="#363636"
                href="https://www.youtube.com/watch?v=Yir_XAcccmY"
                target="_blank"
                rel="noopener noreferrer"
            >
                <div class="text-zinc-200">使用教學</div>
            </VaButton>
        </div>
    </div>
</template>

<style>
.va-navbar {
    --va-navbar-padding-x: 0.7rem;
    --va-navbar-padding-y: 0.6rem;
}
</style>
