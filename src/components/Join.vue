<script setup lang="ts">
import { NGrid, NGi, NCard, NButton, NTimeline, NTimelineItem } from "naive-ui";
import { copyToClipboard } from "@composables/utils"
</script>

<template>
    <div class="text-center">
        <div class="text-6xl perspective-x-30 text-cyan-400">
            現在就立刻加入我們
        </div>
    </div>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1" class="text-center">
            <n-card title="IP" @click="copyToClipboard('play.mercuryland.online:25565')">
                <n-button text class="!text-2xl">play.mercuryland.online:25565</n-button>
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
    <div class="join_guide">
        1. 加入我們的Discord群組來申請白名單 https://discord.gg/A2cMZRr
    </div>
    <div class="join_guide">2. 閱讀伺服器規則</div>
    <div class="join_guide">
        包含《水星法》、《水星伺服器破壞舉報獎勵規則》等。
    </div>
    <div class="join_guide">3. 在#申請伺服打上Minecraft ID</div>
    <div class="join_guide">4. 等待白名單申請通過</div>

    <n-grid x-gap="12" :cols="4" class="w-11/12"> </n-grid>

    <n-grid x-gap="12" :cols="3" class="w-11/12 h-80vh overflow-y-hidden">
    </n-grid>

    <n-divider />
</template>

<style>
.perspective-x-30 {
    transform: perspective(600px) rotateX(30deg);
}
</style>
