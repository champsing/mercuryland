<script setup lang="ts">
import { NButton, NIcon } from "naive-ui";
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
                <n-icon size="128">
                    <angle-up />
                </n-icon>
            </template>
        </n-button>
    </div>
</template>
