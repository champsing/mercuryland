<script setup lang="ts">
import { NGrid, NGi, NDivider } from "naive-ui";
import { ccMix } from "@/composables/utils";
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
    <div class="h-screen overflow-hidden bg-teal-800 relative">
        <NextPageButton :page="3" />
        <n-grid
            x-gap="0"
            :cols="2"
            item-responsive
            class="absolute x-center w-10/12 h-full"
        >
            <n-gi span="0 768:1" class="relative">
                <img
                    src="@assets/images/welcome/statue_of_liberty.png"
                    class="absolute y-center x-center h-5/6 object-contain"
                    alt="Welcome"
                />
            </n-gi>
            <n-gi span="2 768:1" class="relative">
                <div class="absolute y-center x-center">
                    <div class="text-yellow-400 font-bold text-6xl">
                        {{ ccMix("發揮您的創意，建造您夢想中的建築。") }}
                    </div>
                    <n-divider />
                    <div class="text-yellow-600 font-bold text-3xl">
                        {{
                            ccMix(
                                "絕無思想審查制度。您可以在我們的伺服器復刻世界上的有名建築，或單單只是在這個世界蓋出專屬於您的根據地。"
                            )
                        }}
                    </div>
                </div>
            </n-gi>
        </n-grid>
    </div>
</template>
