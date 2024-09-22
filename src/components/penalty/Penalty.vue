<script setup lang="ts">
import { ref, Ref } from "vue";
import {
    NButton,
    NCard,
    NDatePicker,
    NDivider,
    NFlex,
    NGrid,
    NGi,
    NSelect,
    NIcon,
    NInput,
    NModal,
    NTable,
} from "naive-ui";
import { VaButton, VaButtonGroup, VaChip, VaTextarea } from "vuestic-ui";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    BarElement,
    ArcElement,
    CategoryScale,
    LinearScale,
    TimeScale,
    ChartOptions,
    ChartData,
} from "chart.js";
import { Bar, Doughnut } from "vue-chartjs";
import {
    openLink,
    openLinks,
    ofId,
    copyToClipboard,
} from "@composables/utils.ts";
import penaltyData from "@assets/data/penalty.json";
import penaltyStatus from "@assets/data/penalty_status.json";
import vodData from "@assets/data/vod.json";
import PenaltySyntax from "./PenaltySyntax.vue";
import "chartjs-adapter-date-fns";
import { InfoCircle } from "@vicons/tabler";

ChartJS.register(
    Title,
    Tooltip,
    BarElement,
    ArcElement,
    CategoryScale,
    LinearScale,
    TimeScale
);

let filterDate: Ref<[number, number]> = defineModel("filterDate", {
    default: [1672502400000, Date.now() + 28800000] as const,
    set(value: [number, number]) {
        filteredData.value = filterPenaltyData(
            value,
            filterStatus.value,
            filterSearch.value
        );
        return value;
    },
});

let filterStatus = defineModel("filterStatus", {
    default: null,
    set(value) {
        filteredData.value = filterPenaltyData(
            filterDate.value,
            value,
            filterSearch.value
        );
        return value;
    },
});

let filterSearch = defineModel("filterSearch", {
    default: "",
    set(value) {
        filteredData.value = filterPenaltyData(
            filterDate.value,
            filterStatus.value,
            value
        );
        return value;
    },
});

let finishOptions = [{ label: "", value: null }].concat(
    penaltyStatus
        .map((x) => x.name)
        .sort()
        .map((x) => {
            return { label: x, value: x };
        })
);

let filteredData = defineModel("filteredData", {
    default: filterPenaltyData([0, Date.now()], null, ""),
    set(value) {
        doughnutChartData.value = generateDoughnutChartData(value);
        barChartData.value = generateBarChartData(value);
        return value;
    },
});

let doughnutChartData: Ref<any> = ref(
    generateDoughnutChartData(filteredData.value)
);
let doughnutChartOptions = {
    maintainAspectRatio: false,
    plugins: {
        legend: {
            display: false,
        },
    },
    layout: {
        padding: 20,
    },
} as ChartOptions<"doughnut">;

let barChartData: Ref<any> = ref(generateBarChartData(filteredData.value));
let barChartOptions = {
    maintainAspectRatio: false,
    plugins: {
        legend: {
            display: false,
        },
    },
    scales: {
        x: {
            type: "time",
            grid: {
                display: false,
            },
            time: {
                minUnit: "day",
            },
        },
        y: {
            grid: {
                display: false,
            },
        },
    },
} as ChartOptions<"bar">;

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
const showExistModal = ref(false);
const showCompleteModal = ref(false);

let notYetStartedPenalties = penaltyData
    .filter((x) => x.status == "未開始")
    .map((x) => x.name)
    .join("\n")
    .toString();

let completedPenalties = penaltyData
    .filter((x) => x.status == "已完成")
    .map((x) => x.name)
    .join("\n")
    .toString();

let barelyPassedPenalties = penaltyData
    .filter((x) => x.status == "勉強過")
    .map((x) => x.name)
    .join("\n")
    .toString();

let proceedingPenalties = penaltyData
    .filter((x) => x.status == "進行中")
    .map((x) => x.name)
    .join("\n")
    .toString();
</script>

<script lang="ts">
class PenaltyDataEntry {
    id: number;
    date: string;
    name: string;
    status: string;
    description?: { block: string; str?: string; uri?: string }[];
    reapply?: { times: number; entries: { date: string; status: string }[] };
}

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

function generateDoughnutChartData(
    filteredData: typeof penaltyData
): ChartData<"doughnut", number[], string> {
    return {
        labels: penaltyStatus.map((x) => x.name),
        datasets: [
            {
                label: null,
                data: penaltyStatus.map(
                    (x) => filteredData.filter((y) => x.name == y.status).length
                ),
                backgroundColor: penaltyStatus.map((x) => x.color),
                borderWidth: 0,
                hoverOffset: 50,
            },
        ],
    };
}

function generateBarChartData(
    filteredData: typeof penaltyData
): ChartData<"bar", number[], number> {
    return {
        labels: filteredData
            .map((x) => new Date(x.date).getTime())
            .filter((e, i, a) => e !== a[i - 1]), // may be wrong
        datasets: penaltyStatus.map((x) => {
            return {
                label: x.name,
                data: Array.from(Map.groupBy(filteredData, (d) => d.date))
                    .sort((lhs, rhs) => lhs[0].localeCompare(rhs[0]))
                    .map(
                        ([_, v]) => v.filter((y) => x.name == y.status).length
                    ),
                backgroundColor: x.color,
                stack: "0",
            };
        }),
    };
}

function statusOf(status: string): (typeof penaltyStatus)[0] {
    return penaltyStatus.filter((x) => x.name == status)[0];
}

function vodLinkOfDate(date: string): string[] {
    return vodData.filter((x) => x.date == date).map((x) => x.link);
}
</script>

<template>
    <n-grid x-gap="12" :cols="4" class="w-11/12">
        <n-gi :span="2">
            <n-date-picker type="daterange" v-model:value="filterDate" />
        </n-gi>
        <n-gi>
            <n-select
                v-model:value="filterStatus"
                :options="finishOptions"
                placeholder="請選擇一種完成狀態"
                :consistent-menu-width="false"
            />
        </n-gi>
        <n-gi>
            <n-input
                round
                placeholder="輸入懲罰內容來搜尋"
                v-model:value="filterSearch"
                type="text"
            />
        </n-gi>
    </n-grid>

    <n-divider class="!mt-2 !mb-2" />

    <n-grid x-gap="12" :cols="3" class="w-11/12 h-80vh overflow-y-hidden">
        <n-gi :span="2" class="h-40vh w-full p-0 m-0 overflow-y-scroll">
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
                            {{ item.status }}
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </n-gi>
        <n-gi>
            <Doughnut
                :options="doughnutChartOptions"
                :data="doughnutChartData"
                class="h-40vh w-full p-0 m-0"
            />
        </n-gi>
        <n-gi :span="3">
            <Bar
                :options="barChartOptions"
                :data="barChartData"
                class="h-40vh w-full p-0 m-0"
            />
        </n-gi>
    </n-grid>

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

    <n-flex justify="center" size="small" class="m-auto" item-responsive>
        <VaChip class="vachip2" color="#3d807c" readonly>
            <n-icon size="25" class="mt-1 mr-2">
                <InfoCircle />
            </n-icon>
            <div class="text-center text-amber-200">
                <div class="text-lg mt-1">將滑鼠移至圖表上可查看數量</div>
            </div>
        </VaChip>
        <div>
            <!--This div is for its own size, don't delete.-->
            <VaButtonGroup round class="overall-button">
                <VaButton
                    color="danger"
                    @click="showExistModal = !showExistModal"
                >
                    現存
                </VaButton>
                <VaButton
                    color="success"
                    @click="showCompleteModal = !showCompleteModal"
                >
                    完成
                </VaButton>
            </VaButtonGroup>
        </div>
    </n-flex>

    <!-- 現存 和 完成 -->
    <n-modal v-model:show="showExistModal">
        <n-card
            style="width: 600px"
            title="懲罰一覽表：現存"
            :bordered="false"
            size="huge"
            role="dialog"
            aria-modal="true"
        >
            <template #header-extra>
                <n-button
                    @click="
                        copyToClipboard(
                            notYetStartedPenalties + '\n' + proceedingPenalties
                        )
                    "
                >
                    複製所有現存懲罰
                </n-button>
            </template>
            <n-grid
                :x-gap="3"
                :y-gap="2"
                :cols="3"
                class="text-center"
                screen-responsive
            >
                <n-gi>
                    <div class="text-sm mt-1 text-[#ef3b3b]">未開始</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter((x) => x.status == "未開始")
                                .length
                        }}
                    </div>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-1 text-[#de8039]">進行中</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter((x) => x.status == "進行中")
                                .length
                        }}
                    </div>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-1 text-[#eda9a9]">現存總計</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter(
                                (x) =>
                                    x.status == "未開始" || x.status == "進行中"
                            ).length
                        }}
                    </div>
                </n-gi>
            </n-grid>
            <n-divider class="!mt-2 !mb-1" />
            <n-grid
                :x-gap="4"
                :y-gap="4"
                :cols="2"
                class="text-center mt-2"
                screen-responsive
            >
                <n-gi>
                    <div class="text-sm mt-4 mb-2">未開始</div>
                    <VaTextarea
                        v-model="notYetStartedPenalties"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                    <n-flex style="justify-content: start" class="mt-4">
                        <div class="text-sm">
                            <kbd>Ctrl</kbd
                            >&nbsp;<kbd>A</kbd>&ensp;可快速複製全部項目
                        </div>
                    </n-flex>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-4 mb-2">進行中</div>
                    <VaTextarea
                        v-model="proceedingPenalties"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                </n-gi>
            </n-grid>
        </n-card>
    </n-modal>

    <n-modal v-model:show="showCompleteModal">
        <n-card
            style="width: 600px"
            title="懲罰一覽表：完成"
            :bordered="false"
            size="huge"
            role="dialog"
            aria-modal="true"
        >
            <template #header-extra>
                <n-button
                    @click="
                        copyToClipboard(
                            completedPenalties + '\n' + barelyPassedPenalties
                        )
                    "
                >
                    複製所有完成懲罰
                </n-button>
            </template>
            <n-grid
                :x-gap="3"
                :y-gap="2"
                :cols="3"
                class="text-center"
                screen-responsive
            >
                <n-gi>
                    <div class="text-sm mt-1 text-[#4be66c]">已完成</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter((x) => x.status == "已完成")
                                .length
                        }}
                    </div>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-1 text-[#218d37]">勉強過</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter((x) => x.status == "勉強過")
                                .length
                        }}
                    </div>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-1 text-[#39e3e3]">完成總計</div>
                    <div class="text-3xl mt-1">
                        {{
                            penaltyData.filter(
                                (x) =>
                                    x.status == "已完成" || x.status == "勉強過"
                            ).length
                        }}
                    </div>
                </n-gi>
            </n-grid>
            <n-divider class="!mt-2 !mb-1" />
            <n-grid
                :x-gap="4"
                :y-gap="4"
                :cols="2"
                class="text-center"
                screen-responsive
            >
                <n-gi>
                    <div class="text-sm mt-4 mb-2">已完成</div>
                    <VaTextarea
                        v-model="completedPenalties"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                    <n-flex style="justify-content: start" class="mt-4">
                        <div class="text-sm">
                            <kbd>Ctrl</kbd
                            >&nbsp;<kbd>A</kbd>&ensp;可快速複製全部項目
                        </div>
                    </n-flex>
                </n-gi>
                <n-gi>
                    <div class="text-sm mt-4 mb-2">勉強過</div>
                    <VaTextarea
                        v-model="barelyPassedPenalties"
                        :maxRows="7"
                        :resize="false"
                        readonly
                    />
                </n-gi>
            </n-grid>
        </n-card>
    </n-modal>

    <PenaltySyntax class="mb-4" />
</template>

<style>
.vachip2 {
    margin-top: 0.5rem;
    margin-bottom: 1rem;
    --va-chip-border: 0.2rem solid transparent;
    --va-chip-font-size: 1.15rem;
    --va-chip-padding: 0 1.2rem;
}
.left-margin {
    margin-left: 25%;
}
.overall-button {
    margin-top: 0.65rem;
    --va-button-group-font-size: 1.15rem;
    --va-button-group-border-radius: 2px;
    --va-button-group-button-padding: 0.3rem;
    --va-button-group-button-width: 90px;
}

.penalty-reapply {
    display: inline-block;
    white-space: nowrap;
}

kbd {
    background-color: #e3d0d0;
    border-radius: 3px;
    border: 1px solid #b4b4b4;
    box-shadow: 0 1px 1px rgba(0, 0, 0, 0.2),
        0 2px 0 0 rgba(255, 255, 255, 0.7) inset;
    color: #0b17c3d4;
    display: inline-block;
    font-size: 0.85em;
    font-weight: 700;
    line-height: 1;
    padding: 2px 4px;
    white-space: nowrap;
}
</style>
