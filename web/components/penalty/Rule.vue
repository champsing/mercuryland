<script setup lang="ts">
import { ref } from "vue";
import { VaButton, VaIcon, VaModal } from "vuestic-ui";
import { Info24Regular } from "@vicons/fluent";

const showRuleDescModal = ref(false);
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
        hide-default-actions
        close-button
        :mobile-fullscreen="false"
        z-index="4"
    >
        <div class="text-lg font-semibold text-zinc-200">直播懲罰規則說明</div>
        <div class="text-xl mt-2">一、概述</div>
        <div class="text-base mt-2">
            惡靈會在直播的時候跟觀眾打賭該局遊戲加減懲罰的賭注，然後在直播最後以隨機輪盤抽出當天的懲罰數量。
            <br />
            每個懲罰會各自擁有一個完成狀態，分別有：已生效、已完成、勉強過、進行中。
            <br />
            關於各完成狀態的說明，請點擊下方圖例中的「完成狀態」查看。
        </div>
        <div class="text-xl mt-4">二、加班台懲罰</div>
        <div class="text-base mt-2">
            如果懲罰主文要求加班台時數，則只有在該懲罰生成「之後」加的班才會被計算進該懲罰的完成進度裡。
            <br />
            例如：
            <br />
            01/01被懲罰加班台2小時，01/02惡靈有加班時數47分鐘，則這47分鐘可以被計算進01/01的「加班台2小時」懲罰裡。
            <br />
            反之，若在01/03也有懲罰加班台2小時懲罰，01/02的47分鐘就不會被算進01/03懲罰完成進度裡。
        </div>
    </VaModal>
</template>

<style scoped>
/* 玻璃擬態樣式，對標 EditPenalty.vue */
:deep(.va-modal__dialog) {
    background: rgba(255, 255, 255, 0.1) !important;
    backdrop-filter: blur(20px) !important;
    -webkit-backdrop-filter: blur(20px) !important;
    border-radius: 24px !important;
    border: 1px solid rgba(255, 255, 255, 0.4) !important;
    box-shadow:
        0 20px 40px -15px rgba(0, 0, 0, 0.08),
        0 0 0 1px rgba(0, 0, 0, 0.02) !important;
    max-width: 680px !important;
    width: 95% !important;
    transition: all 0.3s ease;
}

:global(.dark) :deep(.va-modal__dialog),
:global(.va-theme--dark) :deep(.va-modal__dialog) {
    background: rgba(15, 23, 42, 0.65) !important;
    border: 1px solid rgba(255, 255, 255, 0.08) !important;
    box-shadow:
        0 25px 50px -12px rgba(0, 0, 0, 0.35),
        0 0 0 1px rgba(255, 255, 255, 0.04) !important;
}

:deep(.va-modal__close) {
    top: 1rem !important;
    right: 1rem !important;
    color: currentColor !important;
    opacity: 0.6;
}
</style>
