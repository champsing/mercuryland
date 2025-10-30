<script setup lang="ts">
// TODO: Update Wheel style
import Spinner from "./Spinner.vue";
import { ref, onMounted, reactive, onBeforeUnmount } from "vue";
import {
    VaTextarea,
    VaButton,
    VaModal,
    VaSwitch,
    VaInput,
    VaIcon,
    VaDivider,
    useToast,
} from "vuestic-ui";
import axios from "axios";
import { BASE_URL } from "@/composables/utils";
import { AlertCircleOutline } from "@vicons/ionicons5";
import {
    ArrowClockwise24Filled,
    ArrowSyncCheckmark24Filled,
} from "@vicons/fluent";

document.title = "幸運轉盤 - 水星人的夢幻樂園";

interface WheelItem {
    label: string;
    weight: number;
}

const re = /x[1-9][0-9]*$/;
const wheelRef = ref<any>(null);
const items = ref<WheelItem[]>([{ label: "example", weight: 1 }]);
const isSpinning = ref(false); //轉盤旋轉中
const isLeftAreaLocked = ref(false); //鎖定待抽區
const clearRightArea = ref(true); //清除右邊區域
const currentWinnerIndex = ref<number | null>(null);
const APIstatus = ref<boolean | null>(null);

async function getAPIStatus() {
    const status = await axios
        .get(BASE_URL + "/api/ping")
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

function submit(hide?: CallableFunction) {
    axios
        .post(BASE_URL + "/api/wheel/submit", {
            password: modal3.password,
            penalties: textArea.value.split("\n").filter((x) => x != ""),
        })
        .then((response) => {
            console.log(response);
            useToast().init({
                duration: 2000,
                message: "已廣播至Discord",
            });
            hide();
        })
        .catch((error) => {
            console.log(error);
            useToast().init({
                duration: 2000,
                message: "廣播失敗",
                color: "danger",
            });
            modal3.password = "";
            modal3.fail = true;
        });
}

function beforeCancel(hide: CallableFunction) {
    modal3.password = "";
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
    password: "",
    fail: false,
});
</script>

<template>
    <div class="mt-8 m-auto w-11/12">
        <div class="flex w-full justify-evenly">
            <div class="wheel-wrapper w-2/5 mt-4">
                <Spinner ref="wheelRef" :items="items" @winner="handleWinner" />
            </div>
            <div class="w-1/5">
                <div class="va-h4">待抽區 ({{ count(textArea) }}個)</div>
                <VaTextarea
                    v-model="textArea"
                    color="#ffffff"
                    :resize="false"
                    class="w-full h-96 mt-8"
                    :readonly="isLeftAreaLocked"
                />
                <VaButton
                    class="w-full mt-8"
                    @click="spin"
                    :disabled="isSpinning || count(textArea) == 0"
                >
                    旋轉
                </VaButton>
                <VaButton
                    class="w-full mt-8"
                    @click="modal3.show = true"
                    :disabled="
                        isSpinning || count(textArea2) == 0 || !APIstatus
                    "
                >
                    完成抽選
                </VaButton>
                <div class="text-red-500 text-right" v-if="!APIstatus">
                    <VaDivider color="danger" orientation="right">
                        停用
                    </VaDivider>
                </div>

                <div class="h-44"></div>
            </div>
            <div class="w-1/5">
                <div class="va-h4">抽中區 ({{ count(textArea2) }}個)</div>
                <VaTextarea
                    v-model="textArea2"
                    color="#ffffff"
                    :resize="false"
                    class="w-full h-96 mt-8"
                />
                <div class="flex w-full justify-between">
                    <VaButton class="w-3/5 mt-8" @click="modal2.show = true">
                        清空
                    </VaButton>
                    <VaSwitch
                        class="mt-8"
                        v-model="clearRightArea"
                        off-color="#1ccba2"
                        color="#3444a2"
                        style="--va-switch-checker-background-color: #252723"
                        false-inner-label="待抽區"
                        true-inner-label="抽中區"
                        :disabled="isSpinning"
                    />
                </div>
                <div class="mt-10">
                    <div
                        class="flex flex-row gap-2 text-sm text-lime-400"
                        v-if="APIstatus"
                    >
                        <VaIcon size="large">
                            <ArrowSyncCheckmark24Filled />
                        </VaIcon>
                        已連接到伺服器
                    </div>
                    <div
                        class="flex flex-row gap-2 text-sm text-gray-400"
                        v-else-if="APIstatus == null"
                    >
                        <VaIcon size="large">
                            <ArrowClockwise24Filled />
                        </VaIcon>
                        正在連接伺服器...
                    </div>
                    <div
                        class="flex flex-row gap-2 text-sm text-red-600"
                        v-else
                    >
                        <VaIcon size="large">
                            <AlertCircleOutline />
                        </VaIcon>
                        無法連接到伺服器
                    </div>
                </div>
                <div class="h-44"></div>
            </div>
        </div>

        <VaModal
            v-model="modal.show"
            noDismiss
            @ok="move"
            ok-text="移动"
            cancel-text="取消"
        >
            <div class="text-3xl">
                {{ modal.text }}
            </div>
        </VaModal>
        <VaModal
            v-model="modal2.show"
            noDismiss
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
        >
            <div class="items-baseline text-xl text-center">
                請輸入轉盤廣播密碼
            </div>
            <div
                id="submit-wheel"
                class="flex items-baseline justify-evenly h-14 mt-4"
            >
                <VaInput
                    v-model="modal3.password"
                    label="轉盤廣播密碼"
                    type="password"
                    name="password"
                    immediate-validation
                    error-messages="Submission failed"
                    :error="modal3.fail"
                    @input="modal3.fail = false"
                />
            </div>
        </VaModal>
    </div>
</template>

<style scoped>
.pointer-img {
    width: clamp(8rem, 18vw, 10rem);
    height: auto;
    margin-top: clamp(12px, 2vw, 28px);
}
</style>
