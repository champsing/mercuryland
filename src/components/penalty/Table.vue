<script setup lang="ts">
import { Ref, ref, computed } from "vue";
import { NButton, NCard, NDivider, NModal, NTable } from "naive-ui";
import { VaButton } from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import penaltyStatus from "@assets/data/penalty_status.json";
import vodData from "@assets/data/vod.json";
import { openLink, openLinks, ofId } from "@/composables/utils";

const props = defineProps<{
    dateRange: [number, number];
    status?: string;
    search: string;
}>();

const emit = defineEmits<{
    (e: "updateStatus", status: string): void;
}>();

class PenaltyDataEntry {
    id: number;
    date: string;
    name: string;
    status: string;
    description?: { block: string; str?: string; uri?: string }[];
    reapply?: { times: number; entries: { date: string; status: string }[] };
}

function statusOf(status: string): (typeof penaltyStatus)[0] {
    return penaltyStatus.filter((x) => x.name == status)[0];
}

function vodLinkOfDate(date: string): string[] {
    return vodData.filter((x) => x.date == date).map((x) => x.link);
}

const showPenaltyEntryModal = ref(false);
const penaltyEntryModalContent: Ref<PenaltyDataEntry> = defineModel(
    "penaltyEntryModalContent",
    {
        default: null,
        set(value) {
            showPenaltyEntryModal.value = !showPenaltyEntryModal.value;
            return value;
        },
    }
);

const filteredData = computed(() =>
    filterPenaltyData(props.dateRange, props.status, props.search)
);

function filterPenaltyData(
    date: [number, number],
    status: string,
    search: string
): typeof penaltyData {
    return penaltyData
        .filter(
            (v) =>
                v.date >= new Date(date[0]).toISOString().slice(0, 10) &&
                v.date <=
                    new Date(date[1] + 28800000).toISOString().slice(0, 10)
        )
        .filter((v) => status == null || status == v.status)
        .filter(
            (v) =>
                search == "" ||
                v.name.toLowerCase().includes(search.toLowerCase())
        )
        .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));
}
</script>

<template>
    <n-table
        :bordered="true"
        size="small"
        class="text-center w-full"
        item-responsive
    >
        <thead>
            <tr>
                <td class="font-bold">日期</td>
                <td class="font-bold">惩罚内容</td>
                <td class="font-bold">完成状况</td>
            </tr>
        </thead>

        <tbody>
            <!-- !bg-[#b91c1c] !bg-[#4d7c0f] !bg-[#047857] !bg-[#b45309] -->
            <!-- TAILWIND CSS: DO NOT REMOVE ABOVE COMMENT -->
            <tr v-for="item in filteredData">
                <td :class="`!bg-[${statusOf(item.status).color}]`">
                    {{ item.date }}
                </td>
                <td :class="`!bg-[${statusOf(item.status).color}]`">
                    <n-button
                        @click="penaltyEntryModalContent = item"
                        :text="true"
                        :focusable="false"
                    >
                        {{ item.name }}
                    </n-button>
                </td>
                <td :class="`!bg-[${statusOf(item.status).color}]`">
                    <n-button
                        @click="() => emit('updateStatus', item.status)"
                        :text="true"
                        :focusable="false"
                    >
                        {{ item.status }}
                    </n-button>
                </td>
            </tr>
        </tbody>
    </n-table>

    <n-modal v-model:show="showPenaltyEntryModal">
        <n-card
            style="width: 600px"
            :title="penaltyEntryModalContent.name"
            :bordered="false"
            size="huge"
            role="dialog"
            aria-modal="true"
        >
            <template #header-extra>
                <n-button
                    @click="
                        openLinks(vodLinkOfDate(penaltyEntryModalContent.date))
                    "
                >
                    直播转盘連結
                </n-button>
            </template>
            <template v-for="block in penaltyEntryModalContent.description">
                <span v-if="block.block == 'text'">{{ block.str }}</span>
                <n-button
                    v-if="block.block == 'link'"
                    @click="openLink(block.uri)"
                    :text="true"
                    :focusable="false"
                >
                    {{ block.str }}
                </n-button>

                <n-button
                    v-if="block.block == 'vod'"
                    @click="openLink(ofId(vodData, parseInt(block.uri)).link)"
                    :text="true"
                    :focusable="false"
                >
                    {{ block.str }}
                </n-button>

                <VaButton
                    v-if="block.block == 'penalty'"
                    @click="() => {
                        penaltyEntryModalContent = ofId(penaltyData, parseInt(block.uri));
                        showPenaltyEntryModal = !showPenaltyEntryModal;
                    }"
                    class="mt-1"
                    color="#30e0e7"
                    size="small"
                    round
                >
                    {{ ofId(penaltyData, parseInt(block.uri)).date }}．{{ ofId(penaltyData, parseInt(block.uri)).name }}
                </VaButton>

                <img
                    v-if="block.block == 'image'"
                    :src="`penalty/${block.uri}`"
                    :alt="block.str"
                />
                <br v-if="block.block == 'br'" />
            </template>

            <template
                v-if="penaltyEntryModalContent.reapply?.times !== undefined"
            >
                <div class="mt-3">
                    <span class="text-base">
                        😇&nbsp;復活&ensp;
                        <div class="penalty-reapply text-2xl text-orange-300">
                            {{ penaltyEntryModalContent.reapply?.times }}
                        </div>
                        &ensp;次
                    </span>
                </div>
                <n-divider class="!m-1" />
            </template>

            <template
                v-for="entry in penaltyEntryModalContent.reapply?.entries"
            >
                <div class="mt-1">
                    <n-button
                        @click="openLinks(vodLinkOfDate(entry.date))"
                        :text="true"
                        :focusable="false"
                    >
                        {{ entry.date }}
                    </n-button>
                    &ensp;
                    <!-- !text-[#b91c1c] !text-[#4d7c0f] !text-[#047857] !text-[#b45309] -->
                    <div class="penalty-reapply text-sm">
                        <div
                            v-if="entry.status == '未開始'"
                            class="!text-[#b91c1c]"
                        >
                            ◼
                        </div>
                        <div
                            v-if="entry.status == '已完成'"
                            class="!text-[#4d7c0f]"
                        >
                            ◼
                        </div>
                        <div
                            v-if="entry.status == '勉強過'"
                            class="!text-[#047857]"
                        >
                            ◼
                        </div>
                        <div
                            v-if="entry.status == '進行中'"
                            class="!text-[#b45309]"
                        >
                            ◼
                        </div>
                    </div>
                    &nbsp;{{ entry.status }}
                </div>
            </template>
        </n-card>
    </n-modal>
</template>
