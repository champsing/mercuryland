<script setup lang="ts">
import VueWheelSpinner from "vue-wheel-spinner";
import { ref, computed, onMounted, onBeforeUnmount } from "vue";

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

const MAX_SLICE_TEXT_UNITS = 10;

function getSliceCharUnits(char: string): number {
    return /[ -~]/.test(char) ? 0.6 : 1; // narrow ASCII counts as 0.6, CJK/full-width counts as 1
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

const props = defineProps<{
    items: string[];
}>();

const emit = defineEmits<{
    winner: [winnerId: number];
}>();

const wheelRef = ref<WheelSpinnerExpose | null>(null);
const isSpinning = ref(false);
const spinDuration = ref(1500);
const currentWinnerIndex = ref<number | null>(null);
const pendingWinnerIndex = ref<number | null>(null);
let tickTimer: ReturnType<typeof setInterval> | null = null;

const spinnerSlices = computed<SpinnerSlice[]>(() => {
    const result: SpinnerSlice[] = [];
    props.items.forEach((item, itemIndex) => {
        const displayText = truncateSliceLabel(item);
        result.push({
            color: colorPalette[itemIndex % colorPalette.length],
            textColor: "#000000",
            text: displayText,
            baseIndex: itemIndex,
        });
    });
    return result;
});

const cursorAngle = 270;
const cursorPosition = "edge";
const cursorDistance = 12;
const sliceFontStyle = "bold 14px 'Noto Sans TC', sans-serif";
const extraSpins = 6;

function pickWinnerSlice(): number | null {
    if (props.items.length === 0) return null;
    const randomIndex = Math.floor(Math.random() * props.items.length);
    return randomIndex;
}

function spin() {
    if (props.items.length === 0 || wheelRef.value === null || isSpinning.value)
        return;

    const winnerIndex = pickWinnerSlice();
    if (winnerIndex === null) return;

    isSpinning.value = true;
    spinDuration.value = 1000 + Math.round(Math.random() * 1000);
    pendingWinnerIndex.value = winnerIndex;
    wheelRef.value.spinWheel(winnerIndex);
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
        :slice-font-style="sliceFontStyle"
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
