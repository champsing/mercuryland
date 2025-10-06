<script setup lang="ts">
import { ref, computed } from "vue";
import { VaInput, VaModal, VaButton, VaIcon } from "vuestic-ui";
import { BrandSteam } from "@vicons/tabler";

const props = defineProps<{
    textareaRef: any; // Ref to the VaTextarea
}>();

const emit = defineEmits<{
    insertHtml: [html: string, position: number];
}>();

const isModalOpen = ref(false);
const steamId = ref("");

const steamHtml = computed(() => {
    if (!steamId.value.trim()) return "";
    let url = `https://store.steampowered.com/widget/${steamId.value.trim()}/`;
    return `<div style="height: 200px;"><iframe src="${url}" frameborder="0" width="510" height="200" scrolling="no"/></div>`;
});

function openModal() {
    isModalOpen.value = true;
    steamId.value = "";
}

function closeModal() {
    isModalOpen.value = false;
}

function save() {
    if (!steamId.value.trim()) return;

    const position =
        props.textareaRef?.selectionStart ||
        props.textareaRef?.value?.length ||
        0;

    emit("insertHtml", steamHtml.value, position);
    closeModal();
}
</script>

<template>
    <VaButton @click="openModal" color="#2a475e" class="w-full h-full">
        <VaIcon class="mr-2">
            <BrandSteam />
        </VaIcon>
        添加游戏
    </VaButton>

    <VaModal
        v-model="isModalOpen"
        @update:model-value="closeModal"
        hide-default-actions
        size="small"
        close-button
    >
        <div class="space-y-4">
            <VaInput
                v-model="steamId"
                label="Steam游戏ID"
                placeholder="请输入Steam游戏ID，例如：289070"
            />

            <div>
                <label class="block text-sm font-medium mb-2">预览:</label>
                <div v-html="steamHtml"></div>
            </div>

            <div class="flex justify-end gap-2">
                <VaButton @click="closeModal" color="secondary">
                    取消
                </VaButton>
                <VaButton
                    @click="save"
                    color="primary"
                    :disabled="!steamId.trim()"
                >
                    保存
                </VaButton>
            </div>
        </div>
    </VaModal>
</template>
