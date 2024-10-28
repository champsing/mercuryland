<script setup lang="ts">
import { VaButton } from "vuestic-ui";
import mapArchives from "@assets/data/map_archives.json"
</script>

<template>
    <div class="flex flex-row justify-center gap-20">
        <template v-for="archive in mapArchives">
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
