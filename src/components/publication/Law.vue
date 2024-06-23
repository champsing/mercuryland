<script setup lang="ts">
import { NGrid, NGi } from 'naive-ui';
import { ccMix } from '@/composables/utils';
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://drive.google.com/file/d/17zIPRyN0BpR7TU10ARDJwkKVV-zLpsz8/preview?usp=drive_link" 
                title="水星法"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("《水星法》") }}
            </div>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("水星伺服器法律支柱。") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://drive.google.com/file/d/1Imw336b2-dnXVQ1dePxOtopQrtRlBoxh/preview?usp=drive_link" 
                title="水星伺服器破壞舉報獎勵規則"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("《水星伺服器破壞舉報獎勵規則》") }}
            </div>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("舉報破壞伺服器和平的玩家。") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe 
                width="410" height="400" 
                src="https://docs.google.com/spreadsheets/d/1srqIISm1Dn908trUeRZgarB5p_2xmdmtK0HGzMEz-yM/edit?usp=drive_link" 
                title="水星伺服器公開資料清冊"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("《水星伺服器公開資料清冊》") }}
            </div>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("查詢您的申請資料。") }}
            </div>
        </n-gi>
    </n-grid> 
</template>