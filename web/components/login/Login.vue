<script setup lang="ts">
import api from "@composables/axios";
import { onMounted, reactive, ref } from "vue";
import { VaButton, VaIcon, VaModal } from "vuestic-ui";

import { useAuthState } from "@/composables/authState";
import { SignInAlt, SignOutAlt } from "@vicons/fa";

const props = defineProps({
    renderTrigger: {
        type: Boolean,
        default: true,
    },
});

const modal = reactive({
    show: false,
    auth: false,
    fail: false,
});

const form = reactive({
    code: "",
});

const isSubmitting = ref(false);
const isFocused = ref(false); // 追蹤輸入框是否聚焦
const authState = useAuthState();
const clientIP = ref("unknown");

onMounted(async () => {
    try {
        const response = await fetch("https://api.ipify.org?format=json");
        const data = await response.json();
        clientIP.value = data.ip ?? "unknown";
    } catch (error) {
        console.error("Error fetching IP address:", error);
    }
});

function openLoginLogin() {
    modal.show = true;
    modal.fail = false;
    form.code = ""; // 開啟時清空
}

function openLogoutModal() {
    if (!modal.auth) return;
    modal.show = true;
    modal.fail = false;
}

async function authenticate() {
    const trimmed = form.code.trim().toUpperCase();
    if (trimmed.length !== 8) {
        modal.fail = true;
        return;
    }

    isSubmitting.value = true;
    try {
        const response = await api.post("/api/auth/login", {
            code: trimmed,
            ip: clientIP.value,
        });
        localStorage.setItem("token", response.data);
        modal.auth = true;
        modal.show = false;
        modal.fail = false;
        form.code = "";
        authState.isAuthenticated = true;
    } catch (_) {
        modal.fail = true;
    } finally {
        isSubmitting.value = false;
    }
}

function logout() {
    localStorage.removeItem("token");
    modal.auth = false;
    authState.isAuthenticated = false;
}

function tick() {
    const token = localStorage.getItem("token");
    if (token == null) {
        modal.auth = false;
        authState.isAuthenticated = false;
        return;
    }

    api.post("/api/auth/tick", {
        token,
    })
        .then((response) => {
            localStorage.setItem("token", response.data);
            modal.auth = true;
            authState.isAuthenticated = true;
        })
        .catch((_) => {
            modal.auth = false;
            authState.isAuthenticated = false;
        });
}

tick();
setInterval(
    () => {
        tick();
    },
    1000 * 60 * 10,
);

defineExpose({ openLoginModal: openLoginLogin, openLogoutModal });
</script>

<template>
    <template v-if="modal.auth">
        <VaButton
            v-if="props.renderTrigger"
            preset="plain"
            aria-label="登出"
            @click="openLogoutModal"
        >
            <VaIcon size="32px">
                <SignOutAlt />
            </VaIcon>
        </VaButton>
        <VaModal
            v-model="modal.show"
            hide-default-actions
            close-button
            max-width="400px"
        >
            <div
                class="flex flex-col items-center p-4 sm:p-5 gap-6 text-slate-800 dark:text-slate-100"
            >
                <!-- 頂部安全警告圖示（柔和紅底） -->
                <div
                    class="w-14 h-14 bg-rose-50 dark:bg-rose-950/30 text-rose-500 dark:text-rose-400 rounded-full flex items-center justify-center shadow-inner"
                >
                    <VaIcon size="24px">
                        <SignOutAlt />
                    </VaIcon>
                </div>

                <!-- 正式標題與說明文字 -->
                <div class="text-center space-y-2">
                    <h3 class="text-2xl font-bold tracking-wide">登出</h3>
                    <p
                        class="text-sm text-slate-500 dark:text-slate-400 leading-relaxed"
                    >
                        您確定要結束目前的登入狀態嗎？<br />
                        為了保護您的資訊安全，請務必在編輯完成後登出。
                    </p>
                </div>

                <!-- 底部正式按鈕列 -->
                <div class="flex w-full gap-3 mt-1">
                    <VaButton
                        class="flex-1 h-12 rounded-2xl font-semibold text-base transition-colors duration-200"
                        preset="secondary"
                        border-color="secondary"
                        @click="modal.show = false"
                    >
                        取消
                    </VaButton>
                    <VaButton
                        class="flex-1 h-12 rounded-2xl font-semibold tracking-wider text-base shadow-md transition-colors duration-200"
                        color="danger"
                        @click="logout"
                    >
                        確認登出
                    </VaButton>
                </div>
            </div>
        </VaModal>
    </template>

    <template v-else>
        <VaButton
            v-if="props.renderTrigger"
            preset="plain"
            aria-label="登入"
            @click="openLoginLogin"
        >
            <VaIcon size="32px">
                <SignInAlt />
            </VaIcon>
        </VaButton>

        <VaModal
            v-model="modal.show"
            hide-default-actions
            close-button
            max-width="440px"
        >
            <div
                class="flex flex-col items-center p-4 sm:p-5 gap-6 text-slate-800 dark:text-slate-100"
            >
                <div class="text-center space-y-2">
                    <h3 class="text-2xl font-bold tracking-wide">
                        Discord 驗證碼
                    </h3>
                    <p class="text-sm text-slate-500 dark:text-slate-400">
                        請輸入來自 Discord 的 8 位數驗證碼
                    </p>
                </div>

                <div class="relative w-full max-w-sm mt-1">
                    <input
                        v-model="form.code"
                        type="text"
                        maxlength="8"
                        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
                        autocomplete="one-time-code"
                        @focus="isFocused = true"
                        @blur="isFocused = false"
                        @input="form.code = form.code.toUpperCase()"
                        @keyup.enter="authenticate"
                    />

                    <div class="grid grid-cols-8 gap-1.5 w-full justify-center">
                        <div
                            v-for="i in 8"
                            :key="i"
                            :class="[
                                'h-12 rounded-2xl border-2 flex items-center justify-center text-lg font-bold transition-all duration-200 select-none',
                                // 調整驗證碼格子的半透明背景，使其更好地融入玻璃背板
                                isFocused && form.code.length === i - 1
                                    ? 'border-blue-500 ring-4 ring-blue-500/10 scale-105 bg-white/80 dark:bg-slate-900/80'
                                    : form.code.length > i - 1
                                      ? 'border-slate-400 dark:border-slate-500 text-slate-800 dark:text-slate-100 bg-white/50 dark:bg-slate-950/40'
                                      : 'border-slate-200/60 dark:border-slate-800/80 text-slate-300 dark:text-slate-700 bg-white/20 dark:bg-slate-950/20',
                            ]"
                        >
                            <span v-if="form.code[i - 1]">{{
                                form.code[i - 1]
                            }}</span>

                            <div
                                v-else-if="
                                    isFocused && form.code.length === i - 1
                                "
                                class="w-[2px] h-5 bg-blue-500 animate-pulse"
                            ></div>
                        </div>
                    </div>
                </div>

                <div
                    v-if="modal.fail"
                    class="w-full text-center text-sm font-medium text-rose-500 bg-rose-50/60 dark:bg-rose-950/20 py-3 px-4 rounded-2xl border border-rose-100/70 dark:border-rose-900/30 backdrop-blur-sm animate-fade-in"
                >
                    驗證失敗，請確認驗證碼是否正確或是否已使用、已過期等。
                </div>

                <VaButton
                    class="w-full h-12 rounded-2xl font-semibold tracking-wider shadow-md text-base"
                    color="primary"
                    :loading="isSubmitting"
                    :disabled="isSubmitting || form.code.length !== 8"
                    @click="authenticate"
                >
                    驗證並登入
                </VaButton>
            </div>
        </VaModal>
    </template>
</template>

<style scoped>
/* 深度穿透修改 Vuestic UI Modal 本體，將其改造為 Apple 風格的玻璃擬態 */
:deep(.va-modal__dialog) {
    /* 輕度白透背景 */
    background: rgba(255, 255, 255, 0.1) !important;
    /* 核心：高強度的背景模糊 */
    backdrop-filter: blur(20px) !important;
    -webkit-backdrop-filter: blur(20px) !important;
    /* 圓角升級（3xl / 24px） */
    border-radius: 24px !important;
    /* 玻璃邊框高光 */
    border: 1px solid rgba(255, 255, 255, 0.4) !important;
    /* 層次感陰影 */
    box-shadow:
        0 20px 40px -15px rgba(0, 0, 0, 0.08),
        0 0 0 1px rgba(0, 0, 0, 0.02) !important;
    transition: all 0.3s ease;
}

/* 兼容 Tailwind 的 .dark 與 Vuestic UI 的暗黑模式主題 */
:global(.dark) :deep(.va-modal__dialog),
:global(.va-theme--dark) :deep(.va-modal__dialog) {
    /* 深色調半透背景 */
    background: rgba(15, 23, 42, 0.65) !important;
    /* 暗色玻璃邊框 */
    border: 1px solid rgba(255, 255, 255, 0.08) !important;
    box-shadow:
        0 25px 50px -12px rgba(0, 0, 0, 0.35),
        0 0 0 1px rgba(255, 255, 255, 0.04) !important;
}

/* 稍微修正關閉按鈕的位置，使其與大圓角更協調 */
:deep(.va-modal__close) {
    top: 1rem !important;
    right: 1rem !important;
    color: cubic-bezier(0.4, 0, 0.2, 1);
}
</style>
