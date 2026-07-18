<script setup lang="ts">
import { Ref, ref, computed } from "vue";
import { VaButton, VaDivider, VaIcon, VaSplit, VaTab, VaTabs } from "vuestic-ui";
import lawDocument from "@assets/data/law_document.json";
import { WindowNew20Filled } from "@vicons/fluent";
</script>

<script select-doc lang="ts">
class LawDocEntry {
    id: number;
    name: string;
    description: string;
    url: string;
}

let currentDocument: Ref<LawDocEntry> = ref({
    id: 0,
    name: "《水星法》",
    description: "水星伺服器法律支柱。",
    url: "https://drive.google.com/file/d/17zIPRyN0BpR7TU10ARDJwkKVV-zLpsz8/preview?usp=drive_link",
    group: "",
});

/** Unique group names extracted from the law documents, preserving order of first appearance. */
const groups = computed(() =>
    [...new Set(lawDocument.map((doc) => doc.group))]
);

/** Documents grouped by their `group` field. */
const docsByGroup = computed(() => {
    const map: Record<string, LawDocEntry[]> = {};
    for (const doc of lawDocument) {
        (map[doc.group] ??= []).push(doc as LawDocEntry);
    }
    return map;
});

/** Currently active group tab — defaults to the first group. */
const activeGroup = ref(groups.value[0] ?? "");

function selectDoc(doc: LawDocEntry) {
    currentDocument.value = doc;
}
</script>

<template>
    <VaSplit :model-value="40" disabled>
        <template #start>
            <VaTabs v-model="activeGroup" class="law-tabs" vertical>
                <template #tabs>
                    <VaTab v-for="group in groups" :key="group" :name="group" class="law-tab">
                        {{ group }}
                    </VaTab>
                </template>

                <div
                    v-for="group in groups"
                    :key="group"
                    v-show="activeGroup === group"
                    class="flex flex-col gap-1 px-2"
                >
                    <div
                        v-for="doc in docsByGroup[group]"
                        :key="doc.id"
                        class="law-doc-item"
                        :class="{ 'law-doc-item--active': currentDocument.name === doc.name }"
                        @click="selectDoc(doc)"
                    >
                        {{ doc.name }}
                    </div>
                </div>
            </VaTabs>
            <VaDivider class="mt-8" />
            <div class="text-zinc-300 text-center text-3xl mt-4">
                {{ currentDocument.name }}
            </div>
            <div class="text-zinc-300 text-center text-lg mt-4">
                {{ currentDocument.description }}
            </div>
            <div class="text-center mt-5">
                <VaButton
                    round
                    size="medium"
                    color="#38b67d"
                    :href="currentDocument.url"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <VaIcon size="large">
                        <WindowNew20Filled />
                    </VaIcon>
                    <div class="ml-2 mr-2 text-center">在新分頁開啟</div>
                </VaButton>
            </div>
        </template>
        <template #end>
            <!-- need calciFrameHeight() -->
            <iframe
                class="ml-2"
                width="100%"
                height="600"
                frameborder="0"
                :src="currentDocument.url"
                title="preview iframe"
            />
        </template>
    </VaSplit>
</template>

<style lang="scss">
.law-tabs {
    --va-tabs-gap: 0;
}

.law-tab {
    text-align: left;
    font-size: 1.125rem;
    padding: 8px 16px;
}

.law-doc-item {
    @apply text-white text-base cursor-pointer px-3 py-2 rounded;
    transition: background-color 0.15s, color 0.15s;

    &:hover {
        background-color: rgba(70, 114, 12, 0.6);
    }

    &--active {
        background-color: rgba(56, 182, 125, 0.5);
        color: #38b67d;
        font-weight: 600;
    }
}

</style>
