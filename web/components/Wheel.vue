<script setup lang="ts">
import { Wheel } from "spin-wheel";
import { ref, onMounted, reactive } from "vue";
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
import { ArrowClockwise24Filled } from "@vicons/fluent";

document.title = "幸運轉盤 - 水星人的夢幻樂園";

const wheelContainer = ref(null);
const isSpinning = ref(false); //轉盤旋轉中
const isLeftAreaLocked = ref(false); //鎖定待抽區
const clearRightArea = ref(true); //清除右邊區域
const re = /x[1-9][0-9]*$/;

let wheelConnect = reactive({
    id: 0,
    secret: "",
});

if (sessionStorage.getItem("wheelConnect")) {
    wheelConnect = JSON.parse(sessionStorage.getItem("wheelConnect"));
} else {
    axios.get(BASE_URL + "/api/wheel/create").then(
        (response) => {
            wheelConnect.id = response.data.id;
            wheelConnect.secret = response.data.secret;
            console.log(wheelConnect);
            sessionStorage.setItem(
                "wheelConnect",
                JSON.stringify(wheelConnect)
            );
        },
        (error) => {
            wheelConnect.id = -1;
            console.log(error);
        }
    );
}

let wheel: Wheel = null;

const textArea = defineModel("textArea", {
    type: String,
    default: sessionStorage.getItem("textArea") || "",
    set(value: string) {
        if (wheel != null) {
            wheel.items = value
                .split("\n")
                .filter((x) => x != "")
                .map((x) => {
                    const weight = parseInt(
                        (x.match(re) || ["x1"])[0].substring(1)
                    );
                    const label = x.replace(re, "");
                    return {
                        label: label,
                        weight: weight,
                    };
                });
        }
        sessionStorage.setItem("textArea", value);
        return value;
    },
});

const textArea2 = defineModel("textArea2", {
    type: String,
    default: sessionStorage.getItem("textArea2") || "",
    set(value: string) {
        let content = value.split("\n").filter((x) => x != "");
        axios.post(BASE_URL + "/api/wheel/update", {
            id: wheelConnect.id,
            secret: wheelConnect.secret,
            content: content,
        });
        sessionStorage.setItem("textArea2", value);
        return value;
    },
});

function submit(hide?: CallableFunction) {
    //強制refresh textArea2 讓資料庫更新
    axios.post(BASE_URL + "/api/wheel/update", {
        id: wheelConnect.id,
        secret: wheelConnect.secret,
        content: textArea2.value.split("\n").filter((x) => x != ""),
    });

    axios
        .post(BASE_URL + "/api/wheel/submit", {
            id: wheelConnect.id,
            secret: wheelConnect.secret,
            password: modal3.password,
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
    // prettier-ignore
    if (count(textArea.value) == 0) return; //若空轉，當旋轉結束會變無限響鈴火警。
    else {
        isSpinning.value = true; //旋轉、清空開關
        isLeftAreaLocked.value = true; //鎖住待抽區
        clearRightArea.value = true; //強制只能清除右邊
        wheel.spin(1000 + Math.round(Math.random() * 1000));
    }
}

function rest() {
    var audio = new Audio("/sounds/rest.mp3");
    audio.play();
    modal.text = wheel.items[wheel.getCurrentIndex()].label;
    modal.show = true;
    isLeftAreaLocked.value = false; //解鎖待抽區
    isSpinning.value = false; //解鎖旋轉、清空開關
}

function move() {
    // copy text to new area
    if (textArea2.value == "") textArea2.value += modal.text;
    else {
        textArea2.value += "\n";
        textArea2.value += modal.text;
    }

    // delete text in old area
    textArea.value = wheel.items
        .filter(
            (_: { label: string }, i: number) => i != wheel.getCurrentIndex()
        )
        .map((x: { label: string; weight: number }) => {
            if (x.weight != 1) return x.label + "x" + x.weight;
            else return x.label;
        })
        .join("\n");

    if (count(textArea.value) == 0) initializeWheel();
}

function tick() {
    var audio = new Audio("/sounds/tick.mp3");
    audio.play();
}

function count(text: string): number {
    return text.split("\n").filter((x) => x != "").length;
}

onMounted(() => {
    const overlay = new Image();
    overlay.src = "/pointer.svg";

    const props = {
        items:
            textArea.value
                .split("\n")
                .filter((x) => x != "")
                .map((x) => {
                    const weight = parseInt(
                        (x.match(re) || ["x1"])[0].substring(1)
                    );
                    const label = x.replace(re, "");
                    return {
                        label: label,
                        weight: weight,
                    };
                }) || [],
        itemLabelRadiusMax: 0.4,
        itemBackgroundColors: [
            "#dc2626",
            "#ea580c",
            "#d97706",
            "#ca8a04",
            "#65a30d",
            "#16a34a",
            "#059669",
            "#0d9488",
            "#0891b2",
            "#0284c7",
            "#2563eb",
            "#4f46e5",
            "#7c3aed",
            "#9333ea",
            "#c026d3",
            "#db2777",
            "#e11d48",
        ],
        isInteractive: false,
        overlayImage: overlay,
        onRest: rest,
        onCurrentIndexChange: tick,
    };
    wheel = new Wheel(wheelContainer.value, props);
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
        <div class="flex w-full justify-end">
            <div class="flex-col">
                <div class="text-lime-400 font-bold text-4xl text-right">
                    {{
                        wheelConnect.id <= 0
                            ? "----"
                            : wheelConnect.id
                                  .toString(16)
                                  .padStart(4, "0")
                                  .toUpperCase()
                    }}
                </div>
                <div
                    class="flex flex-row gap-2 text-sm text-gray-400 !text-right"
                    v-if="wheelConnect.id == 0"
                >
                    <VaIcon size="large">
                        <ArrowClockwise24Filled />
                    </VaIcon>
                    正在連接伺服器...
                </div>
                <div
                    class="flex flex-row gap-2 text-sm text-red-600 !text-right"
                    v-if="wheelConnect.id == -1"
                >
                    <VaIcon size="large">
                        <AlertCircleOutline />
                    </VaIcon>
                    無法連接到伺服器
                </div>
            </div>
        </div>

        <div class="flex w-full justify-evenly">
            <div class="wheel-wrapper w-2/5 -mt-20" ref="wheelContainer"></div>

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
                        isSpinning ||
                        count(textArea2) == 0 ||
                        wheelConnect.id <= 0
                    "
                >
                    完成抽選
                </VaButton>
                <div
                    class="text-red-500 text-right"
                    v-if="wheelConnect.id <= 0"
                >
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
