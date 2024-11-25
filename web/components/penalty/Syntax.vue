<script setup lang="ts">
import { ref } from "vue";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaCardTitle,
    VaDivider,
    VaIcon,
    VaModal,
} from "vuestic-ui";
import penaltyStatus from "@assets/data/penalty_status.json";

function colorOfStatus(status: string): string {
    return penaltyStatus.filter((x) => x.name == status)[0]?.color;
}

const statusColorSet = [
    colorOfStatus("未開始"),
    colorOfStatus("已完成"),
    colorOfStatus("勉強過"),
    colorOfStatus("進行中"),
];

const additionalMetaData = ref(false);
const statusMetaData = ref(false);
const showExtraConditionDesc = ref(false);
</script>

<template>
    <div class="text-4xl text-neutral-100 text-left mt-4">圖例</div>
    <VaDivider class="!mt-3" />
    <div class="text-sm text-neutral-100 ml-6 mb-3 text-center">
        點擊卡片以查看更多資訊
    </div>
    <div class="flex flex-row justify-center gap-3" item-responsive>
        <VaCard
            gradient
            color="#28c9c7"
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
                    <div
                        v-for="color in statusColorSet"
                        :class="`inline bg-black !text-[${color}]`"
                    >
                        ▲
                    </div>
                    四大完成狀態、✅已抽、🏁給過、⏲️⚔️目前進度
                </div>
            </VaCardContent>
        </VaCard>
    </div>

    <VaModal v-model="additionalMetaData" hide-default-actions close-button>
        <div class="text-base">
            🆙增加：符合以下條件而新增的懲罰會被標上此標籤。
            <ol>
                <li>
                    <!-- 1. -->
                    抽中「懲罰 + X」轉化的新懲罰
                </li>
                <li>
                    <!-- 2. -->
                    由2️⃣備案轉化而成的懲罰
                </li>
                <li>
                    <!-- 3. -->
                    由具有轉換條件*的懲罰轉化的額外懲罰
                </li>
            </ol>
            <VaButton
                preset="plain"
                color="warning"
                @click="showExtraConditionDesc = !showExtraConditionDesc"
            >
                <div class="mt-2 mb-4 text-sm text-left">
                    <VaIcon name="help_outline" />
                    無法完成的懲罰與轉換條件
                </div>
            </VaButton>
            <VaModal
                v-model:model-value="showExtraConditionDesc"
                hide-default-actions
                close-button
            >
                <div class="text-2xl mb-2">無法完成的懲罰與轉換條件說明</div>
                某些懲罰的主文有可能存在會造成該懲罰永遠無法完成的條件，例如時間限制等；雖然這類懲罰通常會在進入轉盤之前被過濾，但難保完全沒有。而這類懲罰在該條件無法被滿足之後會被判定失敗，永遠無法獲得
                <div :class="`inline !text-[${statusColorSet[1]}]`">
                    ▲已完成
                </div>
                狀態，僅可獲得
                <div :class="`inline !text-[${statusColorSet[2]}]`">
                    ▲勉強過
                </div>
                。
                除非該懲罰帶有「否則」、「失敗」等語，即視為擁有轉換條件，當前項條件無法被滿足時，會轉換為後面所說的懲罰。
                <br />
                <br />
                假設今有一個懲罰：「2024年結束以前打完星鐵主線」；如主文所示，在2024年結束之後便永遠無法達成前項條件，此時將會判定為懲罰失敗，並將永遠無法獲得
                <div :class="`inline !text-[${statusColorSet[1]}]`">
                    ▲已完成
                </div>
                ；唯一的完成方式是懲罰原主人或投票給過，即以
                <div :class="`inline !text-[${statusColorSet[2]}]`">
                    ▲勉強過
                </div>
                方式完成。
                <br />
                但是，若懲罰為「2024年結束以前打完星鐵主線，否則懲罰+2」，就會在判定失敗時將原懲罰取代為2個新的未抽出懲罰，並於之後由直播時抽出，原懲罰便轉化為2個新的懲罰了。
            </VaModal>

            🔁重抽：抽中「重抽」懲罰並重新抽選過的懲罰，會被標上此標籤
            <br />
            <br />
            2️⃣備案：惡靈怕自己電腦不好或其他突發狀況而抽出的備案，生效時將取代原懲罰並移除此標籤，標上🆙增加。
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
    <VaModal v-model="statusMetaData" hide-default-actions close-button>
        <div class="text-base">
            <div :class="`inline !text-[${statusColorSet[0]}]`">▲未開始</div>
            ：尚未開始嘗試完成該懲罰，沒有進度
            <br />
            <div :class="`inline !text-[${statusColorSet[1]}]`">▲已完成</div>
            ：已經完成該懲罰主文要求的全部條件
            <br />
            <div :class="`inline !text-[${statusColorSet[2]}]`">▲勉強過</div>
            ：該懲罰的原主人決定讓惡靈在沒有完成全部條件情況下，直接完成該懲罰
            <br />
            （不清楚原主人時則由群組投票）
            <br />
            <div :class="`inline !text-[${statusColorSet[3]}]`">▲進行中</div>
            ：正在嘗試完成，已經有進度的懲罰
            <br />
            <br />
            ✅已抽：該懲罰中所產生的內容已經抽出
            <br />
            <br />
            🏁原主人或投票給過：
            <div :class="`inline !text-[${statusColorSet[2]}]`">▲勉強過</div>
            懲罰成立的原因
            <br />
            <br />
            ⏲️⚔️目前已完成進度：
            <div :class="`inline !text-[${statusColorSet[3]}]`">▲進行中</div>
            懲罰目前的進度
        </div>
    </VaModal>
</template>
