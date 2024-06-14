<script setup lang="ts">
import { h } from "vue";
import { NTabs, NTabPane, NConfigProvider, darkTheme } from "naive-ui";
import Welcome from "./components/Welcome.vue";
import Vod from "./components/Vod.vue";
import Penalty from "./components/Penalty.vue";

let icon = h("img", {
    src: "/hexagon.svg",
    class: "invert h-8 w-8",
    alt: "hexagon",
});
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <n-tabs type="line" default-value="welcome" animated>
            <n-tab-pane name="welcome" :tab="icon">
                <Welcome />
            </n-tab-pane>
            <n-tab-pane name="vod" tab="直播">
                <div class="pl-8 pr-8">
                    <Vod />
                </div>
            </n-tab-pane>
            <n-tab-pane name="penalty" tab="懲罰">
                <div class="pl-8 pr-8">
                    <Penalty />
                </div>
            </n-tab-pane>
            <n-tab-pane name="Join" tab="加入伺服">
                <div class="pl-8 pr-8">
                    <Announcement />
                </div>
            </n-tab-pane>
            <n-tab-pane name="Publication" tab="資料公開">
                <div class="pl-8 pr-8">
                    <Announcement />
                </div>
            </n-tab-pane>
            <n-tab-pane name="Admins" tab="聯絡管理員">
                <div class="pl-8 pr-8">
                    <Announcement />
                </div>
            </n-tab-pane>
        </n-tabs>
    </n-config-provider>
</template>

<style>
.n-tabs-nav {
    position: sticky !important;
    padding-left: 32px;
    padding-right: 32px;
    z-index: 1;
    top: 0 !important;
    @apply bg-zinc-800;
}
</style>
