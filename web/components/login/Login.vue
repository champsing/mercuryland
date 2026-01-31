<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { VaButton, VaIcon, VaInput, VaModal } from "vuestic-ui";
import api from "@composables/axios";

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

function openLoginModal() {
    modal.show = true;
    modal.fail = false;
}

function openLogoutModal() {
    if (!modal.auth) return;
    modal.show = true;
    modal.fail = false;
}

async function authenticate() {
    const trimmed = form.code.trim().toUpperCase();
    if (!trimmed) {
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

defineExpose({ openLoginModal, openLogoutModal });
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
            max-width="400px"
            close-button
            @ok="logout"
        >
            <div>您确定要登出吗?</div>
        </VaModal>
    </template>
    <template v-else>
        <VaButton
            v-if="props.renderTrigger"
            preset="plain"
            aria-label="登入"
            @click="openLoginModal"
        >
            <VaIcon size="32px">
                <SignInAlt />
            </VaIcon>
        </VaButton>
        <VaModal
            v-model="modal.show"
            hide-default-actions
            close-button
            max-width="400px"
        >
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
                    驗證失敗，請確認驗證碼是否正確或仍在有效期限內。
                </div>
            </div>
        </VaModal>
    </template>
</template>

<style scoped></style>
