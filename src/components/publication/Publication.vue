<script setup lang="ts">
import { NDivider } from 'naive-ui';
import { ccMix } from '@/composables/utils';
import Law from "./Law.vue";
// import NewLaw from "./NewLaw.vue"; //如果沒有新版本水星法可以註解起來
import Archive from './Archive.vue';
</script>

<template>
    <!-- 如果沒有新版本水星法可以註解起來 -->
        <!-- <NewLaw /> -->
    <n-divider />
        <div class="text-4xl text-neutral-100 text-left mt-4 mb-6">
            {{ ccMix("法規與資料") }}
        </div>
        <div class="mt-4">
            <Law />
        </div>
    <n-divider class="!mt-6 !mb-2" />
        <div class="text-4xl text-neutral-100 text-left mt-4 mb-6">
            {{ ccMix("地圖檔") }}
        </div>
        <div class="mt-4">
            <Archive />
        </div>
    <n-divider />
</template>
