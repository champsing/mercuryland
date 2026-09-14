<script setup lang="ts">
import { VaButton, VaInput } from "vuestic-ui";
import { backToTop, copyToClipboard } from "@/composables/utils";
import { ref } from "vue";

backToTop();
const email = ref("0.flunky-sojourn@icloud.com");
</script>

<template>
    <main
        class="law-page min-h-[calc(100vh-48px)] text-[#f7f7f8] pt-10 pb-5 px-6 max-md:pt-16 max-md:pb-4 max-md:px-5"
    >
        <div class="w-full max-w-3xl mx-auto">
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-neutral-100">隱私政策</h1>
                <p class="mt-1.5 text-sm text-zinc-400">
                    請您在使用我們的服務前，仔細閱讀並了解本隱私政策的內容。
                </p>
            </div>

            <div class="law-document">
                <p>
                    本隱私政策旨在規範使用者（以下簡稱「您」）在使用水星樂園（以下簡稱「本網站」）時，本網站對您的個人資料之蒐集、處理、使用、儲存及保護等事項，並告知您相關權利。
                </p>

                <p>
                    本網站重視您的隱私權，並致力於保護您的個人資料。本隱私政策說明本網站如何蒐集、使用及處理您的個人資料。
                </p>

                <p>當您使用本網站服務時，即表示您同意本隱私政策之內容。</p>

                <h1>一、個人資料的蒐集與使用</h1>
                <ol>
                    <li>
                        蒐集的資料類型
                        <ol>
                            <li>
                                您在註冊、填寫表單或使用本網站服務時提供的資料，例如姓名、電子郵件地址、電話號碼等。
                            </li>
                            <li>您的瀏覽紀錄、IP 地址、裝置資訊等技術資料。</li>
                            <li>
                                您與本網站的互動記錄，例如客服對話、回饋意見。
                            </li>
                        </ol>
                    </li>
                    <li>
                        蒐集的使用目的
                        <ol>
                            <li>提供、維護及改進本網站服務。</li>
                            <li>回應您的查詢及提供客戶支援。</li>
                            <li>進行行銷與廣告相關活動。</li>
                            <li>符合法律法規的要求。</li>
                        </ol>
                    </li>
                </ol>

                <h1>二、個人資料的保護</h1>
                <ol>
                    <li>
                        本網站採取以下措施保護您的個人資料
                        <ol>
                            <li>使用加密技術確保資料傳輸過程中的安全。</li>
                            <li>
                                採用權限控管，限制未經授權之人員存取您的資料。
                            </li>
                            <li>定期檢視系統安全，並修補潛在的安全漏洞。</li>
                        </ol>
                    </li>
                </ol>

                <h1>三、瀏覽外部連結時的資料處理</h1>
                <ol>
                    <li>
                        本網站可能包含指向其他網站的連結。本網站無法控制這些第三方網站的隱私政策及其資料處理方式。
                    </li>
                    <li>
                        建議您在使用這些網站時，查閱其隱私政策以了解如何保護您的個人資料。
                    </li>
                </ol>

                <h1>四、個人資料的刪除與更正</h1>
                <ol>
                    <li>
                        刪除個人資料
                        <ol>
                            <li>
                                您可隨時聯繫本網站要求刪除您的個人資料，除非法律另有規定或為完成交易必要，本網站將在合理時間內處理您的請求。
                            </li>
                        </ol>
                    </li>
                    <li>
                        更正個人資料
                        <ol>
                            <li>
                                若您的個人資料有誤，您可透過本網站提供的聯絡方式要求更正，本網站將於核實後立即進行更新。
                            </li>
                        </ol>
                    </li>
                </ol>

                <h1>五、與第三方共用個人資料</h1>
                <ol>
                    <li>
                        共用情境
                        <ol>
                            <li>
                                為提供服務必要，可能與合作夥伴或服務提供商共用，例如支付處理或物流服務。
                            </li>
                            <li>
                                在法律要求下或保護本網站權益時提供給相關機構。
                            </li>
                        </ol>
                    </li>
                    <li>
                        共用原則
                        <ol>
                            <li>
                                僅共用必要的資料，並要求第三方遵守本政策或更高的隱私標準。
                            </li>
                        </ol>
                    </li>
                </ol>

                <h1>六、隱私政策的更新</h1>
                <ol>
                    <li>
                        本網站保留隨時修改本隱私政策的權利。若有重大變更，本網站將透過電子郵件或網站公告通知您。建議您定期查閱本政策，以確保瞭解最新內容。
                    </li>
                </ol>

                <h1>七、聯絡方式</h1>
                <ol>
                    <li>
                        若您對本隱私政策有任何疑問，或需行使您的個人資料權利，請使用電子郵件，或前往「聯絡我們」分頁尋找網站管理人員的Discord，本網站將竭誠為您服務。
                    </li>
                </ol>

                <div class="law-closing">
                    感謝您信任本網站，本網站將竭力保護您的個人資料及隱私。
                </div>
            </div>
            <div
                class="mt-10 mb-4 flex w-full max-w-2xl mx-auto flex-col items-center gap-3 font-semibold md:flex-row md:items-center md:justify-center md:gap-2"
            >
                <span class="text-zinc-300 whitespace-nowrap"
                    >電子郵件信箱：</span
                >
                <VaInput
                    v-model="email"
                    readonly
                    class="w-full md:w-64"
                    @click="copyToClipboard(email)"
                />
                <div class="flex flex-row gap-2">
                    <VaButton
                        href="mailto:0.flunky-sojourn@icloud.com"
                        preset="secondary"
                        color="info"
                        border-color="info"
                    >
                        送信
                    </VaButton>
                    <VaButton
                        preset="secondary"
                        color="textPrimary"
                        border-color="textPrimary"
                        to="contact"
                    >
                        聯絡我們
                    </VaButton>
                </div>
            </div>
        </div>
    </main>
</template>

<style scoped>
.law-page {
    background:
        radial-gradient(
            circle at 50% 0%,
            rgba(142, 120, 255, 0.09),
            transparent 36rem
        ),
        radial-gradient(
            circle at 15% 0%,
            rgba(88, 166, 255, 0.2),
            transparent 32rem
        ),
        linear-gradient(135deg, #0f1117 0%, #16191f 48%, #101318 100%);
}

.law-document p {
    margin: 0 0 1rem;
    font-size: 1.0625rem;
    line-height: 1.8;
    color: #e4e4e7;
}

.law-document h1 {
    margin: 2rem 0 0.75rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    font-size: 1.25rem;
    font-weight: 700;
    color: #fafafa;
    letter-spacing: 0.02em;
}

.law-document ol {
    margin: 0 0 1rem;
    padding-left: 1.5rem;
    list-style: decimal;
}

.law-document ol li {
    margin: 0.5rem 0;
    font-size: 1rem;
    line-height: 1.8;
    color: #d4d4d8;
}

.law-document ol ol {
    list-style: lower-alpha;
}

.law-document ol ol li {
    margin: 0.25rem 0;
    font-size: 0.9375rem;
    line-height: 1.7;
    color: #a1a1aa;
}

.law-document .law-closing {
    margin-top: 2rem;
    font-size: 1.0625rem;
    font-weight: 600;
    color: #e4e4e7;
}
</style>
