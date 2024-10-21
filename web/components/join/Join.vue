<script setup lang="ts">
import { NStep, NSteps } from "naive-ui";
import { copyToClipboard } from "@composables/utils";
import { ref } from "vue";
import { MdArrowRoundBack, MdArrowRoundForward } from "@vicons/ionicons4";
import {
    VaButton,
    VaButtonGroup,
    VaCard,
    VaCardContent,
    VaCardTitle,
    VaIcon,
} from "vuestic-ui";

// const emit = defineEmits<{
//     (e: "toTab", tab: string): void;
// }>();

let currentStep = ref<number | null>(1); //current step

function clickLinkButton() {
    currentStep.value < 4 ? currentStep.value++ : (currentStep.value = 1);
}

//Prev/Next Button
function next() {
    if (currentStep.value === null) currentStep.value = 1;
    else if (currentStep.value >= 4) currentStep.value = null;
    else currentStep.value++;
}

function prev() {
    if (currentStep.value === null) currentStep.value = 0;
    else if (currentStep.value <= 0) currentStep.value = 4;
    else currentStep.value--;
}

const serverIP = "play.mercuryland.online:25565";
const seed = -9100272987300380909;
const version = 1.21;
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

        <n-steps :current="currentStep" :horizontal="true" class="w-full mt-10">
            <n-step title="加入群組" class="text-1xl w-1/4">
                加入水星人的夢幻樂園Discord群組
                <br />
                <VaButton
                    preset="secondary"
                    color="textPrimary"
                    border-color="#969494"
                    @click="clickLinkButton()"
                    href="https://discord.gg/A2cMZRr"
                    target="_blank"
                    class="mt-2 mb-2"
                >
                    點擊加入群組
                </VaButton>
                <br />

                或使用連結：https://discord.gg/A2cMZRr
            </n-step>

            <n-step title="閱讀規則" class="text-1xl w-1/4">
                包含《水星法》、《水星伺服器破壞舉報獎勵規則》等。
                <br />
                <div class="w-full m-auto mt-2 mb-2">
                    <VaButton
                        preset="secondary"
                        color="textPrimary"
                        border-color="#969494"
                        to="publication"
                    >
                        點擊閱讀規則
                    </VaButton>
                </div>
            </n-step>

            <n-step class="text-1xl w-1/4" title="申請白名單">
                在 #申請伺服 打上Minecraft ID
                <br />
                <div class="w-full m-auto mt-2 mb-2">
                    <VaButton
                        preset="secondary"
                        color="textPrimary"
                        border-color="#969494"
                        @click="clickLinkButton()"
                        href="https://discord.gg/CXSQq4nVAH"
                        target="_blank"
                    >
                        點擊跳轉頻道
                    </VaButton>
                </div>
            </n-step>

            <n-step
                class="text-1xl w-1/4"
                title="等待通過"
                line-type="dashed"
                description="等待白名單申請通過"
            />
        </n-steps>
        <VaButtonGroup
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
        </VaButtonGroup>
    </div>
</template>

<style>
.perspective-x-30 {
    transform: perspective(600px) rotateX(30deg);
}
</style>
