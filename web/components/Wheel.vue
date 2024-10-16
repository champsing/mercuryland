<script setup lang="ts">
import { Wheel } from "spin-wheel";
import { ref, onMounted, onUpdated } from "vue";
import { VaTextarea } from "vuestic-ui";

const textArea = defineModel("textArea", {
    type: String,
    default: "",
    set(value: string) {
        return value;
    },
});

const numItems = 2;

const wheelContainer = ref(null);
let wheel = null;

const wheelProps = {
    items: [],
    itemLabelRadiusMax: 0.4,
};

for (let i = 0; i < numItems; i++) {
    wheelProps.items.push({
        label: "item " + i,
    });
}

onMounted(() => {
    wheel = new Wheel(wheelContainer.value, wheelProps);
    wheel.itemBackgroundColors = ["#12181c", "#363636"];
});

// onUpdated(() => {
//     wheelProps.items.push({
//         label: "item " + numItems,
//     });
//     wheel.items = wheelProps.items;
// });
</script>

<template>
    <div class="flex">
        <div class="wheel-wrapper" ref="wheelContainer"></div>
        <VaTextarea v-model="textArea" color="#ffffff"/>
    </div>
</template>
