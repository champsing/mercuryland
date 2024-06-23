<script setup lang="ts">
import { NDivider, NGrid, NGi } from 'naive-ui';
import { ccMix } from '@/composables/utils';
import Law from "./Law.vue";
import Archive from './Archive.vue';
</script>

<template>
    <div class="text-2xl text-neutral-100 text-center mt-4">
        {{ ccMix("在這裡可以找到所有規則、伺服器名簿與檔案。") }}
    </div>
    <n-divider />
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1"></n-gi>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://drive.google.com/file/d/1Lti-30AKf4cz9feAp6sfxyT0p82awT2F/preview?usp=drive_link" 
                title="水星法"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("《水星法》公告版本") }}
            </div>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("公告一週後實施。") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1"></n-gi>
    </n-grid>
    <n-divider />
        <div class="text-4xl text-neutral-100 text-left mt-4 mb-6">
            {{ ccMix("法規與資料") }}
        </div>
        <div class="mt-4">
            <Law>
            </Law>
        </div>
    <n-divider class="!mt-6 !mb-2" />
        <div class="text-4xl text-neutral-100 text-left mt-4 mb-6">
            {{ ccMix("地圖檔") }}
        </div>
        <div class="mt-4">
            <Archive>
            </Archive>
        </div>
    <n-divider />
</template>
