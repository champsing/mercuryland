<script setup lang="ts">
// TODO: replaced by vueuse
import { useWindowSize } from "vue-window-size";
import { NButton, NIcon } from "naive-ui";
import { AngleDown } from "@vicons/fa";

const props = defineProps<{ page: number }>();

const vh = useWindowSize().height;
function action() {
    window.scrollTo({ top: props.page * vh.value, behavior: "smooth" });
}
</script>

<template>
    <div class="z-10 absolute bottom-10 w-full text-center">
        <n-button
            class="w-auto opacity-30 hover:opacity-100"
            text
            rounded
            @click="action()"
        >
            <template #icon>
                <n-icon size="128">
                    <angle-down />
                </n-icon>
            </template>
        </n-button>
    </div>
</template>
