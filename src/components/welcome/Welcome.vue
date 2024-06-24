<script setup lang="ts">
import { computed, ref } from "vue";
import Slide1 from "./Slide1.vue";
import Slide2 from "./Slide2.vue";
import Slide3 from "./Slide3.vue";
import Slide4 from "./Slide4.vue";
import Slide5 from "./Slide5.vue";
import {
    useWindowSize,
    useWindowScroll,
    useElementBounding,
} from "@vueuse/core";

const emit = defineEmits<{
    (e: "toTab", tab: string): void;
}>();

const vh = useWindowSize().height;
const scroll = useWindowScroll();

const slide = ref<HTMLInputElement | null>(null);
const slideBounding = useElementBounding(slide);
const slideStyle = computed(() => {
    if (slide.value == null) {
        return { marginTop: "0px" }; // this is important for code to work
    }
    let s = slide.value.style.marginTop;
    let mt = -parseFloat(s.substring(0, s.length - 2));
    let margin = scroll.y.value + slideBounding.top.value + mt;
    return {
        marginTop: "-" + margin + "px",
    };
});

const musicStyle = computed(() => {
    let bottom = slideBounding.height.value - scroll.y.value - vh.value;
    return {
        bottom: "" + bottom + "px",
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
            scrolling="no"
            height="150"
            width="300"
            class="overflow-hidden"
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
