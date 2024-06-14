<script setup lang="ts">
import { h } from "vue"
import { NTabs, NTabPane, NConfigProvider, darkTheme } from "naive-ui";
import Welcome from "./components/Welcome.vue";
import Vod from "./components/Vod.vue";
import Penalty from "./components/Penalty.vue";

let icon = h("img", { "src": "/hexagon.svg", "class": "invert h-8 w-8" })

</script>

<template>
    <n-config-provider :theme="darkTheme">
        <n-tabs type="line" default-value="vod" animated>
            <n-tab-pane name="Welcome" :tab="icon">
                <Welcome />
            </n-tab-pane>
            <n-tab-pane name="vod" tab="直播">
                <Vod />
            </n-tab-pane>
            <n-tab-pane name="penalty" tab="惩罚">
                <Penalty />
            </n-tab-pane>
        </n-tabs>
    </n-config-provider>
</template>

<style>
.n-tabs-nav {
    display: block;
    position: sticky !important;
    z-index: 1;
    top: 0 !important;
    @apply bg-zinc-800
}   
</style>
