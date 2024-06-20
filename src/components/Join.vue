<script setup lang="ts">
import { NGrid, NGi, NCard, NButton, NTimeline, NTimelineItem, NDivider } from "naive-ui";
import { copyToClipboard } from "@composables/utils";
</script>

<template>
    <div class="text-center">
        <div class="text-6xl perspective-x-30 text-cyan-400">
            現在就立刻加入我們
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
            <n-card title="Seed" @click="copyToClipboard('unknown')">
                <n-button text class="!text-2xl">unknown</n-button>
            </n-card>
        </n-gi>
        <n-gi span="3 800:1" class="text-center">
            <n-card title="Version">
                <div class="!text-2xl">Minecraft Java 1.21</div>
            </n-card>
        </n-gi>
    </n-grid>
    <n-divider />
    <n-timeline size="large">
        <n-timeline-item title="加入群组" type="success">
            加入我們的Discord群組來申請白名單 https://discord.gg/A2cMZRr
        </n-timeline-item>
        <n-timeline-item title="阅读规则" type="success">
            包含《水星法》、《水星伺服器破壞舉報獎勵規則》等。
        </n-timeline-item>
        <n-timeline-item title="申请白名单" type="success">
            在 #申請伺服 打上Minecraft ID
        </n-timeline-item>
        <n-timeline-item title="等待通过" line-type="dashed" type="success">
            等待白名單申請通過
        </n-timeline-item>
    </n-timeline>
</template>

<style>
.perspective-x-30 {
    transform: perspective(600px) rotateX(30deg);
}
</style>
