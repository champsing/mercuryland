<script setup lang="ts">
import { h, ref } from "vue";
import { NTabs, NTabPane, NConfigProvider, darkTheme } from "naive-ui";
import hexagonIcon from "@assets/images/hexagon.svg";
import { RouterLink } from "vue-router";

let icon = h("img", {
    src: hexagonIcon,
    class: "invert h-8 w-8",
    alt: "hexagon",
});
let tabStyle = ref({
    "--tab-nav-color": "transparent",
});
let tabValue = defineModel("tabValue", {
    default: "welcome",
    set(value: string) {
        if (value == "welcome") {
            tabStyle.value = {
                "--tab-nav-color": "transparent",
            };
        } else {
            tabStyle.value = {
                // equivalent to neutral-800
                "--tab-nav-color": "rgb(38 38 38)",
            };
        }
        return value;
    },
});
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <nav>
            <router-link to="/" class="tab">Go to Home</router-link>
            <router-link to="/join" class="tab">Go to Join</router-link>
            <router-link to="/publication" class="tab">Go to Publication</router-link>
            <router-link to="/contact" class="tab">Go to Contact</router-link>
            <router-link to="/map" class="tab">Go to Map</router-link>
            <router-link to="/vod" class="tab">Go to Vod</router-link>
            <router-link to="/penalty" class="tab">Go to Penalty</router-link>
        </nav>
        <router-view />
        <!--         
        <n-tabs
            type="line"
            default-value="welcome"
            animated
            :style="tabStyle"
            v-model:value="tabValue"
        >
            <n-tab-pane name="welcome" :tab="icon">
                <Welcome @toTab="(value) => (tabValue = value)" />
            </n-tab-pane>
            <n-tab-pane name="join" tab="加入">
                <div class="pl-8 pr-8">
                    <Join @toTab="(value) => (tabValue = value)" />
                </div>
            </n-tab-pane>
            <n-tab-pane name="publication" tab="資料公開">
                <div class="pl-8 pr-8">
                    <Publication />
                </div>
            </n-tab-pane>
            <n-tab-pane name="contact" tab="聯絡">
                <div class="pl-8 pr-8">
                    <Contact />
                </div>
            </n-tab-pane>
            <n-tab-pane name="map" tab="即時地图">
                <Suspense>
                    <GameMap />
                </Suspense>
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
        </n-tabs> -->
    </n-config-provider>
</template>

<style>
.tab {
    @apply text-white
}

.n-tabs-nav {
    position: sticky !important;
    padding-left: 32px;
    padding-right: 32px;
    z-index: 10;
    top: 0 !important;
    background-color: var(--tab-nav-color);
}
.n-tabs-pane-wrapper {
    overflow: visible !important;
}
</style>
