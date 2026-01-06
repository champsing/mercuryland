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
import { useAuthState } from "@/composables/authState";
import { stateString } from "@/composables/penalty";

const emit = defineEmits<{
    (event: "saved"): void;
}>();

const authState = useAuthState();

const showAddPenaltyModal = ref(false);
const isSavingPenalty = ref(false);
const addPenaltyError = ref<string | null>(null);
const addPenaltySuccess = ref<string | null>(null);
const addPenaltyForm = reactive({
    date: new Date(),
    name: "",
    state: 0,
});

const stateOptions = [
    { value: 0, text: stateString(0) },
    { value: 1, text: stateString(1) },
];

const textBy = (option: { text: string }) => option.text;
const valueBy = (option: { value: number }) => option.value;

watch(showAddPenaltyModal, (visible) => {
    if (!visible) {
        resetAddPenaltyForm();
    }
});

function resetAddPenaltyForm() {
    addPenaltyForm.date = new Date();
    addPenaltyForm.name = "";
    addPenaltyForm.state = 0;
    addPenaltyError.value = null;
    addPenaltySuccess.value = null;
}

const openAddPenaltyModal = () => {
    if (!authState.isAuthenticated) {
        return;
    }
    showAddPenaltyModal.value = true;
};

const savePenalty = async () => {
    if (isSavingPenalty.value || !authState.isAuthenticated) {
        return;
    }

    const token = localStorage.getItem("token");
    if (!token) {
        addPenaltyError.value = "請先登入管理員帳號";
        return;
    }

    if (!addPenaltyForm.date || !addPenaltyForm.name.trim()) {
        addPenaltyError.value = "請填寫日期和內容";
        return;
    }

    const payload = {
        token,
        date: formatDate(addPenaltyForm.date),
        name: addPenaltyForm.name.trim(),
        detail: "",
        state: addPenaltyForm.state,
    };

    try {
        isSavingPenalty.value = true;
        addPenaltyError.value = null;
        await axios.post(`${BASE_URL}/api/penalty/insert`, payload);
        addPenaltySuccess.value = "新增成功";
        emit("saved");
        setTimeout(() => {
            showAddPenaltyModal.value = false;
        }, 600);
    } catch (error) {
        console.error("Failed to insert penalty", error);
        addPenaltyError.value = "新增失敗，請稍後再試";
    } finally {
        isSavingPenalty.value = false;
    }
};

defineExpose({ open: openAddPenaltyModal });
</script>

<template>
    <VaModal
        v-model="showAddPenaltyModal"
        hide-default-actions
        close-button
        no-outside-dismiss
        max-width="480px"
    >
        <div class="flex flex-col gap-4 p-4">
            <div class="text-lg font-semibold text-zinc-200">新增懲罰</div>
            <VaDateInput
                v-model="addPenaltyForm.date"
                label="日期"
                color="info"
                :format-date="formatDate"
                :parse-date="parseDate"
                manual-input
                mode="auto"
                required
            />
            <VaTextarea
                v-model="addPenaltyForm.name"
                label="內容"
                color="info"
                :resize="false"
                min-rows="3"
                max-rows="3"
                required
            />
            <!-- prettier-ignore -->
            <VaSelect
                v-model="addPenaltyForm.state"
                label="狀態"
                color="info"
                :options="stateOptions"
                :text-by="textBy"
                :value-by="valueBy"
                required
            />
            <p v-if="addPenaltyError" class="text-sm text-red-400">
                {{ addPenaltyError }}
            </p>
            <p v-if="addPenaltySuccess" class="text-sm text-emerald-400">
                {{ addPenaltySuccess }}
            </p>
            <div class="flex justify-end gap-2">
                <VaButton
                    color="secondary"
                    @click="showAddPenaltyModal = false"
                >
                    取消
                </VaButton>
                <VaButton
                    color="info"
                    :disabled="isSavingPenalty"
                    @click="savePenalty"
                >
                    {{ isSavingPenalty ? "儲存中..." : "新增" }}
                </VaButton>
            </div>
        </div>
    </VaModal>
</template>
