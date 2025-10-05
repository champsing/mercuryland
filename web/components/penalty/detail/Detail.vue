<script setup lang="ts">
import { ref, watch } from "vue";
import { VaChip, VaModal, VaTextarea, VaButton } from "vuestic-ui";
import { BASE_URL } from "@/composables/utils";
import { stateString, stateColor, PenItem } from "@/composables/penalty";
import { useAuthState } from "@/composables/authState";
import axios from "axios";
import Timeline from "../Timeline.vue";

const props = defineProps<{
    modelValue: number | null;
}>();

const emit = defineEmits<{
    (e: "update:modelValue", value: number | null): void;
}>();

const penalty = ref<PenItem | null>(null);

const authState = useAuthState();
const isEditing = ref(false);
const editedDetail = ref("");

async function loadPenalty(id: number) {
    try {
        const response = await axios.get(
            `${BASE_URL}/api/penalty/detail/${id}`,
        );
        if (response.status === 200) {
            console.log("Fetched penalty:", response.data);
            penalty.value = response.data;
        } else {
            console.error("Failed to fetch penalty");
        }
    } catch (error) {
        console.error(error);
    }
}

function startEdit() {
    if (penalty.value) {
        editedDetail.value = penalty.value.detail;
        isEditing.value = true;
    }
}

function cancelEdit() {
    isEditing.value = false;
}

async function saveDetail() {
    const token = localStorage.getItem("token");
    if (!token || !penalty.value) return;

    try {
        const response = await axios.post(
            `${BASE_URL}/api/penalty/detail/update`,
            {
                token,
                id: penalty.value.id,
                detail: editedDetail.value,
            },
        );
        if (response.data.success) {
            penalty.value.detail = editedDetail.value;
            isEditing.value = false;
        }
    } catch (error) {
        console.error(error);
    }
}
watch(
    () => props.modelValue,
    (newId) => {
        if (newId !== null) {
            loadPenalty(newId);
        } else {
            isEditing.value = false;
        }
    },
    { immediate: true },
);
</script>

<template>
    <VaModal
        :model-value="props.modelValue !== null"
        @update:model-value="
            emit('update:modelValue', $event ? props.modelValue : null)
        "
        hide-default-actions
        size="small"
        close-button
        class=""
    >
        <div v-if="penalty" class="min-h-[50vh]">
            <div class="flex gap-4 items-center">
                <VaChip
                    readonly
                    outline
                    size="small"
                    :color="stateColor(penalty.state, 'raw')"
                    class="w-24"
                >
                    ● {{ stateString(penalty.state) }}
                </VaChip>
                <div class="truncate text-xl flex-1">{{ penalty.name }}</div>
            </div>
            <div class="text-lg mt-2">{{ penalty.name }}</div>
            <Timeline :history="penalty.history" />
            <div v-if="isEditing" class="mt-4">
                <div class="flex gap-2">
                    <!-- TODO: support image upload -->
                    <VaTextarea
                        v-model="editedDetail"
                        placeholder="Enter HTML detail"
                        class="w-3/4"
                        :resize="false"
                        min-rows="9"
                        max-rows="9"
                    />
                    <div class="flex gap-2 w-1/4">
                        <!-- TODO: support add vod -->
                        <!-- TODO: support image upload -->
                        <!-- TODO: support add state upload -->
                        <!-- TODO: support add steam page -->
                    </div>
                </div>
                <div class="flex justify-between mt-2">
                    <VaButton @click="cancelEdit" color="secondary">
                        取消
                    </VaButton>
                    <VaButton @click="saveDetail" color="primary">
                        保存
                    </VaButton>
                </div>
            </div>
            <div v-else class="mt-4">
                <div v-html="penalty.detail"></div>
                <div v-if="authState.isAuthenticated" class="mt-2">
                    <VaButton @click="startEdit" color="success" class="w-full">
                        编辑内容
                    </VaButton>
                </div>
            </div>
        </div>
    </VaModal>
</template>
