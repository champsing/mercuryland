<script setup lang="ts">
import { ref } from "vue";
import {
    VaCard,
    VaCardContent,
    VaCardTitle,
    VaDivider,
    VaModal,
} from "vuestic-ui";

const additionalMetaData = ref(false);
const statusMetaData = ref(false);
</script>

<template>
    <div class="text-4xl text-neutral-100 text-left mt-4">圖例</div>
    <VaDivider class="!mt-3" />
    <div class="m-auto flex flex-row justify-center gap-5" item-responsive>
        <VaCard
            gradient
            color="#28c9c7"
            class="ml-2"
            @click="additionalMetaData = !additionalMetaData"
        >
            <VaCardTitle style="font-size: 16px">詳細資料</VaCardTitle>
            <VaCardContent>
                <div class="text-base">
                    🆙增加、🔁重抽、2️⃣備案、😇復活、📝原主人修改、➕其他
                </div>
            </VaCardContent>
        </VaCard>
        <VaCard
            gradient
            color="#005c99"
            @click="statusMetaData = !statusMetaData"
        >
            <VaCardTitle style="font-size: 16px">完成狀態</VaCardTitle>
            <VaCardContent>
                <div class="text-base">
                    <div class="inline bg-black !text-[#b91c1c]">▲</div>
                    <div class="inline bg-black !text-[#4d7c0f]">▲</div>
                    <div class="inline bg-black !text-[#047857]">▲</div>
                    <div class="inline bg-black !text-[#b45309]">▲</div>
                    四大完成狀態、✅已抽、🏁給過、⏲️⚔️目前進度
                </div>
            </VaCardContent>
        </VaCard>
    </div>

    <VaModal v-model="additionalMetaData" hide-default-actions>
        <div class="text-base">
            🆙增加：抽中「懲罰 + X」所轉化的額外懲罰，會被標上此標籤
            <br />
            <br />
            🔁重抽：抽中「重抽」懲罰並重新抽選過的懲罰，會被標上此標籤
            <br />
            <br />
            2️⃣備案：惡靈怕自己電腦不好或其他突發狀況而抽出的備案，生效時將取代原懲罰並移除此標籤
            <br />
            <br />
            😇復活：抽中「復活」懲罰並被復活的懲罰，會被標上此標籤
            <br />
            <br />
            📝原主人修改n次：原主人要求修改懲罰主文的內容。
            <br />
            <br />
            ➕其他：其他後來增加的條件
        </div>
    </VaModal>
    <VaModal v-model="statusMetaData" hide-default-actions>
        <div class="text-base">
            <div class="inline !text-[#b91c1c]">▲未開始</div>
            ：尚未開始嘗試完成該懲罰，沒有進度
            <br />
            <div class="inline !text-[#4d7c0f]">▲已完成</div>
            ：已經完成該懲罰主文要求的全部條件
            <br />
            <div class="inline !text-[#047857]">▲勉強過</div>
            ：該懲罰的原主人決定讓惡靈在沒有完成全部條件情況下，直接完成該懲罰
            <br />
            （不清楚原主人時則由群組投票）
            <br />
            <div class="inline !text-[#b45309]">▲進行中</div>
            ：正在嘗試完成，已經有進度的懲罰
            <br />
            <br />
            ✅已抽：該懲罰中所產生的內容已經抽出
            <br />
            <br />
            🏁原主人或投票給過：
            <div class="inline !text-[#047857]">▲勉強過</div>
            懲罰成立的原因
            <br />
            <br />
            ⏲️⚔️目前已完成進度：
            <div class="inline !text-[#b45309]">▲進行中</div>
            懲罰目前的進度
        </div>
    </VaModal>
</template>
