<script setup lang="ts">
import { NTabs, NTabPane, NConfigProvider, darkTheme } from "naive-ui";
import Vod from "./components/Vod.vue";
import Penalty from "./components/HelloWorld.vue"
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <n-tabs type="line" default-value="vod" animated>
            <n-tab-pane name="vod" tab="直播">
                <Vod />
            </n-tab-pane>
            <n-tab-pane name="penalty" tab="惩罚">
                <Penalty />
            </n-tab-pane>
        </n-tabs>
    </n-config-provider>
</template>

<style scoped></style>
