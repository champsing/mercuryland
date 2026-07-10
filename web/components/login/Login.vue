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
            max-width="380px"
            close-button
            @ok="logout"
        >
            <div
                class="py-4 text-center text-slate-700 dark:text-slate-300 font-medium text-base"
            >
                您確定要登出嗎？
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
            max-width="420px"
        >
            <div
                class="flex flex-col items-center p-3 gap-6 text-slate-800 dark:text-slate-100"
            >
                <div class="text-center space-y-1.5">
                    <h3 class="text-xl font-bold">Discord 驗證碼</h3>
                    <p class="text-sm text-slate-400 dark:text-slate-500">
                        請輸入來自 Discord 的 8 位數驗證碼
                    </p>
                </div>

                <div class="relative w-full max-w-sm mt-2">
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
                                'h-12 rounded-xl border-2 flex items-center justify-center text-lg font-bold transition-all duration-150 bg-slate-50 dark:bg-slate-900 select-none',
                                // 當前正準備輸入的格子高亮（仿 Apple 藍色外框與微放大）
                                isFocused && form.code.length === i - 1
                                    ? 'border-blue-500 ring-4 ring-blue-500/10 scale-105'
                                    : form.code.length > i - 1
                                      ? 'border-slate-400 dark:border-slate-500 text-slate-800 dark:text-slate-100'
                                      : 'border-slate-200 dark:border-slate-800 text-slate-300 dark:text-slate-700',
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
                    class="w-full text-center text-sm font-medium text-rose-500 bg-rose-50 dark:bg-rose-950/30 py-2.5 px-3 rounded-xl border border-rose-100 dark:border-rose-900/30"
                >
                    驗證失敗，請確認驗證碼是否正確或已過期。
                </div>

                <VaButton
                    class="w-full h-11 rounded-xl font-medium tracking-wide shadow-sm"
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

<style scoped></style>
