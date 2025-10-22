<script setup lang="ts">
import VueWheelSpinner from "vue-wheel-spinner";
import { ref, computed, onMounted, onBeforeUnmount } from "vue";

const props = defineProps<{
    items: { label: string; weight: number }[];
}>();

const emit = defineEmits<{
    winner: [winnerId: number];
}>();

const MAX_SLICE_TEXT_UNITS = 10;

function getSliceCharUnits(char: string): number {
    return /[ -~]/.test(char) ? 0.6 : 1; // narrow ASCII counts as 0.6, CJK/full-width counts as 1
}

function countSliceTextUnits(text: string): number {
    let units = 0;
    for (const char of text) units += getSliceCharUnits(char);
    return units;
}

function truncateSliceLabel(
    label: string,
    maxUnits = MAX_SLICE_TEXT_UNITS,
): string {
    const trimmed = label.trim();
    if (maxUnits <= 0) return trimmed === "" ? "" : "…";
    let units = 0;
    let result = "";
    for (const char of trimmed) {
        const charUnits = getSliceCharUnits(char);
        if (units + charUnits > maxUnits) {
            return result ? `${result}…` : "…";
        }
        units += charUnits;
        result += char;
    }
    return result;
}

const colorPalette = [
    "#dc2626",
    "#ea580c",
    "#d97706",
    "#ca8a04",
    "#65a30d",
    "#16a34a",
    "#059669",
    "#0d9488",
    "#0891b2",
    "#0284c7",
    "#2563eb",
    "#4f46e5",
    "#7c3aed",
    "#9333ea",
    "#c026d3",
    "#db2777",
    "#e11d48",
];

interface WheelSpinnerExpose {
    spinWheel: (winnerIndex: number) => void;
    drawWheel: () => void;
}

interface SpinnerSlice {
    color: string;
    textColor: string;
    text: string;
    baseIndex: number;
}

const wheelRef = ref<WheelSpinnerExpose | null>(null);
const isSpinning = ref(false);
const spinDuration = ref(1500);
const currentWinnerIndex = ref<number | null>(null);
const pendingWinnerIndex = ref<number | null>(null);
let tickTimer: ReturnType<typeof setInterval> | null = null;

const spinnerSlices = computed<SpinnerSlice[]>(() => {
    const result: SpinnerSlice[] = [];
    props.items.forEach((item, itemIndex) => {
        const repeats = Math.max(1, item.weight);
        const suffix = item.weight > 1 ? ` x${item.weight}` : "";
        const suffixUnits = suffix ? countSliceTextUnits(suffix) : 0;
        const labelBudget = Math.max(0, MAX_SLICE_TEXT_UNITS - suffixUnits);
        const baseLabel = truncateSliceLabel(item.label, labelBudget);
        const displayText = `${baseLabel}${suffix}`.trim();
        for (let i = 0; i < repeats; i++) {
            result.push({
                color: colorPalette[itemIndex % colorPalette.length],
                textColor: "#000000",
                text: i === 0 ? displayText : "",
                baseIndex: itemIndex,
            });
        }
    });
    return result;
});

const sliceIndexBuckets = computed<number[][]>(() => {
    const buckets: number[][] = props.items.map(() => []);
    spinnerSlices.value.forEach((slice, index) => {
        if (!buckets[slice.baseIndex]) buckets[slice.baseIndex] = [];
        buckets[slice.baseIndex].push(index);
    });
    return buckets;
});

const cursorAngle = 270;
const cursorPosition = "edge";
const cursorDistance = 12;
const extraSpins = 6;

function pickWinnerSlice(): { displayIndex: number; baseIndex: number } | null {
    if (props.items.length === 0 || spinnerSlices.value.length === 0)
        return null;
    const totalWeight = props.items.reduce(
        (sum, item) => sum + Math.max(1, item.weight),
        0,
    );
    if (totalWeight <= 0) return null;
    let threshold = Math.random() * totalWeight;
    for (let i = 0; i < props.items.length; i++) {
        const weight = Math.max(1, props.items[i].weight);
        threshold -= weight;
        if (threshold < 0) {
            const bucket = sliceIndexBuckets.value[i];
            if (bucket && bucket.length > 0) {
                const displayIndex =
                    bucket[Math.floor(Math.random() * bucket.length)];
                return { displayIndex, baseIndex: i };
            }
            return { displayIndex: i, baseIndex: i };
        }
    }
    const lastIndex = props.items.length - 1;
    const fallbackBucket = sliceIndexBuckets.value[lastIndex];
    const displayIndex = fallbackBucket?.[0] ?? lastIndex;
    return { displayIndex, baseIndex: lastIndex };
}

function spin() {
    if (props.items.length === 0 || wheelRef.value === null || isSpinning.value)
        return;

    const winnerSlice = pickWinnerSlice();
    if (!winnerSlice) return;

    isSpinning.value = true;
    spinDuration.value = 1000 + Math.round(Math.random() * 1000);
    pendingWinnerIndex.value = winnerSlice.baseIndex;
    wheelRef.value.spinWheel(winnerSlice.displayIndex);
}

function handleSpinStart() {
    startTickLoop();
}

function handleSpinEnd() {
    stopTickLoop();
    var audio = new Audio("/sounds/rest.mp3");
    audio.play();
    const baseIndex = pendingWinnerIndex.value;
    isSpinning.value = false;
    currentWinnerIndex.value = baseIndex ?? null;
    pendingWinnerIndex.value = null;
    if (baseIndex !== null) {
        emit("winner", baseIndex);
    }
}

function tick() {
    var audio = new Audio("/sounds/tick.mp3");
    audio.play();
}

function startTickLoop() {
    stopTickLoop();
    tickTimer = window.setInterval(tick, 160);
}

function stopTickLoop() {
    if (tickTimer !== null) {
        window.clearInterval(tickTimer);
        tickTimer = null;
    }
}

onMounted(() => {
    if (wheelRef.value) {
        wheelRef.value.drawWheel();
    }
});

onBeforeUnmount(() => {
    stopTickLoop();
});

defineExpose({
    spin,
    isSpinning,
});
</script>

<template>
    <VueWheelSpinner
        ref="wheelRef"
        class="mx-auto w-full"
        :slices="spinnerSlices"
        :spin-duration="spinDuration"
        :extra-spins="extraSpins"
        :cursor-angle="cursorAngle"
        :cursor-position="cursorPosition"
        :cursor-distance="cursorDistance"
        @spin-start="handleSpinStart"
        @spin-end="handleSpinEnd"
    >
        <template #cursor>
            <img
                src="/images/pointer.svg"
                alt="指針"
                class="pointer-img pointer-events-none select-none"
            />
        </template>
    </VueWheelSpinner>
</template>

<style scoped>
.pointer-img {
    width: clamp(8rem, 18vw, 10rem);
    height: auto;
    margin-top: clamp(12px, 2vw, 28px);
}
</style>
