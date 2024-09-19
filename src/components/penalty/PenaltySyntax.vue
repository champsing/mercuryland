<script setup lang="ts">
import { VaCard, VaCardContent, VaCardTitle } from "vuestic-ui";
import { NDivider, NFlex } from "naive-ui";
</script>

<template>
    <div class="text-4xl text-neutral-100 text-left mt-4">圖例</div>
    <n-divider class="!mt-3" />
    <n-flex justify="center" class="m-auto" item-responsive>
        <VaCard gradient color="#28c9c7" class="ml-2">
            <VaCardTitle style="font-size: 16px">詳細資料</VaCardTitle>
            <VaCardContent>
                <div class="text-base">
                    🆙增加、🔁重抽、2️⃣備案、📝原主人修改n次、➕其他後來增加的條件
                </div>
            </VaCardContent>
        </VaCard>
        <VaCard gradient color="#005c99">
            <VaCardTitle style="font-size: 16px">完成狀態</VaCardTitle>
            <VaCardContent>
                <div class="text-base">
                    ✅完成、✅已抽、🏁原主人或投票給過、⏲️⚔️目前已完成進度
                </div>
            </VaCardContent>
        </VaCard>
    </n-flex>
</template>
