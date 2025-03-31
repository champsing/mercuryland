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
function next() {
    if (currentStep.value === null) currentStep.value = 0;
    else if (currentStep.value >= 3) currentStep.value = null;
    else currentStep.value++;
}

function prev() {
    if (currentStep.value === null) currentStep.value = 0;
    else if (currentStep.value <= 0) currentStep.value = 3;
    else currentStep.value--;
}

const serverOnline = false; // Set to false if the server is offline
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
    <div class="mt-8 m-auto w-11/12" v-if="!serverOnline">
        <div class="text-center mb-4">
            <div class="text-6xl perspective-x-30 text-red-400">
                伺服器目前關閉中
            </div>
        </div>
        <div class="text-center text-2xl">
            重新開放日期將於 Discord 群組另行公告。
        </div>
        <div class="text-center text-2xl mt-4">
            敬請耐心等候並留意「伺服公告」頻道，謝謝！
        </div>
        <div class="text-center text-2xl mt-4">
            伺服器開放遊玩時，請務必遵守伺服器規則，<br>以免造成不必要的損失。
            <div class="mt-4 ml-4">
                伺服器遊玩規則在關服期間仍可於<a
                    href="/#/publication"
                    class="underline text-teal-500"
                    >資料公開</a
                >查詢。
            </div>
        </div>
    </div>
    <div class="mt-8 m-auto w-11/12" v-if="serverOnline">
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
                <div class="text-2xl">加入水星人的夢幻樂園Discord群組</div>
                <br />
                <VaButton
                    preset="secondary"
                    color="textPrimary"
                    border-color="#969494"
                    @click="currentStep++"
                    :href="discordInvitation"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="mb-2"
                >
                    點擊加入群組
                </VaButton>
                <br />
                或使用連結：
                <VaInput
                    v-model="discordInvitation"
                    readonly
                    style="width: 260px"
                    @click="copyToClipboard(discordInvitation)"
                />
            </template>
            <template #step-content-1>
                <div class="text-2xl">
                    前往「資料公開」，閱讀包含《水星法》、<br />《水星伺服器破壞舉報獎勵規則》等規則。
                </div>
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
                <div class="text-2xl">在 #申請伺服 打上Minecraft ID</div>
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
            <template #step-content-3>
                <div class="text-2xl">
                    等待白名單申請通過期間，您可自行嘗試連接伺服器；<br />若成功進入遊玩，即代表申請成功。
                </div>
            </template>
            <template #controls>
                <VaButton
                    @click="prev()"
                    :disabled="currentStep == 0"
                    preset="primary"
                >
                    上一步
                </VaButton>
                <VaButton @click="next()" v-if="currentStep < 3">
                    下一步
                </VaButton>
            </template>
        </VaStepper>
    </div>
</template>

<style>
.perspective-x-30 {
    transform: perspective(600px) rotateX(30deg);
}
</style>
