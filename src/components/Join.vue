<script setup lang="ts">
import { NButton, NButtonGroup, NCard, NGrid, NGi, NIcon, NStep, NSteps } from "naive-ui";
import { ccMix, copyToClipboard, openLink } from "@composables/utils";
import { ref } from "vue";
import { MdArrowRoundBack, MdArrowRoundForward } from '@vicons/ionicons4'
//TODO: Server Online Status
//import { IosRadioButtonOn } from "@vicons/ionicons4";

// const emit = defineEmits<{
//     (e: "toTab", tab: string): void;
// }>();

let currentStep = ref<number | null>(1);
//current step

function clickLinkButton(link: string, toBeSetValue: number, setValue: number) {
    openLink(link);
    toBeSetValue = setValue;
}

//Prev/Next Button

function next() {
    if (currentStep.value === null)
        currentStep.value = 1
    else if (currentStep.value >= 4)
        currentStep.value = null
    else currentStep.value++
}

function prev() {
    if (currentStep.value === 0)
        currentStep.value = null
    else if (currentStep.value === null)
        currentStep.value = 4
    else currentStep.value--
}

</script>

<template>
    <br/>
    <div class="text-center mb-4">
        <div class="text-6xl perspective-x-30 text-cyan-400">
            {{ ccMix("現在就立刻加入我們") }}
        </div>
    </div>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12 mb-4" item-responsive>
        <n-gi span="3 800:1" class="text-center">
            <n-card
                title="IP"
                @click="copyToClipboard('play.mercuryland.online:25565')"
            >
                <n-button text class="!text-2xl"
                    >play.mercuryland.online:25565</n-button
                >
            </n-card>
        </n-gi>
        <n-gi span="3 800:1" class="text-center">
            <n-card title="Seed" @click="copyToClipboard('-9100272987300380909')">
                <n-button text class="!text-2xl">{{ ccMix("-9100272987300380909") }}</n-button>
            </n-card>
        </n-gi>
        <n-gi span="3 800:1" class="text-center">
            <n-card title="Version">
                <div class="!text-2xl">{{ ccMix("正版") }} Minecraft Java 1.21</div>
            </n-card>
        </n-gi>
    </n-grid>

    <!-- TODO: prev/next button -->
    <n-steps :current=currentStep :horizontal="true" class="w-full mt-10">
        <n-step :title="ccMix('加入群組')" class="text-1xl w-1/4">
            {{ ccMix("加入我們的Discord群組來申請白名單") }}
            <br/>
            <n-button @click="clickLinkButton('https://discord.gg/A2cMZRr', currentStep, 2)" class="mt-2 mb-2">
                {{ ccMix("加入 水星人的夢幻樂園") }}
            </n-button>
            <br/>
            {{ ccMix("或使用連結") }}：https://discord.gg/A2cMZRr
        </n-step>

        <n-step :title="ccMix('閱讀規則')" class="text-1xl w-1/4">
            {{ ccMix("包含《水星法》、《水星伺服器破壞舉報獎勵規則》等。") }} 
            <br/>
            <div class="w-full m-auto  mt-2 mb-2">  
                <n-button size="large" @click="clickLinkButton('https://mercuryland.online/#/publication', currentStep, 3)">
                    {{ ccMix("點擊閱讀")}}
                </n-button>
            </div>
        </n-step>

        <n-step class="text-2xl w-1/4"
        :title="ccMix('申請白名單')"
        description="在 #申請伺服 打上Minecraft ID"
        />

        <n-step  class="text-2xl w-1/4"
        :title="ccMix('等待通過')" line-type="dashed"
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
</template>

<style>
.perspective-x-30 {
    transform: perspective(600px) rotateX(30deg);
}
</style>
