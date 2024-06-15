<script setup lang="ts">
import { computed, ref } from "vue";
import Slide1 from "./Slide1.vue";
import Slide2 from "./Slide2.vue";
import { useElementBounding } from "@vueuse/core";

const el = ref<HTMLInputElement | null>(null);
const elBounding = useElementBounding(el);
const elStyle = computed(() => {
    if (el.value == null) {
        return { "margin-top": "0px" }; // this is important for code to work
    }
    let s = el.value.style.marginTop;
    let mt = -parseFloat(s.substring(0, s.length - 2));
    let margin = window.scrollY + elBounding.top.value + mt;
    return {
        "margin-top": "-" + margin + "px",
    };
});
</script>

<template>
    <div ref="el" :style="elStyle">
        <Slide1 />
        <Slide2 />
    </div>
    <iframe 
    allow="autoplay *; encrypted-media *;" 
    frameborder="20" 
    height="200" 
    style="width:100%;max-width:660px;overflow:hidden;background:transparent;" 
    sandbox="allow-forms allow-popups allow-same-origin allow-scripts allow-storage-access-by-user-activation allow-top-navigation-by-user-activation" 
    src="https://embed.music.apple.com/tw/album/alpha/739831644?i=739831648">
    </iframe>
    
</template>
