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

const showPenaltyChannelModal = ref(false);
const showCoinChannelModal = ref(false);
const showVoteConfigModal = ref(false);
const penaltyChannel = ref("");
const coinChannel = ref("");
const voteChannel = ref("");
const voteMessage = ref("");

async function openPenaltyChannelModal() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        const response = await axios.get(
            `${BASE_URL}/api/setting/config?token=${token}&id=0`,
        );
        penaltyChannel.value = response.data.value;
        showPenaltyChannelModal.value = true;
    } catch (error) {
        console.error("Failed to fetch wheel password", error);
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
            id: 0,
            value: penaltyChannel.value,
        });
        showPenaltyChannelModal.value = false;
    } catch (error) {
        console.error("Failed to save wheel password", error);
    }
}

async function openCoinChannelModal() {
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

async function saveCoinChannel() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        await axios.post(`${BASE_URL}/api/setting/config`, {
            token,
            id: 1,
            value: coinChannel.value,
        });
        showCoinChannelModal.value = false;
    } catch (error) {
        console.error("Failed to save penalty channel", error);
    }
}

async function openVoteConfigModal() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        const voteChannelResponse = await axios.get(
            `${BASE_URL}/api/setting/config?token=${token}&id=2`,
        );
        voteChannel.value = voteChannelResponse.data.value;
        const voteMessageResponse = await axios.get(
            `${BASE_URL}/api/setting/config?token=${token}&id=3`,
        );
        voteMessage.value = voteMessageResponse.data.value;
        showVoteConfigModal.value = true;
    } catch (error) {
        console.error("Failed to fetch vote channel or msg", error);
    }
}

async function saveVoteConfig() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("No token found");
        return;
    }

    try {
        await axios.post(`${BASE_URL}/api/setting/config`, {
            token,
            id: 2,
            value: voteChannel.value,
        });
        await axios.post(`${BASE_URL}/api/setting/config`, {
            token,
            id: 3,
            value: voteMessage.value,
        });
        showVoteConfigModal.value = false;
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
            頻道與訊息
        </VaCardTitle>
        <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
            <div class="grid grid-cols-2 grid-rows-2 gap-4">
                <VaButton
                    preset="primary"
                    color="info"
                    class="flex-1"
                    @click="openPenaltyChannelModal"
                >
                    惩罚频道
                </VaButton>
                <VaButton
                    preset="primary"
                    color="info"
                    class="flex-1"
                    @click="openCoinChannelModal"
                >
                    交易所频道
                </VaButton>
                <VaButton
                    preset="primary"
                    color="info"
                    class="flex-1"
                    @click="openVoteConfigModal"
                >
                    投票频道與訊息
                </VaButton>
            </div>
        </VaCardContent>
    </VaCard>

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

    <VaModal
        v-model="showCoinChannelModal"
        @ok="saveCoinChannel"
        max-width="400px"
        close-button
        ok-text="保存"
        cancel-text="取消"
    >
        <VaInput
            v-model="coinChannel"
            label="交易所頻道"
            placeholder="更改交易所頻道"
            class="w-full"
        />
    </VaModal>

    <VaModal
        v-model="showVoteConfigModal"
        @ok="saveVoteConfig"
        max-width="400px"
        close-button
        ok-text="保存"
        cancel-text="取消"
    >
        <div class="flex flex-col gap-2">
            <VaInput
                v-model="voteChannel"
                label="投票頻道"
                placeholder="更改投票頻道"
                class="w-full"
            />
            <VaInput
                v-model="voteMessage"
                label="投票訊息"
                placeholder="更改投票訊息"
                class="w-full"
            />
        </div>
    </VaModal>
</template>

<style scoped></style>
