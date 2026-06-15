<script setup lang="ts">
import { copyToClipboard } from "@composables/utils";
import { ref } from "vue";
import { VaButton, VaStepper } from "vuestic-ui";

let currentStep = ref<number | null>(0);

document.title = "加入伺服 - 水星人的夢幻樂園";

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

const serverOnline = true;
const serverIP = "mercuryland.exaroton.me";
const seed = "NOT REVEALED";
const version = "26.1.2";
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
    <div class="h-20" v-if="!serverOnline"></div>
    <div class="flex flex-row justify-between" v-if="!serverOnline">
        <div class="mt-20 w-1/2">
            <div class="text-center mb-4">
                <div class="text-6xl text-red-400">伺服器目前關閉中</div>
            </div>
            <div class="text-center text-2xl">
                重新開放日期將於 Discord 群組另行公告。
            </div>
            <div class="text-center text-2xl mt-4">
                敬請耐心等候並留意「伺服公告」頻道，謝謝！
            </div>
            <div class="text-center text-2xl mt-4">
                伺服器開放遊玩時，請務必遵守伺服器規則，<br />以免造成不必要的損失。
                <div class="mt-4 ml-4">
                    伺服器遊玩規則在關服期間仍可於本頁查詢。
                </div>
            </div>
        </div>
        <iframe
            class="m-auto"
            src="https://discord.com/widget?id=506120681495199756&theme=dark"
            height="600"
            width="500"
        />
    </div>

    <div class="mt-8" v-if="serverOnline">
        <!-- Header -->
        <div class="text-center mb-10">
            <div
                class="inline-flex items-center gap-2 mb-5 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/25"
            >
                <span
                    class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"
                ></span>
                <span class="text-emerald-400 text-sm tracking-wide"
                    >伺服器開放中</span
                >
            </div>
            <h1 class="text-5xl font-bold text-white tracking-tight">
                現在就立刻加入我們
            </h1>
        </div>

        <!-- Server Info Cards -->
        <div class="flex justify-center gap-4 flex-wrap">
            <button
                class="group flex-1 min-w-40 max-w-68 rounded-xl bg-zinc-900 border border-zinc-700 hover:border-cyan-500/40 p-5 text-left transition-all duration-200 cursor-pointer"
                @click="copyToClipboard(serverIP)"
            >
                <div
                    class="text-xs text-zinc-400 uppercase tracking-widest mb-3"
                >
                    IP
                </div>
                <div
                    class="font-mono text-zinc-100 group-hover:text-cyan-300 transition-colors break-all"
                >
                    {{ serverIP }}
                </div>
                <div
                    class="text-xs text-zinc-400 mt-3 group-hover:text-zinc-400 transition-colors"
                >
                    點擊複製
                </div>
            </button>

            <button
                class="group flex-1 min-w-40 max-w-68 rounded-xl bg-zinc-900 border border-zinc-700 hover:border-cyan-500/40 p-5 text-left transition-all duration-200 cursor-pointer"
                @click="copyToClipboard(seed)"
            >
                <div
                    class="text-xs text-zinc-400 uppercase tracking-widest mb-3"
                >
                    Seed
                </div>
                <div
                    class="font-mono text-zinc-100 group-hover:text-cyan-300 transition-colors"
                >
                    {{ seed }}
                </div>
                <div
                    class="text-xs text-zinc-400 mt-3 group-hover:text-zinc-400 transition-colors"
                >
                    點擊複製
                </div>
            </button>

            <div
                class="flex-1 min-w-40 max-w-68 rounded-xl bg-zinc-900 border border-zinc-700 p-5"
            >
                <div
                    class="text-xs text-zinc-400 uppercase tracking-widest mb-3"
                >
                    Version
                </div>
                <div class="font-mono text-zinc-100">Java {{ version }}</div>
                <div class="text-xs text-zinc-400 mt-3">正版限定</div>
            </div>
        </div>

        <!-- Steps -->
        <div class="mt-10">
            <VaStepper
                color="info"
                v-model="currentStep"
                :steps="steps"
                class="w-full"
                finishButtonHidden
            >
                <template #step-content-0>
                    <div class="text-xl text-zinc-200 mb-4">
                        加入水星人的夢幻樂園Discord群組
                    </div>
                    <VaButton
                        preset="secondary"
                        color="textPrimary"
                        border-color="#969494"
                        :href="discordInvitation"
                        target="_blank"
                        rel="noopener noreferrer"
                        @click="currentStep++"
                    >
                        點擊加入群組
                    </VaButton>
                </template>
                <template #step-content-1>
                    <div class="text-xl text-zinc-200 mb-4">
                        前往「資料公開」，閱讀包含《水星法》、<br />《水星伺服器破壞舉報獎勵規則》等規則。
                    </div>
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
                </template>
                <template #step-content-2>
                    <div class="text-xl text-zinc-200 mb-4">
                        在 #申請伺服 打上Minecraft ID
                    </div>
                    <VaButton
                        preset="secondary"
                        color="textPrimary"
                        border-color="#969494"
                        :href="applyWhitelist"
                        target="_blank"
                        rel="noopener noreferrer"
                        @click="currentStep++"
                    >
                        點擊跳轉頻道
                    </VaButton>
                </template>
                <template #step-content-3>
                    <div class="text-xl text-zinc-200">
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
    </div>
</template>
