<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { VaButton, VaIcon, VaInput, VaModal } from "vuestic-ui";
import axios, { isAxiosError } from "axios";
import { BASE_URL } from "@/composables/utils";
import { useAuthState } from "@/composables/authState";
import { SignInAlt, SignOutAlt } from "@vicons/fa";

const modal = reactive({
    show: false,
    auth: false,
    fail: false,
    errorMessage: "",
});

const form = reactive({
    code: "",
});

const isSubmitting = ref(false);
const authState = useAuthState();
const clientIP = ref("unknown");
const REQUEST_TIMEOUT = 10000;
const IP_LOOKUP_TIMEOUT = 5000;

onMounted(async () => {
    try {
        const controller = new AbortController();
        const timeoutId = window.setTimeout(() => controller.abort(), IP_LOOKUP_TIMEOUT);
        const response = await fetch("https://api.ipify.org?format=json", {
            signal: controller.signal,
        });
        window.clearTimeout(timeoutId);
        const data = await response.json();
        clientIP.value = data.ip ?? "unknown";
    } catch (error) {
        console.error("Error fetching IP address:", error);
    }
});

function openLoginModal() {
    modal.show = true;
    modal.fail = false;
    modal.errorMessage = "";
}

async function authenticate() {
    const trimmed = form.code.trim().toUpperCase();
    if (!trimmed) {
        modal.fail = true;
        modal.errorMessage = "請輸入 8 碼驗證碼";
        return;
    }

    isSubmitting.value = true;
    modal.fail = false;
    modal.errorMessage = "";
    try {
        const response = await axios.post(
            BASE_URL + "/api/auth/login",
            {
                code: trimmed,
                ip: clientIP.value,
            },
            { timeout: REQUEST_TIMEOUT }
        );
        localStorage.setItem("token", response.data);
        modal.auth = true;
        modal.show = false;
        modal.fail = false;
        modal.errorMessage = "";
        form.code = "";
        authState.isAuthenticated = true;
    } catch (error) {
        modal.fail = true;
        if (isAxiosError(error)) {
            if (error.code === "ECONNABORTED") {
                modal.errorMessage = "登入逾時，請確認網路後再試。";
            } else if (error.response?.status === 401) {
                modal.errorMessage = "驗證碼錯誤或已過期。";
            } else {
                modal.errorMessage = "登入失敗，請稍後再試。";
            }
        } else {
            modal.errorMessage = "登入失敗，請稍後再試。";
        }
    } finally {
        isSubmitting.value = false;
    }
}

function logout() {
    localStorage.removeItem("token");
    modal.auth = false;
    authState.isAuthenticated = false;
    modal.fail = false;
    modal.errorMessage = "";
}

function tick() {
    const token = localStorage.getItem("token");
    if (token == null) {
        modal.auth = false;
        authState.isAuthenticated = false;
        return;
    }

    axios
        .post(BASE_URL + "/api/auth/tick", {
            token,
        }, { timeout: REQUEST_TIMEOUT })
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
setInterval(() => {
    tick();
}, 1000 * 60 * 10);
</script>

<template>
    <template v-if="modal.auth">
        <VaButton
            preset="plain"
            aria-label="登出"
            @click="modal.show = true"
        >
            <VaIcon size="large">
                <SignOutAlt />
            </VaIcon>
        </VaButton>
        <VaModal v-model="modal.show" max-width="400px" close-button @ok="logout">
            <div>您确定要登出吗?</div>
        </VaModal>
    </template>
    <template v-else>
        <VaButton
            preset="plain"
            aria-label="登入"
            @click="openLoginModal"
        >
            <VaIcon size="large">
                <SignInAlt />
            </VaIcon>
        </VaButton>
        <VaModal v-model="modal.show" hide-default-actions close-button max-width="400px">
            <div class="flex flex-col gap-4 p-2">
                <VaInput
                    v-model="form.code"
                    label="Discord 驗證碼"
                    placeholder="輸入 8 碼驗證碼"
                    maxlength="8"
                    @keyup.enter="authenticate"
                />
                <VaButton
                    color="primary"
                    :loading="isSubmitting"
                    :disabled="isSubmitting"
                    @click="authenticate"
                >
                    使用驗證碼登入
                </VaButton>
                <div v-if="modal.fail" class="text-red-500 text-sm text-center">
                    {{ modal.errorMessage || "驗證失敗，請稍後再試。" }}
                </div>
            </div>
        </VaModal>
    </template>
</template>

<style scoped></style>
