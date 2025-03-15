<script setup lang="ts">
import { VaButton, VaIcon } from "vuestic-ui";
import { AngleUp } from "@vicons/fa";
import { backToTop } from "@/composables/utils";
</script>

<template>
    <div class="z-10 absolute -bottom-2 w-full text-center">
        <VaButton
            class="w-auto opacity-30 hover:opacity-100"
            round
            color="#5bc6a1"
            preset="secondary"
            :hover-opacity="0"
            :pressed-opacity="0"
            size="small"
            @click="backToTop()"
        >
            <VaIcon size="128px">
                <angle-up />
            </VaIcon>
        </VaButton>
    </div>
</template>
