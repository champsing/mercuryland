<script setup lang="ts">
import { ref } from "vue";
import { VaSplit, VaMenuList } from "vuestic-ui";
import lawDocument from "@assets/data/law_document.json";

function findDocId(doc: string) {
    let currentDocument = lawDocument.filter((v) => v.name == doc)[0];
    previewLink.value = currentDocument.url;
    
}
</script>

<script select-doc lang="ts">
let docList = lawDocument.map((x) => x.name)
let previewLink = ref("")
</script>

<template>
    <VaSplit class="doc-split">
        <template #start>
            <!-- need to be 23% -->
            <VaMenuList
                class="text-white text-2xl"
                :options=docList
                @selected="(doc) => findDocId(doc)"
            />
        </template>
        <template #end>
            <iframe
                class="ml-4"
                width="625"
                height="600"
                frameborder="0"
                :src=previewLink.valueOf()
                title="preview iframe"
            >
            </iframe>
        </template>
    </VaSplit>
    <!-- need description text and button to openLink() -->
</template>

<style lang="scss" scoped>
.doc-tab {
    --va-tab-height: auto;
    --va-tab-max-width: 20px;
    --va-tabs-container-height: auto;
    --va-tab-content-padding: 0.4rem 0.75rem;
    --va-tabs-slider-width: 0.7rem;
    --va-tabs-slider-height: 0.7rem;
}

.doc-split {
    position: relative;
    height: 40rem;

    & .preview-block {
        position: relative;
        height: 100%;
        overflow: hidden;
        user-select: none;
    }

    .split-view {
        position: absolute;
        top: 2;

        &--start {
            left: 0;
        }

        &--end {
            right: 0;
        }
    }
}
</style>
