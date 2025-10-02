<script setup lang="ts">
document.title = "資料公開 - 水星人的夢幻樂園";

import { VaDivider, VaIcon } from "vuestic-ui";
import Law from "./Law.vue";
// import NewLaw from "./NewLaw.vue"; //如果沒有新版本水星法可以註解起來
import Archive from "./Archive.vue";
import { WindowNew20Filled } from "@vicons/fluent";
</script>

<template>
  <div class="mt-4 m-auto w-11/12">
    <!-- 如果沒有新版本水星法可以註解起來 -->
    <!-- 
        <NewLaw /> 
        <VaDivider /> 
        -->
    <div class="text-4xl text-neutral-100 text-left mt-4 mb-6">法規與資料</div>
    <div class="mt-4">
      <Law />
    </div>
    <VaDivider class="!mt-6 !mb-2" />
    <div class="flex flex-row gap-10 text-neutral-100 mt-4 mb-10">
      <div class="text-4xl">地圖檔</div>
      <div class="flex flex-row gap-1">
        <VaIcon size="large">
          <WindowNew20Filled />
        </VaIcon>
        <div class="text-base text-center">將開啟新分頁</div>
      </div>
    </div>
    <Archive class="mt-4 mb-6" />
  </div>
</template>
