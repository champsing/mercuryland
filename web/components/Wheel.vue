<script setup lang="ts">
import { Wheel } from "spin-wheel";
import { ref, onMounted, reactive } from "vue";
import {
    VaTextarea,
    VaButton,
    VaModal,
    VaSwitch,
    VaInput,
    useToast,
} from "vuestic-ui";
import axios from "axios";
import { BASE_URL } from "@/composables/utils";

const OREKI_WHEEL_PASSWORD = ref("422613");
const wheelContainer = ref(null);
const isSpinning = ref(false); //轉盤旋轉中
const isLeftAreaLocked = ref(false); //鎖定待抽區
const clearRightArea = ref(true); //清除右邊區域
const re = /x[1-9][0-9]*$/;

let wheelConnect = reactive({
    id: -1,
    secret: "",
});
axios.get(BASE_URL + "/api/wheel/create").then((response) => {
    wheelConnect.id = response.data.id;
    wheelConnect.secret = response.data.secret;

    console.log(wheelConnect);
});

let wheel: Wheel = null;

const textArea = defineModel("textArea", {
    type: String,
    default: "",
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

        //若左邊數量不為0，則可以旋轉
        //prettier-ignore
        count(textArea.value) !== 0 ? isSpinning.value = false : isSpinning.value = true;

        return value;
    },
});

const textArea2 = defineModel("textArea2", {
    type: String,
    default: "",
    set(value: string) {
        let content = value.split("\n").filter((x) => x != "");
        axios.post(BASE_URL + "/api/wheel/update", {
            id: wheelConnect.id,
            secret: wheelConnect.secret,
            content: content,
        });
        return value;
    },
});

function submit(hide?: CallableFunction) {
    if (modal3.password == OREKI_WHEEL_PASSWORD.value) {
        axios
            .post(BASE_URL + "/api/wheel/submit", {
                id: wheelConnect.id,
                secret: wheelConnect.secret,
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
            });
    } else {
        modal3.password = "";
        modal3.fail = true;
    }
}

function beforeCancel(hide: CallableFunction) {
    modal3.password = "";
    modal3.fail = false;
    modal3.show = false;
    hide();
}

function checkLeftTextAreaNull() {
    if (count(textArea.value) == 0) {
        isSpinning.value = true; //若空轉，當旋轉結束會變無限響鈴火警。
        clearRightArea.value = true; //此時左邊一定為空，強制只能清除右邊。
        isLeftAreaLocked.value = false; //此時左邊一定為空，可以輸入。
        return true;
    }
}

function spin() {
    if (checkLeftTextAreaNull()) return;
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
        .map((x: { label: string }) => x.label)
        .join("\n");

    checkLeftTextAreaNull();
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
        items: [],
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
            <div class="text-lime-400 font-bold text-4xl text-right">
                {{
                    wheelConnect.id == -1
                        ? "----"
                        : wheelConnect.id
                              .toString(16)
                              .padStart(4, "0")
                              .toUpperCase()
                }}
            </div>
        </div>

        <div class="flex w-full justify-evenly">
            <div class="wheel-wrapper w-2/5 -mt-20" ref="wheelContainer"></div>

            <div class="w-1/5">
                <div class="va-h4">待抽区 ({{ count(textArea) }}个)</div>
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
                    旋转
                </VaButton>
                <VaButton
                    class="w-full mt-8"
                    @click="modal3.show = true"
                    :disabled="isSpinning || count(textArea2) == 0"
                >
                    完成抽選
                </VaButton>
                <div class="h-44"></div>
            </div>
            <div class="w-1/5">
                <div class="va-h4">抽中区 ({{ count(textArea2) }}个)</div>
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
                    checkLeftTextAreaNull();
                }
            "
        >
            您确定要清空
            <div v-if="clearRightArea" class="inline text-2xl">抽中區</div>
            <div v-else class="inline text-2xl">待抽區</div>
            吗?
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
