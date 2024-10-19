<script setup lang="ts">
import { Wheel } from "spin-wheel";
import { ref, onMounted, reactive } from "vue";
import { VaTextarea, VaButton, VaModal } from "vuestic-ui";
const wheelContainer = ref(null);
const isSpinning = ref(false);
const re = /x[1-9][0-9]*$/;

let wheel: Wheel = null;

const textArea = defineModel("textArea", {
    type: String,
    default: "",
    set(value: string) {
        if (wheel != null) {
            wheel.items = value
                .split("\n")
                .filter((x) => x != "")
                .map((x) => {
                    const weight = parseInt(
                        (x.match(re) || ["x1"])[0].substring(1)
                    );
                    const label = x.replace(re, "");
                    return {
                        label: label,
                        weight: weight,
                    };
                });
        }

        return value;
    },
});
const textArea2 = defineModel("textArea2", { type: String, default: "" });

function spin() {
    isSpinning.value = true;
    wheel.spin(1000 + Math.round(Math.random() * 1000));
}

function rest() {
    var audio = new Audio("/sounds/rest.mp3");
    audio.play();
    modal.text = wheel.items[wheel.getCurrentIndex()].label;
    modal.show = true;
    isSpinning.value = false;
}

function move() {
    // copy text to new area
    textArea2.value += modal.text;
    textArea2.value += "\n";

    // delete text in old area
    textArea.value = wheel.items
        .filter(
            (_: { label: string }, i: number) => i != wheel.getCurrentIndex()
        )
        .map((x: { label: string }) => x.label)
        .join("\n");
}

function tick() {
    var audio = new Audio("/sounds/tick.mp3");
    audio.play();
}

function count(text: string): number {
    return text.split("\n").filter((x) => x != "").length;
}

onMounted(() => {
    // const overlay = new Image();
    // overlay.src = "/pointer.svg";

    const props = {
        items: [],
        itemLabelRadiusMax: 0.4,
        itemBackgroundColors: [
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
        ],
        isInteractive: false,
        // overlayImage: overlay,
        onRest: rest,
        onCurrentIndexChange: tick,
    };
    wheel = new Wheel(wheelContainer.value, props);
});

const modal = reactive({
    show: false,
    text: "",
});
const modal2 = reactive({
    show: false,
});
</script>

<template>
    <div class="mt-8 ml-auto mr-auto w-11/12">
        <div class="flex w-full justify-end">
            <div class="text-lime-400 font-bold text-4xl bg-black text-right">
                BETA
            </div>
        </div>

        <div class="flex w-full justify-evenly">
            <div class="wheel-wrapper w-2/5 -mt-20" ref="wheelContainer"></div>
            <div class="w-1/5">
                <div class="va-h4">待抽区 ({{ count(textArea) }}个)</div>
                <VaTextarea
                    v-model="textArea"
                    color="#ffffff"
                    :resize="false"
                    class="w-full h-96 mt-8"
                />
                <VaButton
                    class="w-full mt-8"
                    @click="spin"
                    :disabled="isSpinning"
                >
                    旋转
                </VaButton>
                <div class="h-44"></div>
            </div>
            <div class="w-1/5">
                <div class="va-h4">抽中区 ({{ count(textArea2) }}个)</div>
                <VaTextarea
                    v-model="textArea2"
                    color="#ffffff"
                    :resize="false"
                    class="w-full h-96 mt-8"
                />
                <VaButton class="w-full mt-8" @click="modal2.show = true">
                    清空
                </VaButton>
                <div class="h-44"></div>
            </div>
        </div>
        <VaModal
            v-model="modal.show"
            noDismiss
            @ok="move"
            ok-text="移动"
            cancel-text="取消"
        >
            <div class="text-3xl">
                {{ modal.text }}
            </div>
        </VaModal>
        <VaModal v-model="modal2.show" noDismiss @ok="textArea2 = ''">
            您确定要清空抽中区吗?
        </VaModal>
    </div>
</template>
