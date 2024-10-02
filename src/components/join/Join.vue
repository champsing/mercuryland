<script setup lang="ts">
import {
    NButton,
    NButtonGroup,
    NCard,
    NDivider,
    NGrid,
    NGi,
    NIcon,
    NStep,
    NSteps,
} from "naive-ui";
import { copyToClipboard, openLink } from "@composables/utils";
import { ref } from "vue";
import { MdArrowRoundBack, MdArrowRoundForward } from "@vicons/ionicons4";
import { VaButton } from "vuestic-ui";

// const emit = defineEmits<{
//     (e: "toTab", tab: string): void;
// }>();

let currentStep = ref<number | null>(1); //current step

function clickLinkButton(link: string) {
    openLink(link);
    if (currentStep.value < 4) currentStep.value++;
    else currentStep.value = 1;
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
    <br />
    <div class="text-center mb-4">
        <div class="text-6xl perspective-x-30 text-cyan-400">
            現在就立刻加入我們
        </div>
    </div>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12 mb-4" item-responsive>
        <n-gi span="3 800:1" class="text-center">
            <n-card
                title="IP"
                @click="copyToClipboard(serverIP)"
            >
                <n-button text class="!text-2xl">
                    {{ serverIP }}
                </n-button>
            </n-card>
        </n-gi>
        <n-gi span="3 800:1" class="text-center">
            <n-card title="Seed" @click="copyToClipboard(seed.toString())">
                <n-button text class="!text-2xl">
                    {{ seed }}
                </n-button>
            </n-card>
        </n-gi>
        <n-gi span="3 800:1" class="text-center">
            <n-card title="Version">
                <div class="!text-2xl">正版 Minecraft Java {{ version }}</div>
            </n-card>
        </n-gi>
    </n-grid>

    <!-- TODO: prev/next button -->
    <n-steps :current="currentStep" :horizontal="true" class="w-full mt-10">
        <n-step title="加入群組" class="text-1xl w-1/4">
            加入水星人的夢幻樂園Discord群組
            <br />
            <VaButton
                preset="secondary"
                color="#FFFFFF"
                border-color="#969494"
                @click="clickLinkButton('https://discord.gg/A2cMZRr')"
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
                    color="#FFFFFF"
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
                    color="#FFFFFF"
                    border-color="#969494"
                    @click="clickLinkButton('https://discord.gg/CXSQq4nVAH')"
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
    <n-button-group class="mt-4">
        <n-button @click="prev">
            <template #icon>
                <n-icon>
                    <md-arrow-round-back />
                </n-icon>
            </template>
        </n-button>
        <n-button @click="next">
            <template #icon>
                <n-icon>
                    <md-arrow-round-forward />
                </n-icon>
            </template>
        </n-button>
    </n-button-group>
    <n-divider class="mt-2" />
</template>

<style>
.perspective-x-30 {
    transform: perspective(600px) rotateX(30deg);
}
</style>
