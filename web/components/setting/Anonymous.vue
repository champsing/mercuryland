<script setup lang="ts">
import { onMounted, ref } from "vue";
import api from "@composables/axios";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaCardTitle,
    VaDataTable,
    VaModal,
} from "vuestic-ui";

interface AnonymousEntry {
    id: number;
    author_id: number;
    nickname: string;
    avatar: string;
    updated_at: string;
}

const showModal = ref(false);
const anonymousData = ref<AnonymousEntry[]>([]);
const isLoading = ref(true);
const isLoaded = ref(false);

async function fetchAnonymousData() {
    try {
        isLoading.value = true;
        const response = await api.get(`/api/anonymous/list`);
        // Sort by ID in descending order (newest first)
        anonymousData.value = response.data.sort(
            (a: AnonymousEntry, b: AnonymousEntry) => b.id - a.id,
        );
        isLoaded.value = true;
    } catch (error) {
        console.error("Failed to fetch anonymous data", error);
    } finally {
        isLoading.value = false;
    }
}

function openModal() {
    showModal.value = true;
}

// Pre-load data when component mounts
onMounted(() => {
    fetchAnonymousData();
});

const columns = [
    {
        key: "avatar",
        label: "头像",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: false,
        width: 80,
    },
    {
        key: "nickname",
        label: "昵称",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
    },
    {
        key: "author_id",
        label: "用户",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
    },
    {
        key: "id",
        label: "消息ID",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
        width: 100,
    },
    {
        key: "updated_at",
        label: "发送时间",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
        width: 180,
    },
];
</script>

<template>
    <VaCard class="rounded-xl border border-zinc-700">
        <VaCardTitle
            class="px-6 pt-6 !text-xl font-medium text-zinc-200 justify-center"
        >
            匿名频道
        </VaCardTitle>
        <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
            <VaButton
                preset="primary"
                color="info"
                class="w-full"
                :loading="isLoading && !isLoaded"
                :disabled="!isLoaded"
                @click="openModal"
            >
                {{ isLoaded ? "消息记录" : "加载中..." }}
            </VaButton>
        </VaCardContent>
    </VaCard>

    <VaModal
        v-model="showModal"
        max-width="800px"
        close-button
        :hide-default-actions="true"
    >
        <template #header>
            <h3 class="text-lg font-semibold">匿名消息记录</h3>
        </template>
        <VaDataTable
            :items="anonymousData"
            :columns="columns"
            style="
                --va-data-table-hover-color: #357286;
                --va-data-table-thead-background: var(--va-background-element);
                --va-data-table-thead-border: 0;
            "
            hoverable
        >
            <template #cell(avatar)="{ value }">
                <img
                    v-if="value"
                    :src="value"
                    alt="Avatar"
                    class="w-8 h-8 rounded-full mx-auto"
                />
                <div
                    v-else
                    class="w-8 h-8 rounded-full bg-gray-500 mx-auto flex items-center justify-center text-xs text-white"
                >
                    ?
                </div>
            </template>
            <template #cell(updated_at)="{ value }">
                {{ new Date(value).toLocaleString("zh-CN") }}
            </template>
        </VaDataTable>
    </VaModal>
</template>

<style scoped>
:deep(.va-data-table__thead) {
    background-color: var(--va-background-element);
}
</style>
