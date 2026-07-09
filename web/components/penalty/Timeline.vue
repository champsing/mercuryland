<script setup lang="ts">
import { stateString, stateColor } from "@/composables/penalty";

const props = defineProps<{
    history: Array<[number, string]>;
}>();
</script>

<template>
    <div v-if="props.history && props.history.length > 0" class="mt-4">
        <div class="relative">
            <div class="absolute top-4 left-0 right-0 h-0.5 bg-gray-300"></div>
            <div class="flex justify-between">
                <div
                    v-for="(item, index) in props.history"
                    :key="index"
                    class="flex flex-col items-center"
                    :style="{
                        flex:
                            index === 0 || index === props.history.length - 1
                                ? '0 0 auto'
                                : '1',
                    }"
                >
                    <div
                        class="w-3 h-3 rounded-full shadow-md"
                        :style="{
                            backgroundColor: stateColor(item[0], 'raw'),
                        }"
                    ></div>
                    <div class="text-xs text-center mt-1">
                        <div
                            class="font-medium mt-1"
                            :style="{
                                color: stateColor(item[0], 'raw'),
                            }"
                        >
                            {{ stateString(item[0]) }}
                        </div>
                        <div class="text-gray-500">{{ item[1] }}</div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
