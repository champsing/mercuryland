<script setup lang="ts">
import { VaButton, VaDivider, VaInput } from "vuestic-ui";
import { backToTop, copyToClipboard } from "@/composables/utils";

backToTop();
const email = "0.flunky-sojourn@icloud.com";
</script>

<template>
  <div
    class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2"
  >
    <h1 class="ml-12 text-2xl font-semibold">隱私政策</h1>
    <p class="text-zinc-400 sm:text-right">
      請您在使用我們的服務前，仔細閱讀並了解本隱私政策的內容。
    </p>
  </div>
  <VaDivider class="w-full !mt-0 !mb-2" />

  <div class="m-auto flex w-full flex-col items-center">
    <div class="w-11/12">
      <p class="text-lg font-semibold">
        本隱私政策旨在規範使用者（以下簡稱「您」）在使用水星伺服器網（以下簡稱「本網站」）時，本網站對您的個人資料之蒐集、處理、使用、儲存及保護等事項，並告知您相關權利。
      </p>

      <p class="text-lg font-semibold mt-4">
        本網站重視您的隱私權，並致力於保護您的個人資料。本隱私政策說明本網站如何蒐集、使用及處理您的個人資料。
      </p>

      <p class="text-lg font-semibold mt-4">
        當您使用本網站服務時，即表示您同意本隱私政策之內容。
      </p>

      <h1 class="text-lg mt-4">一、個人資料的蒐集與使用</h1>
      <ol>
        <li>
          蒐集的資料類型
          <ol>
            <li>
              您在註冊、填寫表單或使用本網站服務時提供的資料，例如姓名、電子郵件地址、電話號碼等。
            </li>
            <li>您的瀏覽紀錄、IP 地址、裝置資訊等技術資料。</li>
            <li>您與本網站的互動記錄，例如客服對話、回饋意見。</li>
          </ol>
        </li>
        <li class="mt-2">
          蒐集的使用目的
          <ol>
            <li>提供、維護及改進本網站服務。</li>
            <li>回應您的查詢及提供客戶支援。</li>
            <li>進行行銷與廣告相關活動。</li>
            <li>符合法律法規的要求。</li>
          </ol>
        </li>
      </ol>

      <h1 class="text-lg mt-4">二、個人資料的保護</h1>
      <ol>
        <li>
          本網站採取以下措施保護您的個人資料
          <ol>
            <li>使用加密技術確保資料傳輸過程中的安全。</li>
            <li>採用權限控管，限制未經授權之人員存取您的資料。</li>
            <li>定期檢視系統安全，並修補潛在的安全漏洞。</li>
          </ol>
        </li>
      </ol>

      <h1 class="text-lg mt-4">三、瀏覽外部連結時的資料處理</h1>
      <ol>
        <li>
          本網站可能包含指向其他網站的連結。本網站無法控制這些第三方網站的隱私政策及其資料處理方式。
        </li>
        <li class="mt-2">
          建議您在使用這些網站時，查閱其隱私政策以了解如何保護您的個人資料。
        </li>
      </ol>

      <h1 class="text-lg mt-4">四、個人資料的刪除與更正</h1>
      <ol>
        <li>
          刪除個人資料
          <ol>
            <li>
              您可隨時聯繫本網站要求刪除您的個人資料，除非法律另有規定或為完成交易必要，本網站將在合理時間內處理您的請求。
            </li>
          </ol>
        </li>
        <li class="mt-2">
          更正個人資料
          <ol>
            <li>
              若您的個人資料有誤，您可透過本網站提供的聯絡方式要求更正，本網站將於核實後立即進行更新。
            </li>
          </ol>
        </li>
      </ol>

      <h1 class="text-lg mt-4">五、與第三方共用個人資料</h1>
      <ol>
        <li>
          共用情境
          <ol>
            <li>
              為提供服務必要，可能與合作夥伴或服務提供商共用，例如支付處理或物流服務。
            </li>
            <li>在法律要求下或保護本網站權益時提供給相關機構。</li>
          </ol>
        </li>
        <li class="mt-2">
          共用原則
          <ol>
            <li>僅共用必要的資料，並要求第三方遵守本政策或更高的隱私標準。</li>
          </ol>
        </li>
      </ol>

      <h1 class="text-lg mt-4">六、隱私政策的更新</h1>
      <ol>
        <li>
          本網站保留隨時修改本隱私政策的權利。若有重大變更，本網站將透過電子郵件或網站公告通知您。建議您定期查閱本政策，以確保瞭解最新內容。
        </li>
      </ol>

      <h1 class="text-lg mt-4">七、聯絡方式</h1>
      <ol>
        <li>
          若您對本隱私政策有任何疑問，或需行使您的個人資料權利，請使用電子郵件，或前往「聯絡我們」分頁尋找網站管理人員的Discord，本網站將竭誠為您服務。
        </li>
      </ol>

      <div class="text-lg mt-4 font-semibold">
        感謝您信任本網站，本網站將竭力保護您的個人資料及隱私。
      </div>
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
