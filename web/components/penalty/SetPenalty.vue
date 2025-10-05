<script setup lang="ts">
import { reactive, ref, watch } from "vue";
import {
    VaButton,
    VaDateInput,
    VaModal,
    VaSelect,
    VaTextarea,
} from "vuestic-ui";
import axios from "axios";
import { BASE_URL, formatDate, parseDate } from "@/composables/utils";
import { PenItem, stateString } from "@/composables/penalty";
import { useAuthState } from "@/composables/authState";

const emit = defineEmits<{
    (event: "updated"): void;
    (event: "deleted"): void;
}>();

const authState = useAuthState();

const showEditPenaltyModal = ref(false);
const showDeleteConfirmModal = ref(false);
const isSavingPenalty = ref(false);
const isDeletingPenalty = ref(false);
const editPenaltyError = ref<string | null>(null);
const editPenaltySuccess = ref<string | null>(null);
const deletePenaltyError = ref<string | null>(null);
const editPenaltyForm = reactive({
    id: 0,
    date: new Date(),
    name: "",
    state: 0,
});

const stateOptions = [
    { value: 0, text: stateString(0) },
    { value: 1, text: stateString(1) },
    { value: 2, text: stateString(2) },
    { value: 3, text: stateString(3) },
    { value: 4, text: stateString(4) },
];

const textBy = (option: { text: string }) => option.text;
const valueBy = (option: { value: number }) => option.value;

watch(showEditPenaltyModal, (visible) => {
    if (!visible) {
        resetEditPenaltyForm();
    }
});

function resetEditPenaltyForm() {
    editPenaltyForm.id = 0;
    editPenaltyForm.date = new Date();
    editPenaltyForm.name = "";
    editPenaltyForm.state = 0;
    editPenaltyError.value = null;
    editPenaltySuccess.value = null;
    showDeleteConfirmModal.value = false;
    deletePenaltyError.value = null;
    isDeletingPenalty.value = false;
}

const open = (penalty: PenItem) => {
    if (!authState.isAuthenticated) {
        return;
    }
    editPenaltyForm.id = penalty.id;
    editPenaltyForm.date = parseDate(penalty.date);
    editPenaltyForm.name = penalty.name;
    editPenaltyForm.state = penalty.state;
    editPenaltyError.value = null;
    editPenaltySuccess.value = null;
    deletePenaltyError.value = null;
    showDeleteConfirmModal.value = false;
    showEditPenaltyModal.value = true;
};

const savePenalty = async () => {
    if (isSavingPenalty.value || !authState.isAuthenticated) {
        return;
    }

    const token = localStorage.getItem("token");
    if (!token) {
        editPenaltyError.value = "請先登入管理員帳號";
        return;
    }

    if (!editPenaltyForm.date || !editPenaltyForm.name.trim()) {
        editPenaltyError.value = "請填寫日期和內容";
        return;
    }

    const payload = {
        token,
        id: editPenaltyForm.id,
        date: formatDate(editPenaltyForm.date),
        name: editPenaltyForm.name.trim(),
        state: editPenaltyForm.state,
    };

    try {
        isSavingPenalty.value = true;
        editPenaltyError.value = null;
        await axios.post(`${BASE_URL}/api/penalty/update`, payload);
        editPenaltySuccess.value = "更新成功";
        emit("updated");
        setTimeout(() => {
            showEditPenaltyModal.value = false;
        }, 600);
    } catch (error) {
        console.error("Failed to update penalty", error);
        editPenaltyError.value = "更新失敗，請稍後再試";
    } finally {
        isSavingPenalty.value = false;
    }
};

const requestDeletePenalty = () => {
    deletePenaltyError.value = null;
    showDeleteConfirmModal.value = true;
};

const deletePenalty = async () => {
    if (isDeletingPenalty.value || !authState.isAuthenticated) {
        return;
    }

    const token = localStorage.getItem("token");
    if (!token) {
        deletePenaltyError.value = "請先登入管理員帳號";
        return;
    }

    try {
        isDeletingPenalty.value = true;
        deletePenaltyError.value = null;
        await axios.post(`${BASE_URL}/api/penalty/delete`, {
            token,
            id: editPenaltyForm.id,
        });
        emit("deleted");
        showDeleteConfirmModal.value = false;
        showEditPenaltyModal.value = false;
    } catch (error) {
        console.error("Failed to delete penalty", error);
        deletePenaltyError.value = "刪除失敗，請稍後再試";
    } finally {
        isDeletingPenalty.value = false;
    }
};

const closeDeleteModal = () => {
    showDeleteConfirmModal.value = false;
};

defineExpose({ open });
</script>

<template>
    <VaModal
        v-model="showEditPenaltyModal"
        hide-default-actions
        close-button
        max-width="480px"
    >
        <div class="flex flex-col gap-4 p-4">
            <div class="text-lg font-semibold text-zinc-200">編輯懲罰</div>
            <VaDateInput
                v-model="editPenaltyForm.date"
                label="日期"
                color="info"
                :format-date="formatDate"
                :parse-date="parseDate"
                manual-input
                mode="auto"
                required
            />
            <VaTextarea
                v-model="editPenaltyForm.name"
                label="內容"
                color="info"
                :resize="false"
                min-rows="3"
                max-rows="3"
                required
            />
            <!-- prettier-ignore -->
            <VaSelect
                v-model="editPenaltyForm.state"
                label="狀態"
                color="info"
                :options="stateOptions"
                :text-by="textBy"
                :value-by="valueBy"
                required
            />
            <p v-if="editPenaltyError" class="text-sm text-red-400">
                {{ editPenaltyError }}
            </p>
            <p v-if="editPenaltySuccess" class="text-sm text-emerald-400">
                {{ editPenaltySuccess }}
            </p>
            <div class="flex justify-between gap-2">
                <VaButton
                    color="danger"
                    :disabled="isSavingPenalty"
                    @click="requestDeletePenalty"
                >
                    刪除
                </VaButton>
                <div class="flex gap-2">
                    <VaButton
                        color="secondary"
                        @click="showEditPenaltyModal = false"
                    >
                        取消
                    </VaButton>
                    <VaButton
                        color="info"
                        :disabled="isSavingPenalty"
                        @click="savePenalty"
                    >
                        {{ isSavingPenalty ? "儲存中..." : "更新" }}
                    </VaButton>
                </div>
            </div>
        </div>
    </VaModal>

    <VaModal
        v-model="showDeleteConfirmModal"
        hide-default-actions
        close-button
        max-width="360px"
    >
        <div class="flex flex-col gap-4 p-4">
            <div class="text-lg font-semibold text-zinc-200">確認刪除</div>
            <p class="text-sm text-zinc-300">
                確定要刪除這個懲罰嗎？此操作無法復原。
            </p>
            <p v-if="deletePenaltyError" class="text-sm text-red-400">
                {{ deletePenaltyError }}
            </p>
            <div class="flex justify-end gap-2">
                <VaButton color="secondary" @click="closeDeleteModal">
                    取消
                </VaButton>
                <VaButton
                    color="danger"
                    :disabled="isDeletingPenalty"
                    @click="deletePenalty"
                >
                    {{ isDeletingPenalty ? "刪除中..." : "刪除" }}
                </VaButton>
            </div>
        </div>
    </VaModal>
</template>
