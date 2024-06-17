<script setup lang="ts">
import { UseElementBounding, UseWindowSize } from "@vueuse/components";
</script>

<template>
    <UseWindowSize v-slot="{ width, height }">
        <UseElementBounding v-slot="{ top }" class="-mt-3">
            <iframe
                src="http://yongdafan-26679.portmap.host:22605/"
                :width="width"
                :height="height - top"
            >
            </iframe>
        </UseElementBounding>
    </UseWindowSize>
</template>
