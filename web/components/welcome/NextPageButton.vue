<script setup lang="ts">
import { useWindowSize } from "@vueuse/core";
import { VaButton, VaIcon } from "vuestic-ui";
import { AngleDown } from "@vicons/fa";

const props = defineProps<{ page: number }>();

const vh = useWindowSize().height;
function nextPage() {
  window.scrollTo({ top: props.page * vh.value, behavior: "smooth" });
}
</script>

<template>
  <div class="z-10 absolute -bottom-2 w-full text-center">
    <VaButton
      class="w-auto opacity-30 hover:opacity-100"
      round
      color="#5bc6a1"
      preset="secondary"
      :hover-opacity="0"
      :pressed-opacity="0"
      size="small"
      @click="nextPage()"
    >
      <VaIcon size="128px">
        <angle-down />
      </VaIcon>
    </VaButton>
  </div>
</template>
