<script setup lang="ts">
import { NButton, NGrid, NGi } from 'naive-ui';
import { ccMix, openLink } from '@/composables/utils';
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://www.google.com/url?q=https%3A%2F%2Fwww.mediafire.com%2Ffile%2F91h9afekvfnuymi%2Fmercury_world.rar%2Ffile&sa=D&sntz=1&usg=AOvVaw2P9l-qAnZb-hTtCCpkUGMI"
                title="S1"
            >
            </iframe>
            <div class="text-center">
                <n-button
                :text="true"
                :focusable="false"
                @click="openLink('https://www.mediafire.com/file/91h9afekvfnuymi/mercury_world.rar/file')"
                >
                <div class="text-2xl text-neutral-100 mt-4 mr-10">
                    {{ ccMix("第一季") }}
                </div>
                </n-button>
                <br/>
                <div class="mr-10">
                    {{ ccMix("[點擊]") }}
                </div>
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://drive.google.com/file/d/12ka9h87OkzmSrRiPTPhFhs_-Q-sfcCm7/preview"
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
                src="https://drive.google.com/file/d/11JeFAEeQHUnSpxRajxiJlKt5o7dRTMh6/preview" 
                title="S6, 7"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("第六、七季") }}
            </div>
        </n-gi>
    </n-grid> 
</template>