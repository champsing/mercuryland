<script setup lang="ts">
import { useAuthState } from "@/composables/authState";
// 🌟 引入我們寫好的 usePenaltyDetail 組合式函數
import { PenItem, stateString, usePenaltyDetail } from "@/composables/penalty";
import { formatDate, parseDate } from "@/composables/utils";
import api from "@composables/axios";
import { reactive, ref, watch } from "vue"; // 🌟 記得引入 watch
import { VaButton, VaModal, VaTab, VaTabs } from "vuestic-ui";
import Detail from "./tabs/Detail.vue";
import History from "./tabs/History.vue";
import Status from "./tabs/Status.vue";

const emit = defineEmits<{
    (event: "updated"): void;
    (event: "deleted"): void;
}>();

const authState = useAuthState();

const showModal = ref(false);
const activeTab = ref<"status" | "detail" | "history">("status");

// 🌟 解構出共用的詳情獲取邏輯（將 penalty 重新命名為 fullPenalty 避免名稱衝突）
const { penalty: fullPenalty, loadPenalty } = usePenaltyDetail();

// ---------- 狀態 ----------
const statusForm = reactive({
    id: 0,
    date: new Date(),
    name: "",
    state: 0,
});
const isSavingstatus = ref(false);
const statusError = ref<string | null>(null);
const statusSuccess = ref<string | null>(null);

const stateOptions = [
    { value: 0, text: stateString(0) },
    { value: 1, text: stateString(1) },
    { value: 2, text: stateString(2) },
    { value: 3, text: stateString(3) },
    { value: 4, text: stateString(4) },
];

// ---------- 详情 ----------
const detailContent = ref("");
const isSavingDetail = ref(false);
const detailError = ref<string | null>(null);
const detailSuccess = ref<string | null>(null);

// ---------- 历史 ----------
interface HistoryEntry {
    state: number;
    date: Date;
}
const historyEntries = ref<HistoryEntry[]>([]);
const isSavingHistory = ref(false);
const historyError = ref<string | null>(null);
const historySuccess = ref<string | null>(null);

// ---------- 删除确认 ----------
const showDeleteConfirm = ref(false);
const isDeleting = ref(false);
const deleteError = ref<string | null>(null);

// 🌟 監聽非同步載入的完整懲罰資料，一旦後端回傳，立刻填入 Detail 與 History 的 ref 中
watch(fullPenalty, (newPenalty) => {
    if (newPenalty) {
        detailContent.value = newPenalty.detail ?? "";
        historyEntries.value = newPenalty.history
            ? newPenalty.history.map(([state, dateStr]) => ({
                  state,
                  date: parseDate(dateStr),
              }))
            : [];
    }
});

// ---------- 打开 Modal ----------
function open(penalty: PenItem) {
    if (!authState.isAuthenticated) return;

    // 1. 先用清單傳進來的基本資料填充（讓使用者點開時不感覺卡頓）
    statusForm.id = penalty.id;
    statusForm.date = parseDate(penalty.date);
    statusForm.name = penalty.name;
    statusForm.state = penalty.state;

    // 2. 先清空舊的詳情與歷史，防止前一次編輯的資料殘留「閃爍」一下
    detailContent.value = "";
    historyEntries.value = [];
    fullPenalty.value = null;

    // 3. 重置所有狀態
    statusError.value = null;
    statusSuccess.value = null;
    detailError.value = null;
    detailSuccess.value = null;
    historyError.value = null;
    historySuccess.value = null;
    deleteError.value = null;
    showDeleteConfirm.value = false;
    activeTab.value = "status";
    showModal.value = true;

    // 4. 🌟 呼叫 Composable 函數，去後端拉取該 ID 的完整資料 (包含 detail, history)
    loadPenalty(penalty.id);
}

// ---------- 保存各标签 ----------
async function savestatus() {
    if (isSavingstatus.value || !authState.isAuthenticated) return;
    const token = localStorage.getItem("token");
    if (!token) {
        statusError.value = "請先登入管理員帳號";
        return;
    }
    if (!statusForm.date || !statusForm.name.trim()) {
        statusError.value = "請填寫日期和內容";
        return;
    }
    try {
        isSavingstatus.value = true;
        statusError.value = null;
        statusSuccess.value = null;
        await api.post("/api/penalty/update", {
            token,
            id: statusForm.id,
            date: formatDate(statusForm.date),
            name: statusForm.name.trim(),
            state: statusForm.state,
        });
        statusSuccess.value = "更新成功";
        emit("updated");
        setTimeout(() => (showModal.value = false), 600);
    } catch (error) {
        console.error(error);
        statusError.value = "更新失敗，請稍後再試";
    } finally {
        isSavingstatus.value = false;
    }
}

async function saveDetail() {
    if (isSavingDetail.value || !authState.isAuthenticated) return;
    const token = localStorage.getItem("token");
    if (!token) {
        detailError.value = "請先登入管理員帳號";
        return;
    }
    try {
        isSavingDetail.value = true;
        detailError.value = null;
        detailSuccess.value = null;
        await api.post("/api/penalty/detail/update", {
            token,
            id: statusForm.id,
            detail: detailContent.value,
        });
        detailSuccess.value = "更新成功";
        emit("updated");
    } catch (error) {
        console.error(error);
        detailError.value = "更新失敗，請稍後再試";
    } finally {
        isSavingDetail.value = false;
    }
}

async function saveHistory() {
    if (isSavingHistory.value || !authState.isAuthenticated) return;
    const token = localStorage.getItem("token");
    if (!token) {
        historyError.value = "請先登入管理員帳號";
        return;
    }
    try {
        isSavingHistory.value = true;
        historyError.value = null;
        historySuccess.value = null;
        const sorted = [...historyEntries.value].sort(
            (a, b) => a.date.getTime() - b.date.getTime(),
        );
        const historyPayload = sorted.map(({ state, date }) => [
            state,
            formatDate(date),
        ]);
        await api.post("/api/penalty/history/update", {
            token,
            id: statusForm.id,
            history: historyPayload,
        });
        historySuccess.value = "更新成功";
        emit("updated");
    } catch (error) {
        console.error(error);
        historyError.value = "更新失敗，請稍後再試";
    } finally {
        isSavingHistory.value = false;
    }
}

async function deletePenalty() {
    if (isDeleting.value || !authState.isAuthenticated) return;
    const token = localStorage.getItem("token");
    if (!token) {
        deleteError.value = "請先登入管理員帳號";
        return;
    }
    try {
        isDeleting.value = true;
        deleteError.value = null;
        await api.post("/api/penalty/delete", {
            token,
            id: statusForm.id,
        });
        emit("deleted");
        showDeleteConfirm.value = false;
        showModal.value = false;
    } catch (error) {
        console.error(error);
        deleteError.value = "刪除失敗，請稍後再試";
    } finally {
        isDeleting.value = false;
    }
}

defineExpose({ open });
</script>

<template>
    <VaModal
        v-model="showModal"
        hide-default-actions
        close-button
        :mobile-fullscreen="false"
        no-outside-dismiss
        size="small"
    >
        <div
            class="p-2 w-full max-h-[80vh] flex flex-col gap-4 text-slate-800 dark:text-slate-100"
        >
            <VaTabs v-model="activeTab" color="#a3e635" center>
                <template #tabs>
                    <VaTab name="status" color="#a3e635">狀態資料</VaTab>
                    <VaTab name="detail" color="#a3e635">詳情</VaTab>
                    <VaTab name="history" color="#a3e635">歷史</VaTab>
                </template>
            </VaTabs>

            <div class="flex-1 overflow-y-auto custom-scrollbar pr-1 pt-4">
                <Status
                    v-if="activeTab === 'status'"
                    v-model:date="statusForm.date"
                    v-model:name="statusForm.name"
                    v-model:state="statusForm.state"
                    :state-options="stateOptions"
                    :saving="isSavingstatus"
                    :error="statusError"
                    :success="statusSuccess"
                    @save="savestatus"
                    @delete="showDeleteConfirm = true"
                    @cancel="showModal = false"
                />

                <Detail
                    v-if="activeTab === 'detail'"
                    v-model:detail="detailContent"
                    :saving="isSavingDetail"
                    :error="detailError"
                    :success="detailSuccess"
                    @save="saveDetail"
                    @back="activeTab = 'status'"
                />

                <History
                    v-if="activeTab === 'history'"
                    v-model:entries="historyEntries"
                    :state-options="stateOptions"
                    :saving="isSavingHistory"
                    :error="historyError"
                    :success="historySuccess"
                    @save="saveHistory"
                    @back="activeTab = 'status'"
                />
            </div>
        </div>

        <!-- 删除确认弹窗 -->
        <VaModal
            v-model="showDeleteConfirm"
            hide-default-actions
            close-button
            :mobile-fullscreen="false"
            max-width="200px"
        >
            <div class="flex flex-col gap-4 p-2">
                <div class="text-lg font-semibold text-zinc-200">確認刪除</div>
                <p class="text-sm text-zinc-300">
                    確定要刪除這個懲罰嗎？此操作無法復原。
                </p>
                <p v-if="deleteError" class="text-sm text-red-400">
                    {{ deleteError }}
                </p>
                <div class="flex justify-end gap-2">
                    <VaButton
                        color="secondary"
                        @click="showDeleteConfirm = false"
                    >
                        取消
                    </VaButton>
                    <VaButton
                        color="danger"
                        :disabled="isDeleting"
                        @click="deletePenalty"
                    >
                        {{ isDeleting ? "刪除中..." : "刪除" }}
                    </VaButton>
                </div>
            </div>
        </VaModal>
    </VaModal>
</template>

<style scoped>
/* 深度穿透修改 Vuestic UI Modal 本體，將其改造為 Apple 風格的玻璃擬態，對標 Detail.vue */
:deep(.va-modal__dialog) {
    background: rgba(255, 255, 255, 0.1) !important;
    backdrop-filter: blur(20px) !important;
    -webkit-backdrop-filter: blur(20px) !important;
    border-radius: 24px !important;
    border: 1px solid rgba(255, 255, 255, 0.4) !important;
    box-shadow:
        0 20px 40px -15px rgba(0, 0, 0, 0.08),
        0 0 0 1px rgba(0, 0, 0, 0.02) !important;
    max-width: 680px !important;
    width: 95% !important;
    transition: all 0.3s ease;
}

:global(.dark) :deep(.va-modal__dialog),
:global(.va-theme--dark) :deep(.va-modal__dialog) {
    background: rgba(15, 23, 42, 0.65) !important;
    border: 1px solid rgba(255, 255, 255, 0.08) !important;
    box-shadow:
        0 25px 50px -12px rgba(0, 0, 0, 0.35),
        0 0 0 1px rgba(255, 255, 255, 0.04) !important;
}

/* 修正關閉按鈕的位置 */
:deep(.va-modal__close) {
    top: 1rem !important;
    right: 1rem !important;
    color: currentColor !important;
    opacity: 0.6;
}

/* 嵌套刪除確認彈窗維持較小尺寸 */
:deep(.va-modal__child .va-modal__dialog) {
    max-width: 360px !important;
}

/* Custom Scrollbar */
.custom-scrollbar::-webkit-scrollbar {
    width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
    @apply bg-slate-500/20 hover:bg-slate-500/40 dark:bg-slate-400/15 dark:hover:bg-slate-400/35 rounded-full;
}
</style>
