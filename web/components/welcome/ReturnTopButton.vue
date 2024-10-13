<script setup lang="ts">
import { NButton } from "naive-ui";
import { VaIcon } from "vuestic-ui";
import { AngleUp } from "@vicons/fa";
function action() {
    window.scrollTo({ top: 0, behavior: "smooth" });
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
                    <angle-up />
                </VaIcon>
            </template>
        </n-button>
    </div>
</template>
