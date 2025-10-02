<script setup lang="ts">
import { UseElementBounding, UseWindowSize } from "@vueuse/components";
</script>

<template>
  <use-window-size v-slot="{ width, height }">
    <use-element-bounding v-slot="{ top }">
      <iframe
        src="https://play.mercuryland.pp.ua:25565/"
        :width="width"
        :height="height - top"
      >
      </iframe>
    </use-element-bounding>
  </use-window-size>
</template>
