<script setup lang="ts">
import { Ref, ref, computed } from "vue";
import { NButton, NTable } from "naive-ui";
import { VaButton, VaDivider, VaModal } from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import penaltyStatus from "@assets/data/penalty_status.json";
import vodData from "@assets/data/vod.json";
import { openLink, openLinks, ofId } from "@/composables/utils";

const props = defineProps<{
    dateRange: { start: Date, end: Date };
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
    description?: { block: string; text?: string; uri?: string }[];
    reapply?: { entries: { date: string; status: string }[] };
    steamID?: number;
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

const showPenaltyScreenshotModal = ref(false);

const filteredData = computed(() =>
    filterPenaltyData(props.dateRange, props.status, props.search)
);

function filterPenaltyData(
    dateRange: { start: Date, end: Date },
    status: string,
    search: string
): typeof penaltyData {
    return penaltyData
        .filter(
            (v) =>
                v.date >= dateRange.start.toISOString().slice(0, 10) &&
                v.date <=
                    new Date(dateRange.end.getTime() + 28800000).toISOString().slice(0, 10)
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

    <VaModal v-model="showPenaltyEntryModal" hide-default-actions size="small">
        <!-- 本體 -->
        <div class="flex flex-row">
            <div class="text-xl flex-grow">
                {{ penaltyEntryModalContent.name }}
            </div>
            <VaButton
                color="info"
                gradient
                class="-mt-2"
                @click="openLinks(vodLinkOfDate(penaltyEntryModalContent.date))"
            >
                直播转盘連結
            </VaButton>
        </div>
        <!-- 補充說明 -->
        <div>
            <template v-for="block in penaltyEntryModalContent.description">
                <div class="mt-4">
                    <span v-if="block.block == 'text'">{{ block.text }}</span>

                    <VaButton
                        v-if="block.block == 'link'"
                        @click="openLink(block.uri)"
                        preset="plain"
                        color="textPrimary"
                    >
                        <div class="inline-block">{{ block.text }}（連結）</div>
                    </VaButton>

                    <VaButton
                        v-if="block.block == 'vod'"
                        @click="
                            openLink(ofId(vodData, parseInt(block.uri)).link)
                        "
                        color="#c82828"
                        size="small"
                        round
                    >
                        {{ ofId(vodData, parseInt(block.uri)).date }}．{{
                            ofId(vodData, parseInt(block.uri)).title
                        }}
                    </VaButton>

                    <VaButton
                        v-if="block.block == 'penalty'"
                        @click="
                            () => {
                                penaltyEntryModalContent = ofId(
                                    penaltyData,
                                    parseInt(block.uri)
                                );
                                showPenaltyEntryModal = !showPenaltyEntryModal;
                            }
                        "
                        color="#8fc1ff"
                        size="small"
                        round
                    >
                        {{ ofId(penaltyData, parseInt(block.uri)).date }}．{{
                            ofId(penaltyData, parseInt(block.uri)).name
                        }}
                    </VaButton>

                    <VaButton
                        v-if="block.block == 'image'"
                        @click="
                            showPenaltyScreenshotModal =
                                !showPenaltyScreenshotModal
                        "
                        gradient
                        color="#0e8110"
                        size="medium"
                    >
                        查看證明圖片
                    </VaButton>

                    <VaModal
                        v-model="showPenaltyScreenshotModal"
                        hide-default-actions
                    >
                        <img :src="`penalty/${block.uri}`" :alt="block.text" />
                    </VaModal>

                    <br v-if="block.block == 'br'" />
                </div>
            </template>

            <template
                v-if="penaltyEntryModalContent.steamID !== undefined"
            >
                <VaDivider class="!mt-4 !mb-2" />
                <iframe
                    :src="`https://store.steampowered.com/widget/${penaltyEntryModalContent.steamID}/`"
                    frameborder="0"
                    width="520"
                    height="150"
                />
            </template>

            <template v-if="penaltyEntryModalContent.reapply !== undefined">
                <div class="mt-3">
                    <span class="text-base">
                        😇&nbsp;復活&ensp;
                        <div class="inline text-2xl text-orange-300">
                            <!-- prettier-ignore -->
                            {{ penaltyEntryModalContent.reapply?.entries.length }}
                        </div>
                        &ensp;次
                    </span>
                </div>
                <VaDivider class="!m-1" />
            </template>

            <template
                v-for="entry in penaltyEntryModalContent.reapply?.entries"
            >
                <div class="mt-1">
                    <VaButton
                        @click="openLinks(vodLinkOfDate(entry.date))"
                        preset="plain"
                        color="textPrimary"
                    >
                        {{ entry.date }}
                    </VaButton>
                    &ensp;
                    <!-- !text-[#b91c1c] !text-[#4d7c0f] !text-[#047857] !text-[#b45309] -->
                    <div class="inline-block text-sm">
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
        </div>
    </VaModal>
</template>
