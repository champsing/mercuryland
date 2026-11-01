<script setup lang="ts">
import { stateColor, stateString } from "@/composables/penalty";

interface LegendItem {
    label: string;
    description: string;
}

const syntaxItems: LegendItem[] = [
    {
        label: "🆙增加",
        description:
            "符合以下條件而新增的懲罰會被標上此標籤。<br />● 抽中「懲罰 + X」轉化的新懲罰<br />● 由「2️⃣備案」轉化而成的懲罰<br />● 由具有轉換條件<sup>1</sup>的懲罰轉化的額外懲罰",
    },
    {
        label: "🔁重抽",
        description: "抽中「重抽」懲罰並重新抽選過的懲罰，會被標上此標籤。",
    },
    {
        label: "2️⃣備案",
        description:
            "惡靈怕自己電腦不好或其他突發狀況而抽出的備案。<br />生效時將取代原懲罰並移除此標籤，標上「🆙增加」。",
    },
    {
        label: "😇復活",
        description: "抽中「復活」懲罰並被復活的懲罰，會被標上此標籤。",
    },
    {
        label: "📝修改",
        description:
            "原主人要求修改懲罰主文的內容。<br />以「📝原主人修改n次」的方式表示，n為修改次數。",
    },
    {
        label: "✅已抽",
        description: "該懲罰中所產生的內容已經抽出。",
    },
    {
        label: "➕其他",
        description: "其他後來增加的條件。",
    },
];

const stateDescriptions = [
    "多抽出預備的懲罰，在沒被加新懲罰之前不會生效；<br />加懲罰時依時間順序優先成為懲罰。",
    "尚未開始嘗試完成該懲罰，沒有進度。",
    "正在嘗試完成，已經有進度的懲罰。<br /><br />⏲️⚔️目前進度<br />「進行中」懲罰目前的進度。",
    "在沒有完成全部條件情況下，以最低及格線通過該懲罰。<br /><br />🏁給過<br />原主人放過惡靈一馬，或群組投票通過。<br />不清楚原主人時則由群組投票，<br />若之後惡靈補完成全部的條件，則一樣能夠獲得「已完成」。",
    "已經完成該懲罰的所有條件，獲得「已完成」狀態。",
];

const stateItems: LegendItem[] = stateDescriptions.map(
    (description, state) => ({
        label: `<span class="${stateColor(state, "text")}">▲${stateString(state)}</span>`,
        description,
    }),
);
</script>

<template>
    <div class="flex flex-col gap-6 pb-4">
        <div class="flex flex-wrap items-baseline gap-x-2">
            <h2 class="text-xl font-bold text-neutral-100">圖例</h2>
            <span
                class="text-[0.72rem] font-extrabold uppercase text-[#28c9c7]"
            >
                Icons
            </span>
        </div>

        <section
            aria-labelledby="penalty-legend-syntax"
            class="flex flex-col gap-3"
        >
            <header class="flex flex-wrap items-baseline gap-x-2">
                <h3
                    id="penalty-legend-syntax"
                    class="text-base font-extrabold text-[#f7f7f8]"
                >
                    詳細資料
                </h3>
                <span
                    class="text-[0.72rem] font-extrabold uppercase text-[#28c9c7]"
                >
                    Syntax
                </span>
            </header>
            <div class="grid gap-3 sm:grid-cols-2 xl:grid-cols-3">
                <article
                    v-for="item in syntaxItems"
                    :key="item.label"
                    class="rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.6)] p-4"
                >
                    <h4 class="text-base font-extrabold text-[#f7f7f8]">
                        {{ item.label }}
                    </h4>
                    <p
                        class="mt-2 text-sm leading-relaxed text-zinc-300"
                        v-html="item.description"
                    ></p>
                </article>
            </div>
        </section>

        <section
            aria-labelledby="penalty-legend-states"
            class="flex flex-col gap-3"
        >
            <header class="flex flex-wrap items-baseline gap-x-2">
                <h3
                    id="penalty-legend-states"
                    class="text-base font-extrabold text-[#f7f7f8]"
                >
                    完成狀態
                </h3>
                <span
                    class="text-[0.72rem] font-extrabold uppercase text-[#7cc4ff]"
                >
                    States
                </span>
            </header>
            <div class="grid gap-3 sm:grid-cols-2 xl:grid-cols-3">
                <article
                    v-for="item in stateItems"
                    :key="item.label"
                    class="rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.6)] p-4"
                >
                    <h4
                        class="text-base font-extrabold"
                        v-html="item.label"
                    ></h4>
                    <p
                        class="mt-2 text-sm leading-relaxed text-zinc-300"
                        v-html="item.description"
                    ></p>
                </article>
            </div>
        </section>

        <section
            aria-labelledby="penalty-legend-footnote"
            class="rounded-2xl border border-[#f59e0b]/30 bg-[rgba(245,158,11,0.06)] p-4"
        >
            <h3
                id="penalty-legend-footnote"
                class="text-base font-extrabold text-[#f7f7f8]"
            >
                <sup class="text-[#f59e0b]">1</sup>
                無法完成的懲罰與轉換條件說明
            </h3>
            <div
                class="mt-3 flex flex-col gap-3 text-sm leading-relaxed text-zinc-300"
            >
                <p>
                    某些懲罰的主文有可能存在會造成該懲罰永遠無法完成的條件，例如時間限制等；雖然這類懲罰通常會在進入轉盤之前被過濾，但難保完全沒有。而這類懲罰在該條件無法被滿足之後會被判定失敗，永遠無法獲得<span
                        :class="stateColor(4, 'text')"
                        >▲{{ stateString(4) }}</span
                    >狀態，僅可獲得<span :class="stateColor(3, 'text')"
                        >▲{{ stateString(3) }}</span
                    >。除非該懲罰帶有「否則」、「失敗」等語，即視為擁有轉換條件，當前項條件無法被滿足時，會轉換為後面所說的懲罰。
                </p>
                <p>
                    假設今有一個懲罰：「2024年結束以前打完星鐵主線」；如主文所示，在2024年結束之後便永遠無法達成前項條件，此時將會判定為懲罰失敗，並將永遠無法獲得<span
                        :class="stateColor(4, 'text')"
                        >▲{{ stateString(4) }}</span
                    >；唯一的完成方式是懲罰原主人或投票給過，即以<span
                        :class="stateColor(3, 'text')"
                        >▲{{ stateString(3) }}</span
                    >的方式完成。
                </p>
                <p>
                    但是，若懲罰為「2024年結束以前打完星鐵主線，否則懲罰+2」，就會在判定失敗時將原懲罰取代為2個新的未抽出懲罰，並於之後由直播時抽出，原懲罰便轉化為2個新的懲罰了。
                </p>
            </div>
        </section>
    </div>
</template>
