<script setup lang="ts">
import { ref } from "vue";
import { NButton, NGrid, NGi } from "naive-ui";
import { VaSplit, VaTab, VaTabs } from "vuestic-ui";
import { ccMix, openLink } from "@/composables/utils";

let docTabValue = defineModel("docTabValue", {
    default: docList[0],
    set(value) {
        let currentTab = ref<number | null>(0); //current tab
        if (currentTab.value < docList.length) currentTab.value++;
        else currentTab.value = 0;
        value = docList[currentTab.value];
        return value;
    },
});
</script>

<script select-doc lang="ts">
let docList = [
    "《水星法》",
    "《水星伺服器破壞舉報獎勵規則》",
    "《水星伺服器性能保護法》",
    "申請模組",
    "《水星伺服器公開資料清冊》",
];
</script>

<template>
    <VaSplit class="doc-split" disabled>
        <template #start>
            <!-- need to be 23% -->
            <VaTabs v-model="docTabValue" color="#6794db" vertical>
                <template #tabs>
                    <div class="text-white text-1xl">
                        <VaTab
                            v-for="title in docList"
                            :key="title"
                            :name="title"
                        >
                            {{ ccMix(title) }}
                        </VaTab>
                    </div>
                </template>
            </VaTabs>
        </template>
        <template #end="{ containerSize }">
            <div class="img-block">
                <img
                    src="https://picsum.photos/seed/picsum/1536/800?grayscale"
                    :style="{ width: `${containerSize}px` }"
                    draggable="false"
                    class="img img--end"
                    alt="Random image"
                />
            </div>
        </template>
    </VaSplit>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1">
            <iframe
                width="410"
                height="400"
                frameborder="0"
                src="https://drive.google.com/file/d/17zIPRyN0BpR7TU10ARDJwkKVV-zLpsz8/preview?usp=drive_link"
                title="水星法"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                {{ ccMix("《水星法》") }}
            </div>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("水星伺服器法律支柱。") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe
                width="410"
                height="400"
                frameborder="0"
                src="https://drive.google.com/file/d/1Imw336b2-dnXVQ1dePxOtopQrtRlBoxh/preview?usp=drive_link"
                title="水星伺服器破壞舉報獎勵規則"
            >
            </iframe>
            <div class="text-2xl text-neutral-100 text-center mt-4 mr-2">
                {{ ccMix("《水星伺服器破壞舉報獎勵規則》") }}
            </div>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-4">
                {{ ccMix("舉報破壞伺服器和平的玩家。") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe
                width="410"
                height="400"
                frameborder="0"
                src="https://docs.google.com/document/d/14MiEMa45ubcNPay2V8XUEFi_rdZFdaeA4TgOn7hk7Iw/preview"
                title="水星伺服器性能保護法"
            >
            </iframe>
            <n-button
                :text="true"
                @click="
                    openLink(
                        'https://docs.google.com/document/d/14MiEMa45ubcNPay2V8XUEFi_rdZFdaeA4TgOn7hk7Iw/preview'
                    )
                "
                class="ml-14"
            >
                <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                    {{ ccMix("《水星伺服器性能保護法》") }}
                </div>
            </n-button>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-2">
                {{ ccMix("保護伺服器的穩定運作。") }}
            </div>
        </n-gi>
    </n-grid>
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12 mt-4" item-responsive>
        <n-gi span="3 800:1">
            <iframe
                width="410"
                height="400"
                frameborder="0"
                src="https://forms.gle/qx9XRHhuojAf7hPj9"
                title="申請模組"
            >
            </iframe>
            <n-button
                :text="true"
                @click="openLink('https://forms.gle/qx9XRHhuojAf7hPj9')"
                class="ml-36"
            >
                <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                    {{ ccMix("申請模組") }}
                </div>
            </n-button>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-4">
                {{ ccMix("您可以在此申請模組。") }}
            </div>
        </n-gi>
        <n-gi span="3 800:1">
            <iframe
                width="410"
                height="400"
                frameborder="0"
                src="https://docs.google.com/spreadsheets/d/1srqIISm1Dn908trUeRZgarB5p_2xmdmtK0HGzMEz-yM/preview?usp=drive_link"
                title="水星伺服器公開資料清冊"
            >
            </iframe>
            <n-button
                :text="true"
                @click="
                    openLink(
                        'https://docs.google.com/spreadsheets/d/1srqIISm1Dn908trUeRZgarB5p_2xmdmtK0HGzMEz-yM/preview?usp=drive_link'
                    )
                "
                class="ml-12"
            >
                <div class="text-2xl text-neutral-100 text-center mt-4 mr-10">
                    {{ ccMix("《水星伺服器公開資料清冊》") }}
                </div>
            </n-button>
            <div class="text-1xl text-neutral-100 text-center mt-4 mr-8">
                {{ ccMix("查詢您的申請資料。") }}
            </div>
        </n-gi>
    </n-grid>
</template>

<style lang="scss" scoped>
.doc-split {
    position: relative;
    height: 33rem;

    & .img-block {
        position: relative;
        height: 100%;
        overflow: hidden;
        user-select: none;
    }

    .img {
        position: absolute;
        top: 0;
        max-width: unset;

        &--start {
            left: 0;
        }

        &--end {
            right: 0;
        }
    }
}

.element.style {
    width: 23%;
}
</style>
