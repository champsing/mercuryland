<script setup lang="ts">
import { VaButton, VaDivider, VaInput } from "vuestic-ui";
import { backToTop, copyToClipboard } from "@/composables/utils";
import { ref } from "vue";

backToTop();

const email = ref("0.flunky-sojourn@icloud.com");
</script>

<template>
    <div
        class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2"
    >
        <h1 class="ml-12 text-2xl font-semibold">使用條款</h1>
        <p class="text-zinc-400 sm:text-right">
            請於使用本網站服務前，詳閱並理解本使用條款之內容。
        </p>
    </div>
    <VaDivider class="w-full !mt-0 !mb-2" />

    <div class="m-auto flex w-full flex-col items-center">
        <div class="w-11/12">
            <p class="text-lg font-semibold">
                本聲明旨在規範網際網路連接視聽閱覽者（以下簡稱「閱聽人」）存取水星伺服器網（以下簡稱「本網站」）所提供之多媒體內容，包括但不限於：文字、文件、圖片、影音、壓縮檔與外部連結，所應享有之權利與應履行之義務，並規範閱聽人與本網站管理者之間的權利義務關係。
            </p>

            <h1 class="text-lg mt-4">一、總則</h1>
            <ol>
                <li>
                    本網站之管理者（以下簡稱「管理方」），若未經正常程序變更，為水星伺服器管理委員會。
                    <ol>
                        <li>
                            管理權之歸屬，若未經正常程序變更，為管理方持有。
                        </li>
                    </ol>
                </li>
                <li class="mt-2">
                    本網站所提供之資訊，包含水星伺服器之簡介、事務、公告、規範與開放下載資源，其著作權由管理方持有。
                </li>
                <li class="mt-2">
                    管理方雖有義務維持所提供資訊之最新與正確性，惟閱聽人瀏覽時應自行留意公告日期、內容及其後續更新狀況，所有資訊之引用與依據均以存取時可取得之最新資訊為準。
                    <ol>
                        <li>
                            閱聽人引用、傳閱或翻攝本網站提供之資訊、規範與標準，並與內容互動時，應遵守隱私政策及使用條款之規範，未經管理方同意，不得擅自變更其部分或全部內容。
                        </li>
                        <li>
                            本網站不保證閱聽人於本網站可獲得之資訊為百分之百正確，亦不對因資訊更新、傳輸延遲或系統故障等「不可歸責於管理方之因素」所產生之資訊落差負責。
                        </li>
                    </ol>
                </li>
                <li class="mt-2">
                    本網站所提供之開放下載檔案，其著作權由管理方持有。
                    <ol>
                        <li>
                            管理方雖有義務維持檔案之最新與正確性，惟閱聽人下載時應自行留意檔案版本及其後續更新狀況，所有資訊之引用與依據均以存取時可取得之最新資訊為準。
                        </li>
                        <li>
                            閱聽人下載本網站檔案或瀏覽本網站提供之外部連結時，應自行留意檔案安全與網路資通隱私權等，若因檔案或連結夾帶病毒而導致使用者電腦遭植入第三方木馬程式、資料遭竊取或資料完整性受損等任何損失，本網站<span
                                class="text-red-500 font-bold"
                                >概不負任何責任</span
                            >。
                        </li>
                        <li>
                            閱聽人引用、傳閱或翻攝本網站提供之開放下載資源，瀏覽本網站提供之外部連結並與其內容互動時，應遵守使用條款之規範，未經管理方同意，不得擅自變更其部分或全部內容。
                        </li>
                        <li>
                            本網站不保證閱聽人於本網站可下載之資源為百分之百正確，亦不對因資訊更新、傳輸延遲或系統故障等「不可歸責於管理方之因素」所產生之資訊落差負責。
                        </li>
                    </ol>
                </li>
                <li class="mt-2">
                    本聲明若有任何未盡之處，管理方保留隨時更新本聲明內容之權利，且無需另行通知。閱聽人應自行留意本聲明內容是否為最新版本，並應遵守所有條款之規定。
                </li>
            </ol>

            <h1 class="text-lg mt-4">二、條款之契約效力</h1>
            <p>
                閱聽人一經進入、瀏覽或使用本網站，即視為已詳閱、理解並同意本使用條款及相關政策（包含但不限於隱私政策、著作權聲明等），並與管理方成立具有法律效力之契約關係。若不同意本條款內容，請立即停止使用本網站。
            </p>

            <h1 class="text-lg mt-4">三、責任限制與合理注意義務聲明</h1>
            <p>
                管理方已盡合理努力確保本網站所提供資訊與資源之正確性、安全性與完整性，惟不保證其完全無誤或不中斷運作。對於因使用本網站、下載資源或連結第三方網站所生之任何直接、間接、附帶、特殊或衍生性損害，管理方概不負責，除非可證明係由管理方之重大過失或故意所致。
            </p>

            <h1 class="text-lg mt-4">四、個人資料與隱私政策</h1>
            <p>
                本網站可能於您瀏覽期間收集必要之技術性資訊（如
                IP位址、裝置資訊與Cookie等），僅作為網站安全與內容優化用途，管理方將依據中華民國《個人資料保護法》等相關法規妥善處理所有個人資料。如欲了解更多資訊，請參閱本網站「隱私政策」。
            </p>

            <h1 class="text-lg mt-4">五、準據法與管轄法院</h1>
            <p>
                本使用條款之成立、解釋與適用，均以中華民國法律為準據法。如因使用本網站而產生任何爭議，雙方同意以臺灣臺北地方法院為第一審專屬管轄法院。
            </p>
        </div>
        <div class="mt-4 mb-4 font-semibold">
            電子郵件信箱：
            <VaInput v-model="email" readonly @click="copyToClipboard(email)" />
            <VaButton
                href="mailto:0.flunky-sojourn@icloud.com"
                preset="secondary"
                color="info"
                border-color="info"
                class="ml-2"
            >
                送信
            </VaButton>
            <VaButton
                preset="secondary"
                color="textPrimary"
                border-color="textPrimary"
                to="contact"
                class="ml-2"
            >
                聯絡我們
            </VaButton>
        </div>
    </div>
</template>

<style scoped>
ol li {
    list-style: decimal;
    list-style-position: inside;
}

ol li ol li {
    margin-left: 24px;
    list-style: lower-alpha;
    list-style-position: inside;
}
</style>
