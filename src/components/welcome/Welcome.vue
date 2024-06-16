<script setup lang="ts">
import { computed, ref } from "vue";
import Slide1 from "./Slide1.vue";
import Slide2 from "./Slide2.vue";
import Slide3 from "./Slide3.vue";
import Slide4 from "./Slide4.vue";
import Slide5 from "./Slide5.vue";
import { useElementBounding } from "@vueuse/core";
import { useWindowSize } from "vue-window-size";

const vh = useWindowSize().height;

const slide = ref<HTMLInputElement | null>(null);
const slideBounding = useElementBounding(slide);
const slideStyle = computed(() => {
    if (slide.value == null) {
        return { marginTop: "0px" }; // this is important for code to work
    }
    let s = slide.value.style.marginTop;
    let mt = -parseFloat(s.substring(0, s.length - 2));
    let margin = window.scrollY + slideBounding.top.value + mt;
    return {
        marginTop: "-" + margin + "px",
    };
});

const musicStyle = ref({ top: "" + (vh.value - 203) + "px" });

addEventListener("scroll", (_) => {
    let top = window.scrollY + vh.value - 203; 
    musicStyle.value = {
        top: "" + top + "px",
    };
});
</script>

<template>
    <div ref="slide" :style="slideStyle">
        <Slide1 />
        <Slide2 />
        <Slide3 />
        <Slide4 />
        <Slide5 />
    </div>
    <div ref="music" class="absolute right-0" :style="musicStyle">
        <iframe
            allow="autoplay *; encrypted-media *;"
            frameborder="20"
            scrolling="no"
            style="
                width: 100%;
                max-width: 660px;
                overflow: hidden;
                background: transparent;
            "
            sandbox="allow-forms allow-popups allow-same-origin allow-scripts allow-storage-access-by-user-activation allow-top-navigation-by-user-activation"
            src="https://embed.music.apple.com/tw/album/alpha/739831644?i=739831648"
        >
        </iframe>
    </div>
</template>

<style>
@tailwind utilities;
@layer utilities {
    .x-center {
        @apply left-1/2;
        @apply -translate-x-1/2;
    }

    .y-center {
        @apply top-1/2;
        @apply -translate-y-1/2;
    }
}
</style>
