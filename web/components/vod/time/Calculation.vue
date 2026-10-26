<script setup lang="ts">
import vodSchedule from "@assets/data/schedule.json";
import {
    CalculationEntry,
    CalculationGroup,
    formatHMS,
    groupCalculationEntries,
    groupCalculationMonths,
    isoWeekNumber,
    parseHMS,
    VodItem,
} from "@composables/vod";
import { ChevronDown24Regular } from "@vicons/fluent";
import { computed, nextTick, ref, watch } from "vue";
import { VaModal } from "vuestic-ui";

const props = defineProps<{
    dateRange: { start: Date; end: Date };
    vodData: VodItem[];
}>();
const emit = defineEmits<{ (e: "computedTime", tag: number): void }>();

const showModal = ref(false);
const scrollRef = ref<HTMLElement | null>(null);

function open() {
    showModal.value = true;
}

defineExpose({ open });

const rawData = computed(() => calcRawData(props.vodData));
const data = computed(() =>
    rawData.value.filter(
        (v: CalculationEntry) =>
            v.date >= props.dateRange.start.toISOString().slice(0, 10) &&
            v.date <=
                new Date(props.dateRange.end.getTime() + 28800000)
                    .toISOString()
                    .slice(0, 10),
    ),
);

const groups = computed(() => groupCalculationEntries(data.value));
const months = computed(() => groupCalculationMonths(groups.value));

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

watch(
    months,
    () => {
        const monthExists = months.value.some(
            (month) => month.key === selectedMonth.value,
        );
        const yearExists =
            selectedYear.value !== "" &&
            months.value.some((month) =>
                month.key.startsWith(selectedYear.value + "-"),
            );

        if (!monthExists || !yearExists) syncIndexToLatest();
    },
    { immediate: true },
);

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

// The ledger spans years; land on the most recent entry instead of 2023.
watch(showModal, async (visible) => {
    if (!visible) return;
    await nextTick();
    syncIndexToLatest();
    const container = scrollRef.value;
    if (container) {
        container.scrollTop = container.scrollHeight;
        updateSpotlight();
    }
});

watch(
    rawData,
    (value) => {
        const lastItem = value[value.length - 1];
        const total = lastItem ? lastItem.previous + lastItem.offset : 0;
        emit("computedTime", total);
    },
    { immediate: true },
);

function calcRawData(vodItems: VodItem[]): CalculationEntry[] {
    let re: CalculationEntry[] = [];

    let sch = vodSchedule.schedule;
    // Sort overrides so every entry is emitted in date order — the per-date
    // grouping below relies on chronological, adjacent dates.
    let ove = vodSchedule.override
        .slice()
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

    let s = 0;
    let o = 0;
    let v = 0;

    let now = new Date(Date.now());
    let date: Date = new Date(
        Math.min(
            (sch.length == 0 ? now : new Date(sch[0].date)).getTime(),
            (ove.length == 0 ? now : new Date(ove[0].date)).getTime(),
        ),
    );
    let vod = vodItems
        .filter((x) => x.date > date.toISOString().slice(0, 10))
        .slice()
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

    let previous = parseHMS(vodSchedule.initial);

    while (date < now) {
        if (
            s + 1 < sch.length &&
            sch[s + 1].date >= date.toISOString().slice(0, 10)
        ) {
            s = s + 1;
        }
        re.push({
            date: date.toISOString().slice(0, 10),
            offset: parseHMS(sch[s].duration),
            previous: previous,
            reason: "計劃",
            divider: true,
        });
        previous = previous + re[re.length - 1].offset;

        date.setDate(date.getDate() + 7);

        // Collect this week's live streams and overrides into one list so the
        // display stays chronological regardless of which source comes first.
        const weekEntries: CalculationEntry[] = [];

        while (v < vod.length && new Date(vod[v].date) < date) {
            weekEntries.push({
                date: vod[v].date,
                offset: -parseHMS(vod[v].duration),
                previous: previous,
                reason: "直播",
                divider: false,
            });

            v = v + 1;
        }

        while (o < ove.length && new Date(ove[o].date) < date) {
            weekEntries.push({
                date: ove[o].date,
                offset: parseHMS(ove[o].duration),
                previous: previous,
                reason: ove[o].reason,
                divider: false,
            });

            o = o + 1;
        }

        weekEntries.sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));

        for (const entry of weekEntries) {
            entry.previous = previous;
            previous = previous + entry.offset;
            re.push(entry);
        }
    }
    return re;
}

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
    <VaModal
        aria-label="計算明細"
        v-model="showModal"
        hide-default-actions
        close-button
        :mobile-fullscreen="false"
        max-width="720px"
        class="calculation-modal"
    >
        <div class="flex w-full min-w-0 flex-col gap-3">
            <div class="flex flex-wrap items-center justify-between gap-2 pr-8">
                <h2 class="text-lg font-semibold text-zinc-100">計算明細</h2>
                <div class="flex items-center gap-3 text-xs">
                    <span
                        class="inline-flex items-center gap-1.5 text-emerald-300"
                    >
                        <span
                            class="h-1.5 w-1.5 rounded-full bg-emerald-400"
                            aria-hidden="true"
                        />
                        增加
                    </span>
                    <span
                        class="inline-flex items-center gap-1.5 text-rose-300"
                    >
                        <span
                            class="h-1.5 w-1.5 rounded-full bg-rose-400"
                            aria-hidden="true"
                        />
                        扣除
                    </span>
                </div>
            </div>

            <div
                v-if="months.length > 0"
                class="flex flex-col gap-1.5 rounded-xl border border-white/[0.08] bg-white/[0.03] px-3 py-2"
            >
                <button
                    type="button"
                    class="flex w-full cursor-pointer items-center justify-between gap-2 text-left"
                    :aria-expanded="indexExpanded"
                    aria-controls="calculation-month-index"
                    @click="toggleIndex"
                >
                    <span class="text-sm font-semibold text-zinc-100">
                        {{ spotlightLabel }}
                    </span>
                    <ChevronDown24Regular
                        class="h-3.5 w-3.5 shrink-0 text-zinc-400 transition-transform"
                        :class="{ 'rotate-180': indexExpanded }"
                    />
                </button>

                <div
                    id="calculation-month-index"
                    v-show="indexExpanded"
                    class="flex flex-col gap-1.5"
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
            </div>

            <div
                ref="scrollRef"
                role="region"
                tabindex="0"
                aria-label="計算明細列表"
                class="custom-scrollbar max-h-[60vh] overflow-y-auto overscroll-contain pr-1 outline-none focus-visible:ring-2 focus-visible:ring-white/30"
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
                            class="sticky top-0 z-10 bg-zinc-900/95 px-1 py-1 text-sm font-semibold text-zinc-100 backdrop-blur-sm"
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
        </div>
    </VaModal>
</template>

<style scoped>
/* Sizing override — appearance (glassomorphism) comes from
   styles/va-modal-glass.css */
:deep(.va-modal__dialog) {
    max-width: 720px !important;
    width: 95% !important;
    overflow: hidden;
}

/* Custom Scrollbar */
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
