<script setup lang="ts">
import { NGrid, NGi } from 'naive-ui';
import { ccMix } from '@/composables/utils';
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://www.mediafire.com/file/91h9afekvfnuymi/mercury_world.rar/file"
                title="S1"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("第一季") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://drive.google.com/file/d/12ka9h87OkzmSrRiPTPhFhs_-Q-sfcCm7/view"
                title="S2 to S5"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("第二～五季") }}
            </div>
        </n-gi>
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
    </n-grid> 
</template>