<script setup lang="ts">
import api from "@composables/axios";
import {
    computed,
    ComputedRef,
    onBeforeUnmount,
    onMounted,
    reactive,
    ref,
} from "vue";
import {
    useToast,
    VaButton,
    VaIcon,
    VaModal,
    VaPopover,
    VaSwitch,
    VaTextarea,
} from "vuestic-ui";
import Spinner from "./Spinner.vue";

import { useAuthState } from "@/composables/authState";
import {
    ArrowClockwise24Filled,
    ArrowSyncCheckmark24Filled,
    PersonLock20Filled,
    PresenceBlocked12Regular,
} from "@vicons/fluent";
import { AlertCircleOutline } from "@vicons/ionicons5";

document.title = "幸運轉盤 - 水星人的夢幻樂園";

interface WheelItem {
    label: string;
    weight: number;
}

const authState = useAuthState();

const re = /x[1-9][0-9]*$/;
const wheelRef = ref<any>(null);
const items = ref<WheelItem[]>([{ label: "example", weight: 1 }]);
const isSpinning = ref(false); //轉盤旋轉中
const isLeftAreaLocked = ref(false); //鎖定待抽區
const clearRightArea = ref(true); //清除右邊區域
const currentWinnerIndex = ref<number | null>(null);
const APIstatus = ref<boolean | null>(null);

async function getAPIStatus() {
    const status = await api
        .get("/api/ping")
        .then((response) => {
            return response.data.status;
        })
        .catch((e) => {
            console.log(e);
            return "down";
        });
    if (status === "operational") APIstatus.value = true;
    else APIstatus.value = false;
}

function parseWheelItems(value: string): WheelItem[] {
    return value
        .split("\n")
        .filter((x) => x.trim() !== "")
        .map((line) => {
            const weight = parseInt((line.match(re) || ["x1"])[0].substring(1));
            const label = line.replace(re, "").trim();
            return {
                label: label,
                weight: isNaN(weight) || weight <= 0 ? 1 : weight,
            };
        });
}

function updateWheelItemsFromText(value: string) {
    items.value = parseWheelItems(value);
    currentWinnerIndex.value = null;
}

const textArea = defineModel("textArea", {
    type: String,
    default: sessionStorage.getItem("textArea") || "",
    set(value: string) {
        updateWheelItemsFromText(value);
        sessionStorage.setItem("textArea", value);
        return value;
    },
});

updateWheelItemsFromText(textArea.value);

const textArea2 = defineModel("textArea2", {
    type: String,
    default: sessionStorage.getItem("textArea2") || "",
    set(value: string) {
        sessionStorage.setItem("textArea2", value);
        return value;
    },
});

// 新增一個介面來處理待送出的懲罰項目
interface PenaltySubmissionItem {
    label: string;
    isActive: boolean;
}

const penaltyItems = ref<PenaltySubmissionItem[]>([]);

// 處理開啟狀態設定 Modal 的邏輯
function openStatusModal() {
    if (count(textArea2.value) === 0) return;

    // 將 textArea2 的文字轉為物件陣列，預設為 true (1)
    penaltyItems.value = textArea2.value
        .split("\n")
        .filter((x) => x.trim() !== "")
        .map((line) => ({
            label: line,
            isActive: true, // 預設勾選，代表已生效 (1)
        }));

    modal4.show = true;
}

// 修改後的提交邏輯
function submit(hide?: CallableFunction) {
    const token = localStorage.getItem("token");
    if (!token) {
        window.alert("請先登入管理員帳號");
        modal3.fail = true;
        return;
    }

    // 根據 penaltyItems 的勾選狀態生成 payload
    // 勾選為 1 (已生效，待完成), 未勾選為 0 (未生效)
    const penaltiesPayload = penaltyItems.value.map((item) => {
        return [item.label, item.isActive ? 1 : 0];
    });

    api.post("/api/wheel/submit", {
        token: token,
        penalties: penaltiesPayload,
    })
        .then(() => {
            useToast().init({
                duration: 2000,
                message: "已廣播至Discord並記錄狀態",
            });
            if (hide) hide();
            modal4.show = false; // 關閉狀態視窗
        })
        .catch((error) => {
            console.error(error);
            useToast().init({
                duration: 2000,
                message: "廣播失敗，請重新登入或再試一次",
                color: "danger",
            });
            modal3.fail = true;
        });
}

function beforeCancel(hide: CallableFunction) {
    modal3.fail = false;
    modal3.show = false;
    hide();
}

function initializeWheel() {
    isSpinning.value = false;
    clearRightArea.value = true; //此時左邊一定為空，強制只能清除右邊。
    isLeftAreaLocked.value = false; //此時左邊一定為空，可以輸入。
}

function spin() {
    if (count(textArea.value) === 0 || wheelRef.value === null) return; //若空轉

    isSpinning.value = true; //旋轉、清空開關
    isLeftAreaLocked.value = true; //鎖住待抽區
    clearRightArea.value = true; //強制只能清除右邊
    wheelRef.value.spin();
}

function handleWinner(winnerId: number) {
    var audio = new Audio("/sounds/rest.mp3");
    audio.play();
    const winner = items.value[winnerId];
    modal.text = winner ? winner.label : "";
    modal.show = true;
    isLeftAreaLocked.value = false; //解鎖待抽區
    isSpinning.value = false; //解鎖旋轉、清空開關
    currentWinnerIndex.value = winnerId;
}

function move() {
    if (currentWinnerIndex.value === null) return;
    // copy text to new area
    if (textArea2.value == "") textArea2.value += modal.text;
    else {
        textArea2.value += "\n";
        textArea2.value += modal.text;
    }

    // delete text in old area
    textArea.value = items.value
        .filter((_, index) => index !== currentWinnerIndex.value)
        .map((x: { label: string; weight: number }) =>
            x.weight !== 1 ? `${x.label}x${x.weight}` : x.label,
        )
        .join("\n");

    if (count(textArea.value) == 0) initializeWheel();
    currentWinnerIndex.value = null;
}

function count(text: string): number {
    return text.split("\n").filter((x) => x !== "").length;
}

onMounted(() => {
    getAPIStatus();
    if (wheelRef.value) {
        wheelRef.value.drawWheel();
    }
});

onBeforeUnmount(() => {
    // Cleanup handled by Spinner component
});

const modal = reactive({
    show: false,
    text: "",
});
const modal2 = reactive({
    show: false,
});
const modal3 = reactive({
    show: false,
    fail: false,
});

// 新增 Modal 控制變數
const modal4 = reactive({
    show: false,
});

const isSubmitAvailable: ComputedRef<boolean> = computed(() => {
    return (
        !isSpinning.value &&
        count(textArea2.value) != 0 &&
        APIstatus.value &&
        authState.isAuthenticated
    );
});
</script>

<template>
    <main
        class="wheel-page min-h-[calc(100vh-48px)] text-[#f7f7f8] pt-10 pb-5 px-4 max-md:pt-16 max-md:pb-4 max-md:px-3"
    >
        <div class="w-full max-w-[1400px] mx-auto">
            <!-- Page Header -->
            <div class="mb-8 px-4">
                <h1 class="text-3xl font-bold text-neutral-100">幸運轉盤</h1>
                <div
                    class="mt-1.5 flex flex-wrap items-center gap-x-4 gap-y-1.5"
                >
                    <p class="text-sm text-zinc-400">
                        每行一個項目，可使用「項目名稱x權重」設定比重；旋轉後隨機抽出結果
                    </p>
                    <!-- API status hashtag -->
                    <span
                        class="inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-xs font-semibold transition-colors duration-300"
                        :class="
                            APIstatus === true
                                ? 'bg-lime-400/10 text-lime-400'
                                : APIstatus === null
                                  ? 'bg-gray-400/10 text-gray-400'
                                  : 'bg-red-400/10 text-red-400'
                        "
                    >
                        <VaIcon size="small">
                            <ArrowSyncCheckmark24Filled v-if="APIstatus" />
                            <ArrowClockwise24Filled
                                v-else-if="APIstatus == null"
                            />
                            <AlertCircleOutline v-else />
                        </VaIcon>
                        {{
                            APIstatus === true
                                ? "已連接到伺服器"
                                : APIstatus === null
                                  ? "正在連接伺服器..."
                                  : "無法連接到伺服器"
                        }}
                    </span>
                    <!-- Auth status hashtag -->
                    <span
                        class="inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-xs font-semibold transition-colors duration-300"
                        :class="
                            authState.isAuthenticated
                                ? 'bg-lime-400/10 text-lime-400'
                                : 'bg-red-400/10 text-red-400'
                        "
                    >
                        <VaIcon size="small">
                            <PersonLock20Filled
                                v-if="authState.isAuthenticated"
                            />
                            <PresenceBlocked12Regular v-else />
                        </VaIcon>
                        {{
                            authState.isAuthenticated
                                ? "已登入管理權限"
                                : "尚未登入管理權限"
                        }}
                    </span>
                </div>
            </div>

            <!-- Three-Panel Layout -->
            <div
                class="flex flex-col lg:flex-row items-start gap-6 px-4 max-md:px-0"
            >
                <!-- Wheel Panel -->
                <div class="w-full lg:w-[44%]">
                    <div
                        class="wheel-panel rounded-2xl border border-[rgba(255,255,255,0.06)] bg-[rgba(18,21,27,0.92)] backdrop-blur-sm p-6 lg:p-8"
                    >
                        <Spinner
                            ref="wheelRef"
                            :items="items"
                            @winner="handleWinner"
                        />
                    </div>
                </div>

                <!-- Left Panel: 待抽區 -->
                <div class="w-full lg:w-[27%]">
                    <div
                        class="panel-card rounded-2xl border border-[rgba(255,255,255,0.08)] bg-[rgba(18,21,27,0.92)] backdrop-blur-sm p-5 flex flex-col"
                    >
                        <!-- Panel Header -->
                        <div class="flex items-center justify-between mb-4">
                            <div>
                                <span
                                    class="text-[0.72rem] font-extrabold uppercase tracking-normal text-[#45d483]"
                                    >Draw Pool</span
                                >
                                <h2
                                    class="mt-[0.1rem] text-base font-extrabold text-[#f7f7f8]"
                                >
                                    待抽區
                                </h2>
                            </div>
                            <span
                                class="rounded-full bg-[#45d483]/15 px-3 py-1 text-xs font-bold text-[#45d483]"
                            >
                                {{ count(textArea) }} 個
                            </span>
                        </div>

                        <!-- Textarea -->
                        <VaTextarea
                            v-model="textArea"
                            :resize="false"
                            class="w-full h-80"
                            :class="{
                                'pointer-events-none opacity-50':
                                    isLeftAreaLocked,
                            }"
                            :readonly="isLeftAreaLocked"
                            placeholder="每行一個項目&#10;可加權重：項目名稱x3"
                        />

                        <!-- Action Buttons -->
                        <div class="flex flex-col gap-3 mt-4">
                            <VaButton
                                class="w-full rounded-xl !normal-case font-bold"
                                size="large"
                                @click="spin"
                                :disabled="isSpinning || count(textArea) == 0"
                            >
                                🎰 旋轉
                            </VaButton>

                            <VaButton
                                class="w-full rounded-xl !normal-case font-bold"
                                size="large"
                                color="success"
                                @click="openStatusModal"
                                :disabled="!isSubmitAvailable"
                            >
                                📤 完成抽選
                            </VaButton>

                            <!-- Submit conditions checklist -->
                            <div
                                v-if="!isSubmitAvailable"
                                class="grid grid-cols-2 gap-1.5 px-0.5 text-xs"
                            >
                                <div
                                    class="flex items-center gap-1 rounded-md px-1.5 py-1"
                                    :class="
                                        isSpinning
                                            ? 'text-zinc-500 bg-zinc-500/5'
                                            : 'text-lime-400 bg-lime-400/5'
                                    "
                                >
                                    <span class="text-sm">⏳</span>
                                    <span>{{
                                        isSpinning
                                            ? "轉盤旋轉中"
                                            : "轉盤未在旋轉"
                                    }}</span>
                                </div>
                                <div
                                    class="flex items-center gap-1 rounded-md px-1.5 py-1"
                                    :class="
                                        count(textArea2) === 0
                                            ? 'text-zinc-500 bg-zinc-500/5'
                                            : 'text-lime-400 bg-lime-400/5'
                                    "
                                >
                                    <span class="text-sm">📭</span>
                                    <span>{{
                                        count(textArea2) === 0
                                            ? "抽中區無項目"
                                            : "抽中區有項目"
                                    }}</span>
                                </div>
                                <div
                                    class="flex items-center gap-1 rounded-md px-1.5 py-1"
                                    :class="
                                        !APIstatus
                                            ? 'text-zinc-500 bg-zinc-500/5'
                                            : 'text-lime-400 bg-lime-400/5'
                                    "
                                >
                                    <span class="text-sm">🔌</span>
                                    <span>{{
                                        APIstatus === true
                                            ? "已連接到伺服器"
                                            : APIstatus === null
                                              ? "正在連接伺服器..."
                                              : "無法連接到伺服器"
                                    }}</span>
                                </div>
                                <div
                                    class="flex items-center gap-1 rounded-md px-1.5 py-1"
                                    :class="
                                        !authState.isAuthenticated
                                            ? 'text-zinc-500 bg-zinc-500/5'
                                            : 'text-lime-400 bg-lime-400/5'
                                    "
                                >
                                    <span class="text-sm">🔒</span>
                                    <span>{{
                                        authState.isAuthenticated
                                            ? "已登入管理權限"
                                            : "尚未登入管理權限"
                                    }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Right Panel: 抽中區 -->
                <div class="w-full lg:w-[27%]">
                    <div
                        class="panel-card rounded-2xl border border-[rgba(255,255,255,0.08)] bg-[rgba(18,21,27,0.92)] backdrop-blur-sm p-5 flex flex-col"
                    >
                        <!-- Panel Header -->
                        <div class="flex items-center justify-between mb-4">
                            <div>
                                <span
                                    class="text-[0.72rem] font-extrabold uppercase tracking-normal text-amber-400"
                                    >Results</span
                                >
                                <h2
                                    class="mt-[0.1rem] text-base font-extrabold text-[#f7f7f8]"
                                >
                                    抽中區
                                </h2>
                            </div>
                            <span
                                class="rounded-full bg-amber-400/15 px-3 py-1 text-xs font-bold text-amber-400"
                            >
                                {{ count(textArea2) }} 個
                            </span>
                        </div>

                        <!-- Textarea -->
                        <VaTextarea
                            v-model="textArea2"
                            :resize="false"
                            class="w-full h-80"
                            placeholder="抽中的項目會移動到這裡"
                        />

                        <!-- Clear Row -->
                        <div class="flex flex-col gap-3 mt-4">
                            <VaButton
                                class="w-full rounded-xl !normal-case font-bold"
                                size="large"
                                color="danger"
                                @click="modal2.show = true"
                            >
                                清空
                            </VaButton>
                            <!-- Tab switcher for clear target -->
                            <div
                                class="flex w-full rounded-xl overflow-hidden border border-[rgba(255,255,255,0.08)]"
                                :class="{
                                    'pointer-events-none opacity-50':
                                        isSpinning,
                                }"
                            >
                                <button
                                    class="flex-1 py-2.5 text-sm font-bold transition-colors duration-200"
                                    :class="
                                        !clearRightArea
                                            ? 'bg-[#1ccba2] text-white'
                                            : 'bg-transparent text-zinc-500 hover:text-zinc-300'
                                    "
                                    :disabled="isSpinning"
                                    @click="clearRightArea = false"
                                >
                                    待抽區
                                </button>
                                <button
                                    class="flex-1 py-2.5 text-sm font-bold transition-colors duration-200"
                                    :class="
                                        clearRightArea
                                            ? 'bg-[#3444a2] text-white'
                                            : 'bg-transparent text-zinc-500 hover:text-zinc-300'
                                    "
                                    :disabled="isSpinning"
                                    @click="clearRightArea = true"
                                >
                                    抽中區
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Modals -->
        <VaModal
            v-model="modal.show"
            noDismiss
            @ok="move"
            ok-text="移动"
            cancel-text="取消"
            :mobile-fullscreen="false"
        >
            <div class="text-3xl">
                {{ modal.text }}
            </div>
        </VaModal>
        <VaModal
            v-model="modal2.show"
            noDismiss
            :mobile-fullscreen="false"
            @ok="
                () => {
                    clearRightArea == true ? (textArea2 = '') : (textArea = '');
                    initializeWheel();
                }
            "
        >
            您確定要清空
            <div v-if="clearRightArea" class="inline text-2xl">抽中區</div>
            <div v-else class="inline text-2xl">待抽區</div>
            嗎？
            <!-- prettier-ignore -->
            <div v-if="clearRightArea" class="text-4xl text-right text-[#1aedab]">清→</div>
            <div v-else class="text-4xl text-right text-[#bae64c]">←清</div>
        </VaModal>
        <VaModal
            v-model="modal3.show"
            ok-text="廣播到Discord"
            cancel-text="取消"
            :before-ok="submit"
            :before-cancel="beforeCancel"
            :mobile-fullscreen="false"
        >
            <div class="flex flex-col gap-2 items-center text-center">
                <div class="text-xl">
                    您確定要送出抽選結果並廣播到Discord嗎？
                </div>
                <div class="text-lg">
                    抽選結果會自動加入到「直播懲罰」頁面。
                </div>
            </div>
            <div class="text-red-500 mt-4" v-if="modal3.fail">
                廣播失敗，請重新登入或再試一次。
            </div>
        </VaModal>
        <VaModal
            v-model="modal4.show"
            title="確認懲罰生效狀態"
            ok-text="設定完成"
            cancel-text="取消"
            :before-ok="() => (modal3.show = true)"
            :mobile-fullscreen="false"
        >
            <div class="mb-2 text-bg">請選擇懲罰送出時的完成狀態</div>
            <div class="mb-4 text-gray-400 text-sm">
                勾選代表「未完成」，未勾選代表「未生效」。
            </div>

            <div class="max-h-96 overflow-y-auto overflow-x-hidden pr-2">
                <div
                    v-for="(item, index) in penaltyItems"
                    :key="index"
                    class="flex flex-row items-center justify-between p-2 border-b border-gray-700 last:border-0"
                >
                    <div>
                        <VaPopover v-if="item.label.length > 30">
                            <p class="text-lg truncate max-w-lg">
                                {{ item.label }}
                            </p>
                            <template #body>
                                <p class="text-lg text-wrap max-w-lg">
                                    {{ item.label }}
                                </p>
                            </template>
                        </VaPopover>
                        <span v-else class="text-lg max-w-lg">
                            {{ item.label }}
                        </span>
                    </div>
                    <div>
                        <VaSwitch
                            v-model="item.isActive"
                            size="small"
                            true-inner-label="未完成"
                            false-inner-label="未生效"
                            color="danger"
                            off-color="textPrimary"
                        />
                    </div>
                </div>
            </div>

            <div class="text-red-500 mt-4" v-if="modal3.fail">
                廣播失敗，請檢查權限或伺服器狀態。
            </div>
        </VaModal>
    </main>
</template>

<style>
.wheel-page {
    background:
        radial-gradient(
            circle at 50% 0%,
            rgba(69, 212, 131, 0.06),
            transparent 36rem
        ),
        radial-gradient(
            circle at 15% 0%,
            rgba(88, 166, 255, 0.2),
            transparent 32rem
        ),
        linear-gradient(135deg, #0f1117 0%, #16191f 48%, #101318 100%);
}

.pointer-img {
    width: clamp(8rem, 18vw, 10rem);
    height: auto;
    margin-top: clamp(12px, 2vw, 28px);
}
</style>
