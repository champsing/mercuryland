<script setup lang="ts">
import { reactive } from "vue";
import { VaButton, VaModal } from "vuestic-ui";
import axios from "axios";
import { BASE_URL } from "@/composables/utils";
import GoogleSso from "@/components/login/GoogleSso.vue";

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
        <VaModal v-model="modal.show" hide-default-actions close-button max-width="400px">
            <div class="h-full flex items-center justify-center ">
                <GoogleSso v-if="!modal.auth" />
            </div>
        </VaModal>
    </template>
</template>

<style scoped></style>
