<script setup lang="ts">
import { reactive, ref } from "vue";
import { VaButton, VaInput, VaModal } from "vuestic-ui";
import axios from "axios";
import { BASE_URL } from "@/composables/utils";

const modal = reactive({
    show: false,
    auth: false,
    fail: false,
});

const form = reactive({
    code: "",
});

const isSubmitting = ref(false);
const sessionLabel = ref<string | null>(null);

function openLoginModal() {
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
        const response = await axios.post(BASE_URL + "/api/auth/login", {
            code: trimmed,
            ip: clientIP ?? "unknown",
        });
        localStorage.setItem("token", response.data);
        modal.auth = true;
        modal.show = false;
        modal.fail = false;
        sessionLabel.value = "Discord Auth";
        form.code = "";
    } catch (_) {
        modal.fail = true;
    } finally {
        isSubmitting.value = false;
    }
}

function logout() {
    localStorage.removeItem("token");
    modal.auth = false;
    sessionLabel.value = null;

    axios.post(BASE_URL + "/api/auth/logout", {
        ip: clientIP ?? "unknown",
    });
}

function tick() {
    const token = localStorage.getItem("token");
    if (token == null) {
        modal.auth = false;
        sessionLabel.value = null;
        return;
    }

    axios
        .post(BASE_URL + "/api/auth/tick", {
            token,
        })
        .then((response) => {
            localStorage.setItem("token", response.data);
            modal.auth = true;
            if (!sessionLabel.value) {
                sessionLabel.value = "Discord Auth";
            }
        })
        .catch((_) => {
            modal.auth = false;
            sessionLabel.value = null;
        });
}

tick();
setInterval(() => {
    tick();
}, 1000 * 60 * 10);
</script>

<script get-ip lang="ts">
let clientIP = await fetch("https://api.ipify.org?format=json")
    .then((response) => response.json())
    .then((data) => data.ip)
    .catch((error) => {
        console.error("Error fetching IP address:", error);
        return "unknown";
    });
</script>

<template>
    <template v-if="modal.auth">
        <div>
            {{ sessionLabel || "已登入" }}
        </div>
        <VaButton @click="modal.show = true">登出</VaButton>
        <VaModal v-model="modal.show" @ok="logout">
            <div>您确定要登出吗?</div>
        </VaModal>
    </template>
    <template v-else>
        <VaButton @click="openLoginModal">登入</VaButton>
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
                    驗證失敗，請確認驗證碼是否正確或仍在有效期限內。
                </div>
            </div>
        </VaModal>
    </template>
</template>

<style scoped></style>
