<script setup lang="ts">
import { NButton, NCard, NGrid, NGi } from "naive-ui";
import { openLink } from "@/composables/utils";
</script>

<template>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1">
            <n-card class="w-auto h-auto">
                <div class="text-center">
                    <img
                        src="@assets/images/archive.jpg"
                        alt="archive"
                        class="w-80 object-center ml-8"
                    />

                    <div class="mr-4 mt-8">
                        <n-button
                            :text="true"
                            :focusable="false"
                            @click="
                                openLink(
                                    'https://www.mediafire.com/file/91h9afekvfnuymi/mercury_world.rar/file'
                                )
                            "
                        >
                            <div class="text-2xl text-neutral-100 mt-4">
                                第一季
                            </div>
                        </n-button>
                        <br />
                        [點擊下載]
                    </div>
                </div>
            </n-card>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe
                width="410"
                height="400"
                frameborder="0"
                src="https://drive.google.com/file/d/12ka9h87OkzmSrRiPTPhFhs_-Q-sfcCm7/preview"
                title="S2 to S5"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-8">
                第二～五季
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe
                width="410"
                height="400"
                frameborder="0"
                src="https://drive.google.com/file/d/11JeFAEeQHUnSpxRajxiJlKt5o7dRTMh6/preview"
                title="S6, 7"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-8">
                第六、七季
            </div>
        </n-gi>
    </n-grid>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1" class="mt-4">
            <iframe
                width="410"
                height="400"
                frameborder="0"
                src="https://drive.google.com/file/d/1rcjNVGFeVIw9Oi02WEFeas4XNT4h6_I6/preview"
                title="S8"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-8">
                第八季
            </div>
        </n-gi>
    </n-grid>
</template>
