<script setup lang="ts">
import { VaButton } from "vuestic-ui";

const publishDate = "2024 年 6 月 24 日";
const effectiveDate = "2024 年 7 月 1 日";
//prettier-ignore
//const newLawLink = "https://drive.google.com/file/d/1Lti-30AKf4cz9feAp6sfxyT0p82awT2F/preview?usp=drive_link";
</script>

<template>
    <div class="flex flex-row justify-center gap-32">
        <!-- <n-icon> </n-icon> -->
        <div class="flex flex-col text-center">
            <div class="text-4xl text-red-400 mt-4">《水星法》新法公告</div>
            <div class="text-2xl text-red-400 mt-4">
                修訂之條文於公告一週後實施。
            </div>
            <div class="text-2xl text-lime-400 mt-4">
                公告日期：{{ publishDate }}
            </div>
            <div class="text-3xl text-emerald-200 font-bold mt-4">
                生效日期：{{ effectiveDate }}
            </div>
        </div>
        <div class="mt-20">
            <VaButton size="large" color="info" gradient @click=""> 
                <!-- need emit -->
                <div class="text-2xl">查看新法</div>
            </VaButton>
        </div>
    </div>
</template>
