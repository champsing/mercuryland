<script setup lang="ts">
import { VaCard, VaCardContent, VaCardTitle } from "vuestic-ui";
</script>

<template>
    <div class="flex flex-wrap gap-5">
        <VaCard color="primary" class="ml-20">
            <VaCardTitle style="font-size: 16px">詳細資料</VaCardTitle>
            <VaCardContent>
                <div class="text-base">
                    🆙增加、🔁重抽、2️⃣備案、📝原主人修改n次、➕其他後來增加的條件
                </div>
            </VaCardContent>
        </VaCard>
        <VaCard>
            <VaCardTitle style="font-size: 16px">完成狀態</VaCardTitle>
            <VaCardContent>
                <div class="text-base">
                    ✅完成、✅已抽、🏁原主人或投票給過、⏲️⚔️目前已完成進度
                </div>
            </VaCardContent>
        </VaCard>
    </div>
</template>
