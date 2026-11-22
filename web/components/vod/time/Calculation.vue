<script setup lang="ts">
import {
    CalculationEntry,
    CalculationGroup,
    formatHMS,
    groupCalculationEntries,
    groupCalculationMonths,
    isoWeekNumber,
    summarizeCalculation,
} from "@composables/vod";
import { ChevronDown24Regular } from "@vicons/fluent";
import { computed, nextTick, onBeforeUnmount, ref, watch } from "vue";

const props = defineProps<{ entries: CalculationEntry[] }>();

const scrollRef = ref<HTMLElement | null>(null);

const groups = computed(() => groupCalculationEntries(props.entries));
const months = computed(() => groupCalculationMonths(groups.value));
const totals = computed(() => summarizeCalculation(props.entries));

const balanceClass = computed(() => {
    if (totals.value.remaining < 0) return "text-rose-200";
    if (totals.value.remaining > 0) return "text-emerald-200";
    return "text-zinc-100";
});

const years = computed(() => [
    ...new Set(months.value.map((month) => month.key.slice(0, 4))),
]);

const selectedYear = ref("");
const selectedMonth = ref("");
const visibleMonth = ref("");
const indexExpanded = ref(false);

const spotlightLabel = computed(() => {
    const month = months.value.find((item) => item.key === visibleMonth.value);
    return month?.label ?? months.value[months.value.length - 1]?.label ?? "";
});

const availableMonths = computed(() => {
    const available = new Set<number>();
    for (const month of months.value) {
        if (month.key.startsWith(selectedYear.value + "-")) {
            available.add(Number(month.key.slice(5)));
        }
    }
    return available;
});

function monthKey(month: number): string {
    return `${selectedYear.value}-${String(month).padStart(2, "0")}`;
}

function syncIndexToLatest() {
    const latest = months.value[months.value.length - 1];
    if (!latest) return;
    selectedYear.value = latest.key.slice(0, 4);
    selectedMonth.value = latest.key;
    visibleMonth.value = latest.key;
}

function scrollToMonth(key: string) {
    selectedMonth.value = key;
    selectedYear.value = key.slice(0, 4);

    const container = scrollRef.value;
    const target = container?.querySelector<HTMLElement>(
        `[data-month="${key}"]`,
    );
    if (!container || !target) return;

    const top =
        target.getBoundingClientRect().top -
        container.getBoundingClientRect().top +
        container.scrollTop;

    container.scrollTo({ top, behavior: "smooth" });
}

function selectYear(year: string) {
    const first = months.value.find((month) =>
        month.key.startsWith(year + "-"),
    );
    if (first) scrollToMonth(first.key);
}

function selectMonth(month: number) {
    scrollToMonth(monthKey(month));
}

function toggleIndex() {
    if (!indexExpanded.value) {
        // Align the index with the month currently in view.
        const current = visibleMonth.value || selectedMonth.value;
        if (current) {
            selectedYear.value = current.slice(0, 4);
            selectedMonth.value = current;
        }
    }

    indexExpanded.value = !indexExpanded.value;
}

function updateSpotlight() {
    const container = scrollRef.value;
    if (!container) return;

    const sections = container.querySelectorAll<HTMLElement>("[data-month]");
    const containerTop = container.getBoundingClientRect().top;
    let current = sections[0]?.dataset.month ?? "";

    for (const section of sections) {
        if (section.getBoundingClientRect().top > containerTop + 1) break;
        current = section.dataset.month ?? current;
    }

    if (!current) return;

    visibleMonth.value = current;

    if (indexExpanded.value) {
        selectedYear.value = current.slice(0, 4);
        selectedMonth.value = current;
    }
}

let spotlightFrame = 0;

function onScroll() {
    if (spotlightFrame) return;
    spotlightFrame = requestAnimationFrame(() => {
        spotlightFrame = 0;
        updateSpotlight();
    });
}

function showLatest() {
    const container = scrollRef.value;
    if (!container) return;

    syncIndexToLatest();
    container.scrollTop = container.scrollHeight;
    updateSpotlight();
}

// The ledger spans years; land on the latest week when the tab opens (or when
// VOD data finishes loading after it).
const syncedLatest = ref(false);

watch(
    months,
    async (value) => {
        const monthExists = value.some(
            (month) => month.key === selectedMonth.value,
        );
        const yearExists =
            selectedYear.value !== "" &&
            value.some((month) =>
                month.key.startsWith(selectedYear.value + "-"),
            );

        if (!monthExists || !yearExists) syncIndexToLatest();

        if (value.length === 0 || syncedLatest.value) return;

        syncedLatest.value = true;
        await nextTick();
        showLatest();
    },
    { immediate: true },
);

onBeforeUnmount(() => {
    if (spotlightFrame) cancelAnimationFrame(spotlightFrame);
});

function format(seconds: number): string {
    if (seconds > 0) return "+ " + formatHMS(seconds);
    if (seconds < 0) return "− " + formatHMS(-seconds);
    return formatHMS(0);
}

function formatBalance(seconds: number): string {
    return seconds < 0 ? "− " + formatHMS(-seconds) : formatHMS(seconds);
}

function amountClass(seconds: number): string {
    if (seconds > 0) return "text-emerald-300";
    if (seconds < 0) return "text-rose-300";
    return "text-zinc-300";
}

function weekTotal(group: CalculationGroup): number {
    return group.entries.reduce((total, entry) => total + entry.offset, 0);
}

function weekRange(group: CalculationGroup): string {
    // A group without a 計劃 is a week clipped by the chosen date range.
    if (!group.entries[0]?.divider) return group.start;

    const end = new Date(group.start + "T00:00:00");
    end.setDate(end.getDate() + 6);
    const month = String(end.getMonth() + 1).padStart(2, "0");
    const day = String(end.getDate()).padStart(2, "0");
    const endLabel =
        end.getFullYear() === Number(group.start.slice(0, 4))
            ? `${month}-${day}`
            : `${end.getFullYear()}-${month}-${day}`;

    return `${group.start} – ${endLabel}`;
}
</script>

<template>
    <div class="flex flex-col gap-3 md:h-full md:min-h-0">
        <section
            aria-labelledby="vod-hours-balance"
            class="relative overflow-hidden rounded-3xl border border-white/[0.08] bg-gradient-to-br from-[#0f3b35]/95 via-[#0c2b33]/92 to-[#0b141c]/95 shadow-[0_2px_12px_rgba(0,0,0,0.3)]"
        >
            <div
                class="pointer-events-none absolute -right-24 -top-28 h-64 w-64 rounded-full bg-[#5bc6a1]/15 blur-3xl"
                aria-hidden="true"
            />
            <div
                class="pointer-events-none absolute -bottom-28 left-1/3 h-56 w-56 rounded-full bg-rose-400/10 blur-3xl"
                aria-hidden="true"
            />

            <div
                class="relative flex flex-col gap-4 p-4 md:p-5 lg:flex-row lg:items-center lg:justify-between lg:gap-8"
            >
                <div class="min-w-0">
                    <p
                        id="vod-hours-balance"
                        class="text-[0.7rem] font-semibold uppercase tracking-[0.3em] text-emerald-200/70"
                    >
                        剩餘時間
                    </p>
                    <div
                        class="mt-1 flex flex-wrap items-baseline gap-x-3 gap-y-1"
                    >
                        <span
                            class="text-4xl font-black leading-none tracking-tight tabular-nums md:text-6xl"
                            :class="balanceClass"
                        >
                            {{ formatBalance(totals.remaining) }}
                        </span>
                        <span class="text-xs text-zinc-400">加班台時數</span>
                    </div>
                </div>

                <div class="grid gap-2 sm:grid-cols-2 lg:w-[27rem] lg:shrink-0">
                    <div
                        class="rounded-2xl border border-emerald-300/20 bg-emerald-400/[0.08] px-3 py-2.5"
                    >
                        <p
                            class="flex items-center gap-1 text-xs font-semibold text-emerald-300"
                        >
                            <span
                                class="text-base leading-none"
                                aria-hidden="true"
                                >＋</span
                            >
                            現在應付的直播時數
                        </p>
                        <p
                            class="mt-1.5 text-lg font-bold leading-none tabular-nums text-emerald-100"
                        >
                            {{ formatHMS(totals.owed) }}
                        </p>
                        <p class="mt-1 text-[0.65rem] text-emerald-200/50">
                            計劃 ＋ 懲罰 ＋ 課金
                        </p>
                    </div>

                    <div
                        class="rounded-2xl border border-rose-300/20 bg-rose-400/[0.08] px-3 py-2.5"
                    >
                        <p
                            class="flex items-center gap-1 text-xs font-semibold text-rose-300"
                        >
                            <span
                                class="text-base leading-none"
                                aria-hidden="true"
                                >−</span
                            >
                            現在已付的時間
                        </p>
                        <p
                            class="mt-1.5 text-lg font-bold leading-none tabular-nums text-rose-100"
                        >
                            {{ formatHMS(totals.paid) }}
                        </p>
                        <p class="mt-1 text-[0.65rem] text-rose-200/50">
                            直播時長
                        </p>
                    </div>
                </div>
            </div>
        </section>

        <section
            aria-labelledby="vod-hours-ledger"
            class="flex min-h-0 flex-col overflow-hidden rounded-3xl border border-white/[0.08] bg-[rgba(18,21,27,0.65)] shadow-[0_2px_12px_rgba(0,0,0,0.3)] md:flex-1"
        >
            <header
                class="flex flex-wrap items-center justify-between gap-2 border-b border-white/[0.06] px-3 py-2.5 md:px-4"
            >
                <div class="flex min-w-0 flex-wrap items-baseline gap-x-2">
                    <h2
                        id="vod-hours-ledger"
                        class="text-base font-semibold text-zinc-100"
                    >
                        計算明細
                    </h2>
                    <span
                        class="text-[0.65rem] font-bold uppercase tracking-[0.18em] text-emerald-300/70"
                    >
                        Ledger
                    </span>
                </div>
                <button
                    type="button"
                    class="flex cursor-pointer items-center gap-1.5 rounded-full px-2 py-1 text-xs text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                    :aria-expanded="indexExpanded"
                    aria-controls="vod-hours-month-index"
                    @click="toggleIndex"
                >
                    <span class="tabular-nums">{{ spotlightLabel }}</span>
                    <ChevronDown24Regular
                        class="h-3.5 w-3.5 shrink-0 text-zinc-400 transition-transform"
                        :class="{ 'rotate-180': indexExpanded }"
                    />
                </button>
            </header>

            <div
                v-show="indexExpanded"
                id="vod-hours-month-index"
                class="flex flex-col gap-1.5 border-b border-white/[0.06] bg-white/[0.02] px-3 py-2"
            >
                <div
                    class="flex flex-wrap items-center gap-1"
                    role="group"
                    aria-label="年份"
                >
                    <button
                        v-for="year in years"
                        :key="year"
                        type="button"
                        :aria-current="
                            year === selectedYear ? 'true' : undefined
                        "
                        class="cursor-pointer rounded-full px-2.5 py-0.5 text-xs tabular-nums transition-colors"
                        :class="
                            year === selectedYear
                                ? 'bg-white/15 text-zinc-100'
                                : 'text-zinc-400 hover:bg-white/10 hover:text-zinc-200'
                        "
                        @click="selectYear(year)"
                    >
                        {{ year }}
                    </button>
                </div>
                <div
                    class="flex flex-wrap items-center gap-1"
                    role="group"
                    aria-label="月份"
                >
                    <button
                        v-for="month in 12"
                        :key="month"
                        type="button"
                        :disabled="!availableMonths.has(month)"
                        :aria-current="
                            monthKey(month) === selectedMonth
                                ? 'true'
                                : undefined
                        "
                        class="min-w-8 rounded-full px-2 py-0.5 text-xs tabular-nums transition-colors disabled:cursor-not-allowed"
                        :class="
                            monthKey(month) === selectedMonth
                                ? 'bg-white/15 text-zinc-100'
                                : availableMonths.has(month)
                                  ? 'cursor-pointer text-zinc-300 hover:bg-white/10 hover:text-zinc-100'
                                  : 'text-zinc-600'
                        "
                        @click="selectMonth(month)"
                    >
                        {{ month }}月
                    </button>
                </div>
            </div>

            <div
                v-if="months.length > 0"
                ref="scrollRef"
                role="region"
                tabindex="0"
                aria-label="計算明細列表"
                class="custom-scrollbar max-h-[55vh] min-h-0 overflow-y-auto overscroll-contain px-2 py-2 outline-none focus-visible:ring-2 focus-visible:ring-white/30 md:max-h-none md:flex-1 md:px-3"
                @scroll="onScroll"
            >
                <div class="flex min-w-0 flex-col gap-4">
                    <section
                        v-for="month in months"
                        :key="month.key"
                        :data-month="month.key"
                        class="flex min-w-0 flex-col gap-2"
                    >
                        <h3
                            class="sticky top-0 z-10 rounded-md bg-[#12151b]/95 px-1 py-1 text-sm font-semibold text-zinc-100 backdrop-blur-sm"
                        >
                            {{ month.label }}
                        </h3>

                        <section
                            v-for="(group, groupIndex) in month.groups"
                            :key="month.key + groupIndex + group.start"
                            class="min-w-0 overflow-hidden rounded-xl border border-white/[0.08] bg-white/[0.03]"
                        >
                            <header
                                class="flex items-center justify-between gap-2 bg-white/[0.04] px-3 py-2"
                            >
                                <div class="flex min-w-0 items-center gap-2">
                                    <span
                                        class="shrink-0 rounded-md bg-white/10 px-1.5 py-0.5 text-[0.65rem] font-semibold tracking-wide text-zinc-300"
                                    >
                                        第{{ isoWeekNumber(group.start) }}週
                                    </span>
                                    <h4
                                        class="truncate text-sm font-semibold tabular-nums text-zinc-100"
                                    >
                                        {{ weekRange(group) }}
                                    </h4>
                                </div>
                                <span
                                    class="shrink-0 text-sm font-semibold tabular-nums text-zinc-100"
                                >
                                    <span class="sr-only">該週開始剩餘 </span>
                                    {{ formatBalance(group.previous) }}
                                </span>
                            </header>

                            <ul class="divide-y divide-white/[0.06]">
                                <li
                                    v-for="(entry, index) in group.entries"
                                    :key="group.start + index + entry.reason"
                                    class="flex items-baseline gap-3 px-3 py-2"
                                >
                                    <span
                                        class="w-11 shrink-0 text-xs tabular-nums text-zinc-400"
                                    >
                                        {{ entry.date.slice(5) }}
                                    </span>
                                    <span
                                        class="min-w-0 flex-1 break-words text-sm text-zinc-300"
                                    >
                                        {{ entry.reason }}
                                    </span>
                                    <span
                                        class="shrink-0 text-sm font-semibold tabular-nums"
                                        :class="amountClass(entry.offset)"
                                    >
                                        {{ format(entry.offset) }}
                                    </span>
                                </li>
                            </ul>

                            <footer
                                class="flex items-baseline justify-between gap-3 border-t border-white/[0.06] bg-white/[0.02] px-3 py-1.5"
                            >
                                <span class="text-xs text-zinc-400"
                                    >本週合計</span
                                >
                                <span
                                    class="text-xs font-semibold tabular-nums"
                                    :class="amountClass(weekTotal(group))"
                                >
                                    {{ format(weekTotal(group)) }}
                                </span>
                            </footer>
                        </section>
                    </section>
                </div>
            </div>

            <div
                v-else
                class="flex flex-1 flex-col items-center justify-center gap-1 px-4 py-12 text-zinc-500"
            >
                <span class="text-2xl" aria-hidden="true">🧾</span>
                <p class="text-sm">暫無計算明細</p>
            </div>
        </section>
    </div>
</template>

<style scoped>
/* Custom Scrollbar */
.custom-scrollbar {
    /* Keep the ledger's tall content from enlarging the document's scroll
       range; without paint containment the page can scroll far into blank
       space below the footer. */
    contain: paint;
}
.custom-scrollbar::-webkit-scrollbar {
    width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
    @apply bg-slate-500/20 hover:bg-slate-500/40 dark:bg-slate-400/15 dark:hover:bg-slate-400/35 rounded-full;
}
</style>
