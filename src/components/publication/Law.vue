<script setup lang="ts">
import { Ref, ref } from "vue";
import { VaSplit, VaTab, VaTabs } from "vuestic-ui";
import lawDocument from "@assets/data/law_document.json";

let docTabValue: Ref<{
    name: string;
    link: string;
}> = defineModel("docTabValue", {
    default: { name: docList[0], link: docLink[0] },
    set(value) {
        let currentTab = ref<number | null>(0); //current tab
        if (currentTab.value < docList.length) currentTab.value++;
        else currentTab.value = 0;
        value.name = docList[currentTab.value];
        value.link = docLink[currentTab.value];
        return value;
    },
});
</script>

<script select-doc lang="ts">
let docList = lawDocument.map((x) => x.name);
let docLink = lawDocument.map((x) => x.url);
</script>

<template>
    <VaSplit class="doc-split">
        <template #start>
            <!-- need to be 23% -->
            <!-- v-model still not change -->
            <VaTabs
                v-model=docTabValue.name
                class="doc-tab"
                color="#6794db"
                vertical
            >
                <div class="text-white text-2xl">
                    <VaTab v-for="title in docList" :key="title" :name="title">
                        {{ title }}
                    </VaTab>
                </div>
            </VaTabs>
        </template>
        <template #end v-for="link in docLink">
            <!-- need to update link alongside title -->
            <iframe
                class="ml-4"
                width="625"
                height="600"
                frameborder="0"
                :src="`${link}`"
                title="preview iframe"
            >
            </iframe>
        </template>
    </VaSplit>
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
