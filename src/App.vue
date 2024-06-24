<script setup lang="ts">
import { NConfigProvider, NDivider, darkTheme } from "naive-ui";
import { RouterLink } from "vue-router";
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <div>
            <div>
                <router-link to="/" class="tab">
                    <img
                        src="@assets/images/hexagon.svg"
                        class="invert h-8 w-8 inline"
                        alt="hexagon"
                    />
                </router-link>
                <router-link to="/join" class="tab"> 加入伺服 </router-link>
                <router-link to="/publication" class="tab">
                    資料公開
                </router-link>
                <router-link to="/contact" class="tab"> 聯絡我們 </router-link>
                <router-link to="/map" class="tab"> 即時地圖 </router-link>
                <router-link to="/vod" class="tab"> 直播隨選 </router-link>
                <router-link to="/penalty" class="tab"> 直播懲罰 </router-link>
            </div>
            <n-divider class="!mt-1 !mb-1" />
        </div>
        <div><router-view /></div>
    </n-config-provider>
</template>

<style>
.tab {
    @apply text-white;
    @apply text-base;
    @apply ml-4 mr-4;
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
