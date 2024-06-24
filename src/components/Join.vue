<script setup lang="ts">
import { NGrid, NGi, NCard, NButton, NTimeline, NTimelineItem, NDivider } from "naive-ui";
import { copyToClipboard, openLink, openLinkSameTab } from "@composables/utils";
import { ccMix } from "@composables/utils";
//TODO: Server Online Status
//import { IosRadioButtonOn } from "@vicons/ionicons4";

// const emit = defineEmits<{
//     (e: "toTab", tab: string): void;
// }>();
</script>

<template>
    <br/>
    <div class="text-center">
        <div class="text-6xl perspective-x-30 text-cyan-400">
            {{ ccMix("現在就立刻加入我們") }}
        </div>
    </div>
    <n-divider />
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
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
    <n-divider />
    <n-timeline size="large" x-gap="12" :horizontal="true" class="w-full">
        <n-timeline-item title="加入群组" type="success" class="text-2xl w-1/4">
            {{ ccMix("加入我們的Discord群組來申請白名單") }}
            <br/>
            <n-button @click="openLink('https://discord.gg/A2cMZRr')" class="mt-2 mb-2">
                加入 水星人的夢幻樂園
            </n-button>
            <br/>
            或使用連結：https://discord.gg/A2cMZRr
        </n-timeline-item>

        <n-timeline-item title="阅读规则" type="success" class="text-2xl w-1/4">
            包含《水星法》、《水星伺服器破壞舉報獎勵規則》等。
            <br/>
            <div class="w-full m-auto  mt-2 mb-2">
                <n-button @click="openLinkSameTab('../publication')">
                    點擊加入
                </n-button>
            </div>
        </n-timeline-item>

        <n-timeline-item title="申请白名单" type="success" class="text-2xl w-1/4">
            在 #申請伺服 打上Minecraft ID
        </n-timeline-item>

        <n-timeline-item title="等待通过" line-type="dashed" type="success" class="text-2xl w-1/4">
            等待白名單申請通過
        </n-timeline-item>
    </n-timeline>
</template>

<style>
.perspective-x-30 {
    transform: perspective(600px) rotateX(30deg);
}
</style>
