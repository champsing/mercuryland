<script setup lang="ts">
import { stateColor, stateString } from "@/composables/penalty";
import { nextTabIndex } from "@/composables/tabList";
import { computed, ref } from "vue";

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

const syntaxTabId = (index: number) => `penalty-syntax-tab-${index}`;
const stateTabId = (state: number) => `penalty-state-tab-${state}`;

const activeSyntax = ref(0);
const activeState = ref(0);

const currentSyntax = computed(() => syntaxItems[activeSyntax.value]);
const currentState = computed(() => stateDescriptions[activeState.value]);

function onTabKeydown(
    event: KeyboardEvent,
    current: number,
    count: number,
    select: (index: number) => void,
) {
    const next = nextTabIndex(event.key, current, count);
    if (next === null) return;

    event.preventDefault();
    select(next);

    const tablist = (event.currentTarget as HTMLElement).parentElement;
    tablist?.querySelectorAll<HTMLElement>('[role="tab"]')[next]?.focus();
}
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

            <div
                role="tablist"
                aria-label="懲罰標籤"
                class="grid grid-cols-2 gap-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-7"
            >
                <button
                    v-for="(item, index) in syntaxItems"
                    :id="syntaxTabId(index)"
                    :key="item.label"
                    type="button"
                    role="tab"
                    aria-controls="penalty-syntax-panel"
                    :aria-selected="activeSyntax === index"
                    :tabindex="activeSyntax === index ? 0 : -1"
                    class="flex min-h-10 cursor-pointer items-center justify-center gap-1.5 rounded-xl border px-3 py-2 text-sm font-bold transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                    :class="
                        activeSyntax === index
                            ? 'border-[#28c9c7]/50 bg-[#28c9c7]/15 text-white'
                            : 'border-white/[0.08] bg-white/[0.03] text-zinc-300 hover:border-white/20 hover:bg-white/[0.07]'
                    "
                    @click="activeSyntax = index"
                    @keydown="
                        onTabKeydown(
                            $event,
                            activeSyntax,
                            syntaxItems.length,
                            (index) => (activeSyntax = index),
                        )
                    "
                >
                    {{ item.label }}
                </button>
            </div>

            <div
                id="penalty-syntax-panel"
                role="tabpanel"
                :aria-labelledby="syntaxTabId(activeSyntax)"
                tabindex="0"
                class="rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.6)] p-5 focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60 md:p-8"
            >
                <div
                    class="flex flex-col gap-4 md:flex-row md:items-center md:justify-center md:gap-6"
                >
                    <h4
                        class="text-center text-2xl font-black leading-tight text-[#f7f7f8] md:text-3xl"
                    >
                        {{ currentSyntax.label }}
                    </h4>
                    <p
                        class="max-w-prose text-center text-base leading-relaxed text-zinc-300 md:text-lg"
                        v-html="currentSyntax.description"
                    ></p>
                </div>
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

            <div
                role="tablist"
                aria-label="懲罰完成狀態"
                class="grid grid-cols-2 gap-2 sm:grid-cols-3 lg:grid-cols-5"
            >
                <button
                    v-for="(_, state) in stateDescriptions"
                    :id="stateTabId(state)"
                    :key="state"
                    type="button"
                    role="tab"
                    aria-controls="penalty-state-panel"
                    :aria-selected="activeState === state"
                    :tabindex="activeState === state ? 0 : -1"
                    class="flex min-h-10 cursor-pointer items-center justify-center rounded-xl border px-3 py-2 text-sm font-bold transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                    :class="
                        activeState === state
                            ? `${stateColor(state, 'bg')} border-transparent text-white`
                            : 'border-white/[0.08] bg-white/[0.03] hover:border-white/20 hover:bg-white/[0.07]'
                    "
                    @click="activeState = state"
                    @keydown="
                        onTabKeydown(
                            $event,
                            activeState,
                            stateDescriptions.length,
                            (state) => (activeState = state),
                        )
                    "
                >
                    <span
                        :class="
                            activeState === state
                                ? 'text-white'
                                : stateColor(state, 'text')
                        "
                    >
                        ▲{{ stateString(state) }}
                    </span>
                </button>
            </div>

            <div
                id="penalty-state-panel"
                role="tabpanel"
                :aria-labelledby="stateTabId(activeState)"
                tabindex="0"
                class="rounded-2xl border border-white/[0.08] bg-[rgba(18,21,27,0.6)] p-5 focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60 md:p-8"
            >
                <div
                    class="flex flex-col gap-4 md:flex-row md:items-center md:justify-center md:gap-6"
                >
                    <h4
                        class="text-center text-2xl font-black leading-tight md:text-3xl"
                        :class="stateColor(activeState, 'text')"
                    >
                        ▲{{ stateString(activeState) }}
                    </h4>
                    <p
                        class="max-w-prose text-center text-base leading-relaxed text-zinc-300 md:text-lg"
                        v-html="currentState"
                    ></p>
                </div>
            </div>
        </section>

        <section
            aria-labelledby="penalty-legend-conversion"
            class="rounded-2xl border border-[#f59e0b]/30 bg-[rgba(245,158,11,0.06)] p-4"
        >
            <h3
                id="penalty-legend-conversion"
                class="text-base font-extrabold text-[#f7f7f8]"
            >
                <sup class="text-[#f59e0b]">1</sup>
                無法轉換的懲罰與轉換條件
            </h3>
            <p class="mt-3 text-sm leading-relaxed text-zinc-300">
                主文若有「否則」、「失敗」等字，判定失敗時會轉換成新的懲罰；沒有的話，就只能以
                <span :class="stateColor(3, 'text')"
                    >▲{{ stateString(3) }}</span
                >
                結案。
            </p>
            <figure class="mt-4">
                <picture>
                    <source
                        media="(max-width: 1023px)"
                        srcset="/images/penalty_conversion_mobile.svg"
                    />
                    <img
                        src="/images/penalty_conversion.svg"
                        alt="圖例：沒有轉換條件的懲罰在期限過後判定失敗，只能拿到勉強過；有「否則懲罰 +2」轉換條件的懲罰，失敗後會生成兩個新懲罰，之後由直播抽出。"
                        class="mx-auto block h-auto w-full max-w-xl rounded-xl border border-white/10 lg:max-w-none"
                        decoding="async"
                    />
                </picture>
                <figcaption
                    class="mt-2 text-center text-xs text-zinc-400 lg:text-left"
                >
                    紅：沒有轉換條件；綠：有轉換條件。
                </figcaption>
            </figure>
        </section>
    </div>
</template>
