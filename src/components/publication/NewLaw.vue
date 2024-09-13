<script setup lang="ts">
import { NGrid, NGi } from "naive-ui";

const publishDate = "2024 年 6 月 24 日";
const effectiveDate = "2024 年 7 月 1 日";
</script>

<template>
    <n-grid x-gap="12" y-gap="10" cols="2" class="w-11/12" item-responsive>
        <n-gi span="2 800:1">
            <!-- <n-icon> </n-icon> -->
            <div class="-ml-28">
                <div class="text-4xl text-red-400 text-center mt-24">
                    《水星法》新法公告
                </div>
                <div class="text-2xl text-red-400 text-center mt-4">
                    修訂之條文於公告一週後實施。
                </div>
                <div class="text-2xl text-lime-400 text-center mt-4">
                    公告日期：{{ publishDate }}
                </div>
                <div
                    class="text-3xl text-emerald-200 text-center font-bold mt-4"
                >
                    生效日期：{{ effectiveDate }}
                </div>
            </div>
        </n-gi>
        <n-gi span="10 800:1" class="-ml-20">
            <!-- want to integrate within the viewer in Law.vue -->
            <iframe
                width="700"
                height="400"
                src="https://drive.google.com/file/d/1Lti-30AKf4cz9feAp6sfxyT0p82awT2F/preview?usp=drive_link"
                title="水星法新法公告"
            >
            </iframe>
        </n-gi>
    </n-grid>
</template>
