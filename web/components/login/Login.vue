<script setup lang="ts">
import { reactive } from "vue";
import { VaInput, VaButton, VaModal } from "vuestic-ui";
import axios from "axios";
import { BASE_URL } from "@/composables/utils";

const modal = reactive({
    show: false, // should we show the modal
    auth: false, // is currently authenticated
    fail: false, // if it's login failed
});

// const modal2 = reactive({
//     show: false, // should we show the modal
// });

const form = reactive({
    username: "",
    password: "",
});

let sessionUsername = reactive(null);

function beforeOk(hide?: CallableFunction) {
    axios
        .post(BASE_URL + "/api/auth/login", {
            username: form.username,
            password: form.password,
            ip: clientIP,
        })
        .then((response) => {
            localStorage.setItem("token", response.data);
            sessionUsername = form.username;
            form.username = "";
            form.password = "";
            modal.auth = true;

            let log =
                "[LOGIN] User " +
                sessionUsername +
                " logged in on " +
                new Date(Date.now()) +
                " at " +
                clientIP +
                ".";
            console.log(log);

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

    let log =
        "[LOGOUT] User " +
        sessionUsername +
        " logged out on " +
        new Date(Date.now()) +
        " at " +
        clientIP +
        ".";
    console.log(log);

    axios.post(BASE_URL + "/api/auth/login", {
        username: sessionUsername,
        ip: clientIP,
    });
    sessionUsername = null;
}

// auth token
function tick() {
    let token = localStorage.getItem("token");
    if (token == null) modal.auth = false;
    else {
        axios
            .post(BASE_URL + "/api/auth/tick", {
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

<script get-ip lang="ts">
// Get the client's IP address 原來要await他resolve
let clientIP = await fetch("https://api.ipify.org?format=json")
    .then((response) => response.json())
    .then((data) => data.ip)
    .catch((error) => console.error("Error fetching IP address:", error));

function handleCredentialResponse(response) {
    console.log("Encoded JWT ID token: " + response.credential);
}

window.onload = function () {
    //@ts-ignore
    google.accounts.id.initialize({
        client_id:
            "557016419724-4gvamcsq0hp8j8e0o1sjum2epq5ls446.apps.googleusercontent.com",
        callback: handleCredentialResponse,
    });
    //@ts-ignore
    google.accounts.id.prompt(); // also display the One Tap dialog
    //@ts-ignore
    google.accounts.id.renderButton(
        document.getElementById("sign-in-with-google"),
        {
            type: "standard",
            size: "large",
            theme: "outline",
            text: "sign_in_with",
            shape: "rectangular",
            logo_alignment: "left",
        }
    );
};
</script>

<template>
    <template v-if="modal.auth">
        <div>
            {{ sessionUsername }}
        </div>
        <!-- <VaButton @click="modal2.show = true">我的帳號</VaButton> -->
        <VaButton @click="modal.show = true">登出</VaButton>
        <VaModal v-model="modal.show" @ok="logout()">
            <div>您确定要登出吗?</div>
        </VaModal>
    </template>
    <template v-else>
        <VaButton @click="modal.show = true">登入</VaButton>
        <VaModal
            v-model="modal.show"
            ok-text="登录"
            cancel-text="取消"
            :before-ok="beforeOk"
            :before-cancel="beforeCancel"
        >
            <div
                id="username-sign-in"
                class="flex items-baseline justify-evenly h-14"
            >
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
            <div class="google-btn-container">
                <div
                    id="sign-in-with-google"
                    class="text-center mt-6 mr-2"
                    v-if="!modal.auth"
                ></div>
            </div>
        </VaModal>
    </template>
</template>

<style scoped></style>
