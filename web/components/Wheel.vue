<script setup lang="ts">
import { Wheel } from "spin-wheel";
import { ref, onMounted, reactive } from "vue";
import { VaTextarea, VaButton, VaModal } from "vuestic-ui";
const wheelContainer = ref(null);
let wheel: Wheel = null;

const textArea = defineModel("textArea", {
    type: String,
    default: "<s>1</s>\n2\n3\n4\n5\n6",
    set(value: string) {
        if (wheel != null) {
            wheel.items = value.split("\n").map((x) => {
                return {
                    label: x,
                };
            });
        }

        return value;
    },
});
const textArea2 = defineModel("textArea2", { type: String, default: "" });

function spin() {
    wheel.spin(10000);
}

function rest() {
    var audio = new Audio("/bell_ring.mp3");
    audio.play();
    modal.text = wheel.items[wheel.getCurrentIndex()].label;
    modal.show = true;

    move()
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
    var audio = new Audio("/disconnect.mp3");
    audio.play();
}

onMounted(() => {
    const overlay = new Image();
    overlay.src = "/pointer.svg";

    const props = {
        items: [{ label: "" }],
        itemLabelRadiusMax: 0.4,
        itemBackgroundColors: ["#ffffff", "#ff0000", "#00ff00"],
        isInteractive: false,
        overlayImage: overlay,
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
    <div class="flex w-full justify-evenly">
        <div class="wheel-wrapper w-2/5" ref="wheelContainer"></div>
        <div class="w-1/5">
            <div class="va-h4">待抽区</div>
            <VaTextarea
                v-model="textArea"
                color="#ffffff"
                :resize="false"
                class="w-full h-96 mt-8"
            />
            <VaButton class="w-full mt-8" @click="spin"> 旋转 </VaButton>
            <div class="h-44"></div>
        </div>
        <div class="w-1/5">
            <div class="va-h4">抽中区</div>
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
    <VaModal v-model="modal.show" noDismiss closeButton hide-default-actions>
        <div class="text-lg">
            {{ modal.text }}
        </div>
    </VaModal>
    <VaModal v-model="modal2.show" noDismiss @ok="textArea2 = ''">
        您确定要清空抽中区吗?
    </VaModal>
</template>
