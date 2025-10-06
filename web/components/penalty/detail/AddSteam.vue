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
    return `<iframe src="${url}" frameborder="0" width="510" height="200" scrolling="no"/>`;
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
        title="Add Steam Game"
    >
        <div class="space-y-4">
            <VaInput
                v-model="steamId"
                label="Steam Game ID"
                placeholder="Enter Steam Game ID"
            />

            <div>
                <label class="block text-sm font-medium mb-2">Preview:</label>
                <div v-html="steamHtml"></div>
            </div>

            <div class="flex justify-end gap-2">
                <VaButton @click="closeModal" color="secondary">
                    Cancel
                </VaButton>
                <VaButton
                    @click="save"
                    color="primary"
                    :disabled="!steamId.trim()"
                >
                    Save
                </VaButton>
            </div>
        </div>
    </VaModal>
</template>
