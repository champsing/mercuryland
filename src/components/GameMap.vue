<script setup lang="ts">
import { UseElementBounding, UseWindowSize } from "@vueuse/components";
</script>

<template>
    <UseWindowSize v-slot="{ width, height }">
        <UseElementBounding v-slot="{ top }" class="-mt-3">
            <iframe
                src="https://play.mercuryland.online:25565/"
                :width="width"
                :height="height - top"
            >
            </iframe>
        </UseElementBounding>
    </UseWindowSize>
</template>
