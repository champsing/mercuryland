<script setup lang="ts">
import { Ref, ref } from "vue";
import { NIcon } from "naive-ui";
import { VaButton, VaDivider, VaSplit, VaMenuList } from "vuestic-ui";
import lawDocument from "@assets/data/law_document.json";
import { WindowNew20Filled } from "@vicons/fluent";
import { ccMix, openLink } from "@/composables/utils";
</script>

<script select-doc lang="ts">
class LawDocEntry {
    id: number;
    name: string;
    description: string;
    url: string;
}

class LawMenuEntry {
    id: number;
    text: string;
    value: string;
    group: string;
}

let currentDocument: Ref<LawDocEntry> = ref({
    id: 0,
    name: "《水星法》",
    description: "水星伺服器法律支柱。",
    url: "https://drive.google.com/file/d/17zIPRyN0BpR7TU10ARDJwkKVV-zLpsz8/preview?usp=drive_link",
    group: "",
});

function findCurrentDoc(doc: string) {
    let target = lawDocument.filter((v) => v.name == doc)[0];
    currentDocument.value = target;
}

let docOptions: Ref<LawMenuEntry[]> = ref(parseOptions(lawDocument));

function parseOptions(law_document: typeof lawDocument) {
    let optionArray = [];
    for (let i = 0; i < law_document.length; i++) {
        optionArray[i] = {
            id: law_document[i].id,
            text: law_document[i].name,
            value: law_document[i].name,
            group: lawDocument[i].group,
        };
    }
    return optionArray;
}
</script>

<template>
    <VaSplit :model-value="40">
        <template #start>
            <!-- need to be 23% -->
            <VaMenuList
                class="text-white text-lg doc-menu-hover"
                :options="docOptions"
                @selected="(doc) => findCurrentDoc(doc.text)"
            />
            <!-- group name too low, need mb-2 -->
            <VaDivider class="mt-2" />
            <div class="text-zinc-300 text-center text-3xl mt-4">
                {{ ccMix(currentDocument.name) }}
            </div>
            <div class="text-zinc-300 text-center text-lg mt-4">
                {{ ccMix(currentDocument.description) }}
            </div>
            <div class="text-center mt-3">
                <!-- need further adjust -->
                <VaButton
                    round
                    size="medium"
                    color="#38b67d"
                    @click="openLink(currentDocument.url)"
                >
                    <n-icon>
                        <WindowNew20Filled/>
                    </n-icon>
                    <div class="mt-1 mr-2 text-center">
                        {{ ccMix("在新分頁開啟") }}
                    </div>
                </VaButton>
            </div>
        </template>
        <template #end>
            <!-- need calciFrameHeight() -->
            <iframe
                class="ml-4"
                width="800"
                height="600"
                frameborder="0"
                :src="currentDocument.url"
                title="preview iframe"
            >
            </iframe>
        </template>
    </VaSplit>
</template>

<style lang="scss" scoped>
.color-picker {
    color: #38b67d;
}
.doc-menu-hover {
    --va-menu-item-hover-color: #e13535;
    --va-menu-item-hover-opacity: 0.6;
    --va-menu-padding-x: 8px;
    --va-menu-padding-y: 10px;
}
</style>
