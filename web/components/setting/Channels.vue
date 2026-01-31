<script setup lang="ts">
import { ref } from "vue";
// 這裡改為 import 你配置了攔截器的 axios 實例
import api from "@composables/axios";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaCardTitle,
    VaInput,
    VaModal,
} from "vuestic-ui";

const showPenaltyChannelModal = ref(false);
const showCoinChannelModal = ref(false);
const showVoteConfigModal = ref(false);

const penaltyChannel = ref("");
const coinChannel = ref("");
const voteChannel = ref("");
const voteMessage = ref("");

// --- 懲罰頻道 ---
async function openPenaltyChannelModal() {
    try {
        const { data } = await api.get("/api/setting/config?id=0");
        penaltyChannel.value = data.value;
        showPenaltyChannelModal.value = true;
    } catch (error) {
        console.error("Failed to fetch penalty channel", error);
    }
}

async function savePenaltyChannel() {
    try {
        await api.post("/api/setting/config", {
            id: 0,
            value: penaltyChannel.value,
        });
        showPenaltyChannelModal.value = false;
    } catch (error) {
        console.error("Failed to save penalty channel", error);
    }
}

// --- 交易所頻道 ---
async function openCoinChannelModal() {
    try {
        const { data } = await api.get("/api/setting/config?id=1");
        coinChannel.value = data.value; // 修正：原代碼誤寫為 penaltyChannel
        showCoinChannelModal.value = true;
    } catch (error) {
        console.error("Failed to fetch coin channel", error);
    }
}

async function saveCoinChannel() {
    try {
        await api.post("/api/setting/config", {
            id: 1,
            value: coinChannel.value,
        });
        showCoinChannelModal.value = false;
    } catch (error) {
        console.error("Failed to save coin channel", error);
    }
}

// --- 投票配置 ---
async function openVoteConfigModal() {
    try {
        // 使用 Promise.all 同時發送請求，效率更高
        const [resChannel, resMsg] = await Promise.all([
            api.get("/api/setting/config?id=2"),
            api.get("/api/setting/config?id=3"),
        ]);
        voteChannel.value = resChannel.data.value;
        voteMessage.value = resMsg.data.value;
        showVoteConfigModal.value = true;
    } catch (error) {
        console.error("Failed to fetch vote config", error);
    }
}

async function saveVoteConfig() {
    try {
        await Promise.all([
            api.post("/api/setting/config", {
                id: 2,
                value: voteChannel.value,
            }),
            api.post("/api/setting/config", {
                id: 3,
                value: voteMessage.value,
            }),
        ]);
        showVoteConfigModal.value = false;
    } catch (error) {
        console.error("Failed to save vote config", error);
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
            <div class="grid grid-cols-2 gap-4">
                <VaButton
                    preset="primary"
                    color="info"
                    @click="openPenaltyChannelModal"
                >
                    惩罚频道
                </VaButton>
                <VaButton
                    preset="primary"
                    color="info"
                    @click="openCoinChannelModal"
                >
                    交易所频道
                </VaButton>
                <VaButton
                    preset="primary"
                    color="info"
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
        ok-text="保存"
        cancel-text="取消"
    >
        <VaInput v-model="penaltyChannel" label="惩罚频道" class="w-full" />
    </VaModal>

    <VaModal
        v-model="showCoinChannelModal"
        @ok="saveCoinChannel"
        ok-text="保存"
        cancel-text="取消"
    >
        <VaInput v-model="coinChannel" label="交易所頻道" class="w-full" />
    </VaModal>

    <VaModal
        v-model="showVoteConfigModal"
        @ok="saveVoteConfig"
        ok-text="保存"
        cancel-text="取消"
    >
        <div class="flex flex-col gap-2">
            <VaInput v-model="voteChannel" label="投票頻道" class="w-full" />
            <VaInput v-model="voteMessage" label="投票訊息" class="w-full" />
        </div>
    </VaModal>
</template>
