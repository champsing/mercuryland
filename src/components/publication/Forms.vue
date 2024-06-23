<script setup lang="ts">
import { NGrid, NGi } from 'naive-ui';
import { ccMix } from '@/composables/utils';
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://forms.gle/qx9XRHhuojAf7hPj9" 
                title="申請模組"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("申請模組") }}
            </div>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("您可以在此申請模組。") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://docs.google.com/forms/d/e/1FAIpQLSclP5J7MVBJMQttKlTQiQRSUKxCwFfbKFfaTM71W0V8u41nVw/viewform" 
                title="違規舉報"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("違規舉報") }}
            </div>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("舉報違反上方規則的玩家。") }}
            </div>
        </n-gi>
    </n-grid> 
</template>