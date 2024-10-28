<script setup lang="ts">
import { Ref, ref } from "vue";
import { VaButton, VaDivider, VaIcon, VaSplit, VaMenuList } from "vuestic-ui";
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

function findCurrentDoc(doc: string) {
    currentDocument.value = lawDocument.filter((v) => v.name == doc)[0];
}

function parseOptions(law_document: typeof lawDocument) {
    let optionArray = [];
    for (let i = 0; i < law_document.length; i++) {
        optionArray[i] = {
            id: law_document[i].id,
            text: law_document[i].name,
            group: law_document[i].group,
        };
    }
    return optionArray;
}

// const newMercuryLaw: LawDocEntry = {
//     id: 99999,
//     name: "《水星法》公告版本",
//     description: "新版水星法，於公告一週後正式實施。",
//     url: "https://drive.google.com/file/d/1Lti-30AKf4cz9feAp6sfxyT0p82awT2F/preview?usp=drive_link",
// };
</script>

<template>
    <VaSplit :model-value="40" disabled>
        <template #start>
            <!-- <div class="mb-2 text-center">
                <VaButton size="medium" color="rgb(78, 93, 137)" gradient @click="currentDocument = newMercuryLaw">
                    <div class="text-xl">查看新法</div>
                </VaButton>
            </div> -->
            <VaMenuList
                class="text-white text-lg doc-menu-hover"
                :options="parseOptions(lawDocument)"
                @selected="(doc) => findCurrentDoc(doc.text)"
            />
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
                width="800"
                height="600"
                frameborder="0"
                :src="currentDocument.url"
                title="preview iframe"
            />
        </template>
    </VaSplit>
</template>

<style lang="scss">
.doc-menu-hover {
    --va-menu-item-hover-color: #e13535;
    --va-menu-item-hover-opacity: 0.6;
    --va-menu-padding-x: 8px;
    --va-menu-padding-y: 10px;
}

.va-menu-list__group-name {
    top: -6px !important;
}
</style>
