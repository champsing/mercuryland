<script setup lang="ts">
import { ref } from "vue";
import { VaButton, VaIcon, VaModal } from "vuestic-ui";
import { Info24Regular } from "@vicons/fluent";

const showRuleDescModal = ref(false);
const showVodDescImg = ref(false);
</script>

<template>
  <VaButton
    preset="plain"
    class="w-full"
    color="#FFFFFF"
    @click="showRuleDescModal = !showRuleDescModal"
  >
    <VaIcon size="large" class="mr-2">
      <Info24Regular />
    </VaIcon>
    <div class="text-lg text-center">規則說明</div>
  </VaButton>

  <VaModal
    v-model="showRuleDescModal"
    title="規則說明"
    hide-default-actions
    close-button
    z-index="4"
  >
    <span class="text-3xl"> 直播時數規則說明 </span>
    <div class="text-2xl mt-2">●概述</div>
    <div class="text-bg mt-2">
      惡靈公布直播紀錄檔時，此處會同步更新計算加班台的剩餘時數，並標上當天遊玩的遊戲，供使用者藉由上方的標籤篩選功能找到自己想看的遊戲直播。
    </div>
    <div class="text-2xl mt-2">●時數計算說明</div>
    <div class="text-bg mt-2">
      在計算明細表中，會以「計劃」、「直播」等項目的時數互相加減得出剩餘時數。
      <br />
      <ol class="ml-3">
        <li>
          「計劃」為惡靈的常規直播時數，目前落在2小時左右，因此以2小時計算。
        </li>
        <li>
          每次直播的時數沒意外的話以YouTube影片時長為準。若直播紀錄檔被和諧了，則以2小時計算。
        </li>
        <li>
          有時在直播懲罰會生成加班台懲罰，此處也會以「懲罰」項目來增加剩餘的加班台時數。
        </li>
        <li>
          觀眾可以用每小時真錢 10 美元或 1000
          水星幣的價格購買加班台，此處也會以「課金」項目來增加剩餘的加班台時數。
        </li>
        <li>若有其他因素導致時數加減也會以獨立項目處理。</li>
        <li>伺服器時間每週三 00:00 會在計算明細表生成一項「計劃」。</li>
      </ol>
      <VaButton
        class="mt-2"
        preset="primary"
        border-color="info"
        color="info"
        gradient
        @click="showVodDescImg = !showVodDescImg"
      >
        查看說明圖例
      </VaButton>
    </div>

    <div class="text-2xl mt-2">●不可抗力因素</div>
    <div class="text-bg mt-2">
      若惡靈在直播過程中斷網或停電，則中間嘗試恢復的多次黑畫面直播紀錄檔將不會被採計，直到恢復1分鐘以上的穩定直播為止。
    </div>
  </VaModal>

  <VaModal
    v-model="showVodDescImg"
    hide-default-actions
    close-button
    z-index="5"
  >
    <img src="/images/vod_time.webp" alt="直播時數規則說明" />
  </VaModal>
</template>
