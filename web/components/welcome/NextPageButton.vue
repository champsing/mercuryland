<script setup lang="ts">
import { useWindowSize } from "@vueuse/core";
import { NButton } from "naive-ui";
import { VaIcon } from "vuestic-ui";
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
                <VaIcon size="128px">
                    <angle-down />
                </VaIcon>
            </template>
        </n-button>
    </div>
</template>
