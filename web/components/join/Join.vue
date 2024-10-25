<script setup lang="ts">
import { copyToClipboard } from "@composables/utils";
import { ref } from "vue";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaCardTitle,
    VaInput,
    VaStepper,
} from "vuestic-ui";

let currentStep = ref<number | null>(0); //current step

// For handwriting Chinese prev/next button set
// function next() {
//     if (currentStep.value === null) currentStep.value = 0;
//     else if (currentStep.value >= 3) currentStep.value = null;
//     else currentStep.value++;
// }

// function prev() {
//     if (currentStep.value === null) currentStep.value = 0;
//     else if (currentStep.value <= 0) currentStep.value = 3;
//     else currentStep.value--;
// }

const serverIP = "play.mercuryland.online:25565";
const seed = -9100272987300380909;
const version = 1.21;
const discordInvitation = "https://discord.gg/A2cMZRr";
const applyWhitelist = "https://discord.gg/CXSQq4nVAH";

const steps = [
    { label: "加入群組" },
    { label: "閱讀規則" },
    { label: "申請白名單" },
    { label: "等待通過" },
];
</script>

<template>
    <div class="mt-8 m-auto w-11/12">
        <div class="text-center mb-4">
            <div class="text-6xl perspective-x-30 text-cyan-400">
                現在就立刻加入我們
            </div>
        </div>
        <div class="flex justify-center text-center gap-16">
            <VaCard class="w-1/3">
                <VaCardTitle style="font-size: 16px"> IP </VaCardTitle>
                <VaCardContent>
                    <VaButton
                        color="textPrimary"
                        preset="plain"
                        @click="copyToClipboard(serverIP)"
                    >
                        <span class="text-2xl">
                            {{ serverIP }}
                        </span>
                    </VaButton>
                </VaCardContent>
            </VaCard>
            <VaCard class="w-1/3">
                <VaCardTitle style="font-size: 16px"> Seed </VaCardTitle>
                <VaCardContent>
                    <VaButton
                        preset="plain"
                        color="textPrimary"
                        @click="copyToClipboard(seed.toString())"
                    >
                        <span class="text-2xl">
                            {{ seed }}
                        </span>
                    </VaButton>
                </VaCardContent>
            </VaCard>
            <VaCard class="w-1/3">
                <VaCardTitle style="font-size: 16px"> Version </VaCardTitle>
                <VaCardContent>
                    <span class="text-2xl">
                        正版 Minecraft Java {{ version }}
                    </span>
                </VaCardContent>
            </VaCard>
        </div>

        <VaStepper
            color="info"
            v-model="currentStep"
            :steps="steps"
            class="w-full mt-10"
            finishButtonHidden
        >
            <template #step-content-0>
                加入水星人的夢幻樂園Discord群組
                <br />
                <VaButton
                    preset="secondary"
                    color="textPrimary"
                    border-color="#969494"
                    @click="currentStep++"
                    :href="discordInvitation"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="mt-2 mb-2"
                >
                    點擊加入群組
                </VaButton>
                <br />
                或使用連結：
                <VaInput v-model="discordInvitation" readonly />
            </template>
            <template #step-content-1>
                包含《水星法》、《水星伺服器破壞舉報獎勵規則》等。
                <br />
                <div class="w-full m-auto mt-2 mb-2">
                    <VaButton
                        preset="secondary"
                        color="textPrimary"
                        border-color="#969494"
                        to="publication"
                        target="_blank"
                        @click="currentStep++"
                    >
                        點擊閱讀規則
                    </VaButton>
                </div>
            </template>
            <template #step-content-2>
                在 #申請伺服 打上Minecraft ID
                <br />
                <div class="w-full m-auto mt-2 mb-2">
                    <VaButton
                        preset="secondary"
                        color="textPrimary"
                        border-color="#969494"
                        @click="currentStep++"
                        :href="applyWhitelist"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        點擊跳轉頻道
                    </VaButton>
                </div>
            </template>
            <template #step-content-3> 等待白名單申請通過 </template>
        </VaStepper>

        <!-- TODO: I still want to handwrite a Chinese prev/next button set; 
        but currently the previous/next button provided by Vuestic is quite enough.-->
        <!-- <VaButtonGroup
            color="secondary"
            border-color="warning"
            gradient
            class="mt-4 mb-6"
        >
            <VaButton preset="secondary" @click="prev">
                <VaIcon>
                    <md-arrow-round-back />
                </VaIcon>
            </VaButton>
            <VaButton preset="secondary" @click="next">
                <VaIcon>
                    <md-arrow-round-forward />
                </VaIcon>
            </VaButton>
        </VaButtonGroup> -->
    </div>
</template>

<style>
.perspective-x-30 {
    transform: perspective(600px) rotateX(30deg);
}
</style>
