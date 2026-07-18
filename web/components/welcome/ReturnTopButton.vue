<script setup lang="ts">
import { VaButton, VaIcon } from "vuestic-ui";
import { AngleUp } from "@vicons/fa";
import { backToTop } from "@/composables/utils";
</script>

<template>
    <div class="z-10 absolute bottom-6 w-full text-center">
        <VaButton
            class="w-auto !opacity-40 hover:!opacity-80 transition-opacity"
            round
            color="#ffffff"
            preset="secondary"
            :hover-opacity="0"
            :pressed-opacity="0"
            size="small"
            @click="backToTop()"
        >
            <VaIcon size="48px">
                <angle-up />
            </VaIcon>
        </VaButton>
    </div>
</template>
