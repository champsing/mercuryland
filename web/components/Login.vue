<script setup lang="ts">
import { reactive } from "vue";
import { VaInput, VaButton, VaModal } from "vuestic-ui";
import axios from "axios";

const modal = reactive({
    show: false, // should we show the modal
    auth: false, // is currently authenticated
    fail: false, // if it's login failed
});

const form = reactive({
    username: "",
    password: "",
});

function beforeOk(hide: CallableFunction) {
    axios
        .post("/api/auth/login", {
            username: form.username,
            password: form.password,
        })
        .then((response) => {
            localStorage.setItem("token", response.data);
            form.username = "";
            form.password = "";
            modal.auth = true;
            hide();
        })
        .catch((_) => {
            form.password = "";
            modal.fail = true;
            modal.auth = false;
        });
}

function beforeCancel(hide: CallableFunction) {
    form.username = "";
    form.password = "";
    hide();
}

function logout() {
    localStorage.removeItem("token");
    modal.auth = false;
}

// auth token
function tick() {
    let token = localStorage.getItem("token");
    if (token == null) modal.auth = false;
    else {
        axios
            .post("/api/auth/tick", {
                token: token,
            })
            .then((response) => {
                localStorage.setItem("token", response.data);
                modal.auth = true;
            })
            .catch((_) => {
                modal.auth = false;
            });
    }
}
tick();
setInterval(() => {
    tick();
}, 1000 * 60 * 10); // 10 minutes
</script>

<template>
    <template v-if="modal.auth">
        <VaButton @click="modal.show = true">登出</VaButton>
        <VaModal v-model="modal.show" @ok="logout()">
            <div>您确定要登出吗?</div>
        </VaModal>
    </template>
    <template v-else>
        <VaButton @click="modal.show = true">登录</VaButton>
        <VaModal
            v-model="modal.show"
            ok-text="登录"
            cancel-text="取消"
            :before-ok="beforeOk"
            :before-cancel="beforeCancel"
        >
            <div class="flex items-baseline justify-evenly h-14">
                <VaInput
                    v-model="form.username"
                    label="Username"
                    name="Username"
                />
                <VaInput
                    v-model="form.password"
                    label="Password"
                    type="password"
                    name="Password"
                    immediate-validation
                    error-messages="Login failed"
                    :error="modal.fail"
                    @input="modal.fail = false"
                />
            </div>
        </VaModal>
    </template>
</template>

<style scoped></style>
