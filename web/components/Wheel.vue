<script setup lang="ts">
import { Wheel } from "spin-wheel";
import { ref, onMounted } from "vue";
import { VaTextarea } from "vuestic-ui";
const wheelContainer = ref(null);
let wheel: Wheel = null;

const textArea = defineModel("textArea", {
    type: String,
    default: "",
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

onMounted(() => {
    const props = {
        items: [{ label: "" }],
        itemLabelRadiusMax: 0.4,
        itemBackgroundColors: ["#ffffff", "#ff0000", "#00ff00"],
    };
    wheel = new Wheel(wheelContainer.value, props);
    const img = new Image()
    img.src = "/pointer.svg"
    wheel.overlayImage = img
});

// onUpdated(() => {
//     wheelProps.items.push({
//         label: "item " + numItems,
//     });
//     wheel.items = wheelProps.items;
// });
</script>

<template>
    <div class="flex w-full justify-evenly">
        <div class="wheel-wrapper w-2/5" ref="wheelContainer"></div>
        <div class="w-2/5">
            <VaTextarea
                v-model="textArea"
                color="#ffffff"
                :resize="false"
                class="w-full h-96 mt-8"
            />
        </div>
    </div>
</template>
