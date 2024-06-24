<script setup lang="ts">
import { NGrid, NGi } from 'naive-ui';
import { ccMix } from '@/composables/utils';
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1"></n-gi>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://drive.google.com/file/d/1Lti-30AKf4cz9feAp6sfxyT0p82awT2F/preview?usp=drive_link" 
                title="水星法公告版本"
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
</template>
