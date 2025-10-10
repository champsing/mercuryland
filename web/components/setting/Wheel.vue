<script setup lang="ts">
import { ref } from "vue";
import axios from "axios";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaCardTitle,
    VaInput,
    VaModal,
} from "vuestic-ui";
import { BASE_URL } from "@/composables/utils";

const showWheelPasswordModal = ref(false);
const showPenaltyChannelModal = ref(false);
const wheelPassword = ref("");
const penaltyChannel = ref("");

async function openWheelPasswordModal() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        const response = await axios.get(
            `${BASE_URL}/api/setting/config?token=${token}&id=0`,
        );
        wheelPassword.value = response.data.value;
        showWheelPasswordModal.value = true;
    } catch (error) {
        console.error("Failed to fetch wheel password", error);
    }
}

async function saveWheelPassword() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        await axios.post(`${BASE_URL}/api/setting/config`, {
            token,
            id: 0,
            value: wheelPassword.value,
        });
        showWheelPasswordModal.value = false;
    } catch (error) {
        console.error("Failed to save wheel password", error);
    }
}

async function openPenaltyChannelModal() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        const response = await axios.get(
            `${BASE_URL}/api/setting/config?token=${token}&id=1`,
        );
        penaltyChannel.value = response.data.value;
        showPenaltyChannelModal.value = true;
    } catch (error) {
        console.error("Failed to fetch penalty channel", error);
    }
}

async function savePenaltyChannel() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        await axios.post(`${BASE_URL}/api/setting/config`, {
            token,
            id: 1,
            value: penaltyChannel.value,
        });
        showPenaltyChannelModal.value = false;
    } catch (error) {
        console.error("Failed to save penalty channel", error);
    }
}
</script>

<template>
    <VaCard class="rounded-xl border border-zinc-700">
        <VaCardTitle
            class="px-6 pt-6 !text-xl font-medium text-zinc-200 justify-center"
        >
            转盘惩罚
        </VaCardTitle>
        <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
            <div class="flex gap-2">
                <VaButton
                    preset="primary"
                    color="info"
                    class="flex-1"
                    @click="openWheelPasswordModal"
                >
                    转盘密码
                </VaButton>
                <VaButton
                    preset="primary"
                    color="info"
                    class="flex-1"
                    @click="openPenaltyChannelModal"
                >
                    惩罚频道
                </VaButton>
            </div>
        </VaCardContent>
    </VaCard>

    <VaModal
        v-model="showWheelPasswordModal"
        @ok="saveWheelPassword"
        max-width="400px"
        close-button
        ok-text="保存"
        cancel-text="取消"
    >
        <VaInput
            v-model="wheelPassword"
            label="转盘密码"
            placeholder="更改转盘密码"
            type="password"
            class="w-full"
        />
    </VaModal>

    <VaModal
        v-model="showPenaltyChannelModal"
        @ok="savePenaltyChannel"
        max-width="400px"
        close-button
        ok-text="保存"
        cancel-text="取消"
    >
        <VaInput
            v-model="penaltyChannel"
            label="惩罚频道"
            placeholder="更改惩罚频道"
            class="w-full"
        />
    </VaModal>
</template>

<style scoped></style>
