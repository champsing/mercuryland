<script setup lang="ts">
import { ref } from "vue";
import {
    VaButton,
    VaModal,
    VaTabs,
    VaTab,
    VaCard,
    VaCardTitle,
    VaCardContent,
} from "vuestic-ui";
import { stateColor, stateString } from "@/composables/penalty";

interface ModalData {
    title: string;
    items: string[];
}

const syntaxs = [
    "🆙增加",
    "🔁重抽",
    "2️⃣備案",
    "😇復活",
    "📝修改",
    "✅已抽",
    "➕其他",
];
const states = [0, 1, 2, 3, 4].map(
    (s) =>
        "<span class='" +
        stateColor(s, "text") +
        "'>▲" +
        stateString(s) +
        "</span>",
);

const modal = ref<ModalData | null>(null);
const tabs = ref<string | null>(null);

function clickSyntax() {
    modal.value = {
        title: "詳細",
        items: syntaxs,
    };
    tabs.value = syntaxs[0];
}
function clickState() {
    modal.value = {
        title: "狀態",
        items: states,
    };
    tabs.value = states[0];
}

const footnote0 = ref(false);
</script>

<template>
    <div class="h-full">
        <VaCard
            style="--va-card-padding: 1rem"
            class="rounded-xl h-full flex flex-col"
        >
            <VaCardTitle class="!text-xl justify-center"> 圖例 </VaCardTitle>
            <VaCardContent class="flex justify-stretch gap-4 flex-1">
                <VaButton
                    class="w-full h-full"
                    gradient
                    color="#28c9c7"
                    @click="clickSyntax"
                >
                    <div class="text-xl">詳細<br />資料</div>
                </VaButton>
                <VaButton
                    class="w-full h-full"
                    gradient
                    color="#005c99"
                    @click="clickState"
                >
                    <div class="text-xl">完成<br />狀態</div>
                </VaButton>
            </VaCardContent>
        </VaCard>

        <VaModal
            :model-value="modal !== null"
            @update:model-value="
                (value) => {
                    if (!value) modal = null;
                }
            "
            size="small"
            close-button
            hide-default-actions
        >
            <template #header>
                <div class="text-lg font-semibold text-zinc-200">
                    懲罰圖例：{{ modal.title }}
                </div>
            </template>

            <VaTabs v-model="tabs" color="info" vertical grow>
                <template #tabs>
                    <VaTab v-for="t in modal.items" :key="t" :name="t">
                        <div v-html="t"></div>
                    </VaTab>
                </template>

                <div class="mt-2">
                    <p v-if="tabs === syntaxs[0]">
                        符合以下條件而新增的懲罰會被標上此標籤。<br />
                        ● 抽中「懲罰 + X」轉化的新懲罰<br />
                        ● 由「2️⃣備案」轉化而成的懲罰<br />
                        ● 由具有轉換條件<sup>1</sup>的懲罰轉化的額外懲罰<br />
                        <VaButton
                            preset="plain"
                            color="warning"
                            @click="footnote0 = true"
                        >
                            <div class="mt-2 text-sm text-left">
                                <sup>1</sup>無法完成的懲罰與轉換條件
                            </div>
                        </VaButton>
                    </p>
                    <p v-else-if="tabs === syntaxs[1]">
                        抽中「重抽」懲罰並重新抽選過的懲罰，會被標上此標籤。
                    </p>
                    <p v-else-if="tabs === syntaxs[2]">
                        惡靈怕自己電腦不好或其他突發狀況而抽出的備案。<br />
                        生效時將取代原懲罰並移除此標籤，標上「🆙增加」。
                    </p>
                    <p v-else-if="tabs === syntaxs[3]">
                        抽中「復活」懲罰並被復活的懲罰，會被標上此標籤。
                    </p>
                    <p v-else-if="tabs === syntaxs[4]">
                        原主人要求修改懲罰主文的內容。<br />
                        以「📝原主人修改n次」的方式表示，n為修改次數。
                    </p>
                    <p v-else-if="tabs === syntaxs[5]">
                        該懲罰中所產生的內容已經抽出。
                    </p>
                    <p v-else-if="tabs === syntaxs[6]">其他後來增加的條件。</p>
                    <p v-else-if="tabs === states[0]">
                        多抽出預備的懲罰，在沒被加新懲罰之前不會生效；<br />
                        加懲罰時依時間順序優先成為懲罰。
                    </p>
                    <p v-else-if="tabs === states[1]">
                        尚未開始嘗試完成該懲罰，沒有進度。
                    </p>
                    <p v-else-if="tabs === states[2]">
                        正在嘗試完成，已經有進度的懲罰。<br />
                        <br />
                        ⏲️⚔️目前進度<br />
                        「進行中」懲罰目前的進度。
                    </p>
                    <p v-else-if="tabs === states[3]">
                        在沒有完成全部條件情況下，以最低及格線通過該懲罰。<br />
                        <br />
                        🏁給過<br />
                        原主人放過惡靈一馬，或群組投票通過。<br />
                        不清楚原主人時則由群組投票，<br />
                        若之後惡靈補完成全部的條件，則一樣能夠獲得「已完成」。
                    </p>
                    <p v-else-if="tabs === states[4]">
                        已經完成該懲罰的所有條件，獲得「已完成」狀態。
                    </p>
                </div>
            </VaTabs>
        </VaModal>

        <VaModal
            v-model:model-value="footnote0"
            hide-default-actions
            close-button
        >
            <div class="text-lg font-semibold">
                無法完成的懲罰與轉換條件說明
            </div>
            <p>
                某些懲罰的主文有可能存在會造成該懲罰永遠無法完成的條件，例如時間限制等；雖然這類懲罰通常會在進入轉盤之前被過濾，但難保完全沒有。而這類懲罰在該條件無法被滿足之後會被判定失敗，永遠無法獲得<span
                    :class="stateColor(4, 'text')"
                    >▲{{ stateString(4) }}</span
                >狀態，僅可獲得<span :class="stateColor(3, 'text')"
                    >▲{{ stateString(3) }}</span
                >。除非該懲罰帶有「否則」、「失敗」等語，即視為擁有轉換條件，當前項條件無法被滿足時，會轉換為後面所說的懲罰。
            </p>
            <br />
            <p>
                假設今有一個懲罰：「2024年結束以前打完星鐵主線」；如主文所示，在2024年結束之後便永遠無法達成前項條件，此時將會判定為懲罰失敗，並將永遠無法獲得<span
                    :class="stateColor(4, 'text')"
                    >▲{{ stateString(4) }}</span
                >；唯一的完成方式是懲罰原主人或投票給過，即以<span
                    :class="stateColor(3, 'text')"
                    >▲{{ stateString(3) }}</span
                >的方式完成。
            </p>
            <br />
            <p>
                但是，若懲罰為「2024年結束以前打完星鐵主線，否則懲罰+2」，就會在判定失敗時將原懲罰取代為2個新的未抽出懲罰，並於之後由直播時抽出，原懲罰便轉化為2個新的懲罰了。
            </p>
        </VaModal>
    </div>
</template>
