<script setup lang="ts">
import { Ref, ref, computed } from "vue";
import { VaButton, VaDivider, VaIcon } from "vuestic-ui";
import lawDocument from "@assets/data/law_document.json";
import { nextTabIndex } from "@/composables/tabList";
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
const groups = computed(() => [
    ...new Set(lawDocument.map((doc) => doc.group)),
]);

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
const activeGroupIndex = computed(() =>
    groups.value.indexOf(activeGroup.value),
);

function onGroupKeydown(event: KeyboardEvent) {
    const next = nextTabIndex(
        event.key,
        activeGroupIndex.value,
        groups.value.length,
    );
    if (next === null) return;

    event.preventDefault();
    activeGroup.value = groups.value[next];

    const tablist = (event.currentTarget as HTMLElement).parentElement;
    tablist?.querySelectorAll<HTMLElement>('[role="tab"]')[next]?.focus();
}

function selectDoc(doc: LawDocEntry) {
    currentDocument.value = doc;
}
</script>

<template>
    <div class="flex flex-col md:flex-row gap-4">
        <div class="w-full md:w-[40%] md:shrink-0">
            <div
                role="tablist"
                aria-label="法規資料分類"
                class="flex w-full items-center gap-1 rounded-full border border-white/10 bg-white/5 p-1 shadow-[0_2px_12px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.08)] backdrop-blur-md"
            >
                <button
                    v-for="(group, index) in groups"
                    :id="`law-group-tab-${index}`"
                    :key="group"
                    type="button"
                    role="tab"
                    :aria-selected="activeGroup === group"
                    aria-controls="law-group-panel"
                    :tabindex="activeGroup === group ? 0 : -1"
                    class="flex-1 rounded-full px-4 py-1.5 text-sm font-medium text-zinc-300 transition-colors hover:bg-white/10 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
                    :class="
                        activeGroup === group
                            ? 'bg-gradient-to-b from-white/20 to-white/10 text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.2),0_1px_3px_rgba(0,0,0,0.25)]'
                            : ''
                    "
                    @click="activeGroup = group"
                    @keydown="onGroupKeydown"
                >
                    {{ group }}
                </button>
            </div>

            <div
                id="law-group-panel"
                role="tabpanel"
                :aria-labelledby="`law-group-tab-${activeGroupIndex}`"
                tabindex="0"
                class="mt-4 flex flex-col gap-1 focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-300/60"
            >
                <div
                    v-for="doc in docsByGroup[activeGroup]"
                    :key="doc.id"
                    class="law-doc-item"
                    :class="{
                        'law-doc-item--active':
                            currentDocument.name === doc.name,
                    }"
                    @click="selectDoc(doc)"
                >
                    {{ doc.name }}
                </div>
            </div>
            <VaDivider class="mt-8" />
            <div class="text-zinc-300 text-center text-2xl md:text-3xl mt-4">
                {{ currentDocument.name }}
            </div>
            <div class="text-zinc-300 text-center text-base md:text-lg mt-4">
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
        </div>
        <div class="w-full md:w-[60%] md:flex-1">
            <iframe
                class="w-full h-[60vh] md:h-[600px]"
                frameborder="0"
                :src="currentDocument.url"
                title="preview iframe"
            />
        </div>
    </div>
</template>

<style lang="scss">
.law-doc-item {
    @apply text-white text-base cursor-pointer px-3 py-2 rounded;
    transition:
        background-color 0.15s,
        color 0.15s;

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
