<script setup lang="ts">
import { ref } from "vue";
import {
    NButton,
    NCard,
    NConfigProvider,
    NDivider,
    NFlex,
    darkTheme,
} from "naive-ui";
import { RouterLink } from "vue-router";
import { useElementBounding } from "@vueuse/core";
import { openLinkSameTab } from "./composables/utils";

const tabNav = ref<HTMLInputElement | null>(null);
const tabNavBounding = useElementBounding(tabNav);

// function calcTabNavStyle(path: string) {
//     if (path == "/") {
//         return {};
//     } else {
//         return {
//             backgroundColor: "rgb(38 38 38)",
//         };
//     }
// }

function calcMainStyle(path: string) {
    // if (path == "/") {
    //     return {};
    // }
    if (path == "/map") {
        return {
            marginTop: "" + tabNavBounding.height.value + "px",
        };
    } else {
        return {
            marginTop: "" + (tabNavBounding.height.value + 8) + "px",
            marginLeft: "auto",
            marginRight: "auto",
            width: "91.666%",
        };
    }
}

// const slide = ref<HTMLInputElement | null>(null);
// const slideBounding = useElementBounding(slide);
// const slideStyle = computed(() => {
//     if (slide.value == null) {
//         return { marginTop: "0px" }; // this is important for code to work
//     }
//     let s = slide.value.style.marginTop;
//     let mt = -parseFloat(s.substring(0, s.length - 2));
//     let margin = scroll.y.value + slideBounding.top.value + mt;
//     return {
//         marginTop: "-" + margin + "px",
//     };
// });
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <!-- don't need calcTabNavStyle($route.fullPath) when no server -->
        <div
            ref="tabNav"
            class="tab-nav w-full"
            style="background-color: rgb(38 38 38)"
        >
            <div class="p-3">
                <!-- <router-link to="/" class="tab">
                    <img
                        src="@assets/images/hexagon.svg"
                        class="invert h-8 w-8 inline"
                        alt="hexagon"
                    />
                </router-link> -->
                <!-- <router-link to="/join" class="tab"> 加入伺服 </router-link> -->
                <router-link to="/publication" class="tab">
                    資料公開
                </router-link>
                <!-- <router-link to="/map" class="tab"> 即時地圖 </router-link> -->
                <router-link to="/" class="tab"> 直播隨選 </router-link>
                <router-link to="/penalty" class="tab"> 直播懲罰 </router-link>
                <router-link to="/contact" class="tab"> 聯絡我們 </router-link>
            </div>
            <n-divider class="!m-0" />
        </div>
        <div :style="calcMainStyle($route.fullPath)">
            <router-view />
        </div>
    </n-config-provider>
    <n-card class="bottom-card mt-5 m-auto text-center" size="small">
        <div class="text-center text-base text-slate-800">
            <n-flex style="justify-content: center">
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
            </n-flex>
            <!-- <div class="mt-2">隱私權政策&ensp;|&ensp;使用條款&ensp;|&ensp;法律聲明</div> -->
            <div class="mt-2">
                <n-button
                :bordered="false"
                :focusable="false"
                tertiary
                @click="openLinkSameTab('https://mercuryland.online/#/tos')"
                >
                    使用條款 - 準備中
                </n-button>
            </div>
        </div>
    </n-card>
</template>

<style>
.tab {
    @apply text-white;
    @apply text-base;
    @apply ml-4 mr-4;
}

.tab-nav {
    @apply fixed;
    @apply z-10;
    @apply top-0;
}

.bottom-card {
    width: 90%;
    --n-color: #a5deeb !important;
}
</style>
