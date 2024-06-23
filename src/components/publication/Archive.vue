<script setup lang="ts">
import { NGrid, NGi } from 'naive-ui';
import { ccMix } from '@/composables/utils';
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://drive.google.com/file/d/11JeFAEeQHUnSpxRajxiJlKt5o7dRTMh6/view" 
                title="S6, 7"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("第六、七季") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1">
        </n-gi>
        <n-gi span="3 800:1">
        </n-gi>
    </n-grid> 
</template>