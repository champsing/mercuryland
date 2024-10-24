<script setup lang="ts">
import { VaButton } from "vuestic-ui";

const archives: { season: string; link: string }[] = [
    {
        season: "第一季",
        link: "https://www.mediafire.com/file/91h9afekvfnuymi/mercury_world.rar/file",
    },
    {
        season: "第二～五季",
        link: "https://drive.google.com/file/d/12ka9h87OkzmSrRiPTPhFhs_-Q-sfcCm7/edit",
    },
    {
        season: "第六、七季",
        link: "https://drive.google.com/file/d/11JeFAEeQHUnSpxRajxiJlKt5o7dRTMh6/edit",
    },
    {
        season: "第八季",
        link: "https://drive.google.com/file/d/1rcjNVGFeVIw9Oi02WEFeas4XNT4h6_I6/edit",
    },
];
</script>

<template>
    <div class="flex flex-row justify-center gap-20">
        <template v-for="archive in archives">
            <div>
                <VaButton
                    size="large"
                    color="#105659"
                    :href="archive.link"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <div class="ml-2 mr-2 text-center">
                        {{ archive.season }}
                    </div>
                </VaButton>
            </div>
        </template>
    </div>
</template>
