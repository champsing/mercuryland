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
        本聲明是為規範網際網路連接視聽閱覽者（下稱「您」）存取水星伺服器官網（下稱「本站」）時，所提供之可識別數據、網路連接記錄等可直接識別特定個人連接本站或存取本站提供之內容與服務之身份資訊或資料（下稱個人資料）的蒐集、處理、管理、保存方式，與告知您對於您的個人資料的相關權利事項。
      </p>

      <p class="text-lg font-semibold mt-4">
        本網站（以下簡稱「我們」）非常重視您的隱私，並致力於保護您的個人資料安全。為了讓您了解我們如何蒐集、使用、處理及保護您的個人資料，我們特此制定本隱私政策。
      </p>

      <p class="text-lg font-semibold mt-4">
        請您在使用我們的服務前，仔細閱讀並了解本隱私政策的內容。當您使用我們的服務時，即表示您同意我們按照本隱私政策所述的方式處理您的個人資料。
      </p>

      <h1 class="text-lg mt-4">一、個人資料的蒐集與使用</h1>
      <ol>
        <li>
          <!-- 1.1. -->
          蒐集的資料類型
          <ol>
            <li>
              <!-- 1.1.1. -->
              您在註冊、填寫表單或使用我們的服務時提供的資料，例如姓名、電子郵件地址、電話號碼等。
            </li>
            <li>
              <!-- 1.1.2. -->
              您的瀏覽紀錄、IP 地址、裝置資訊等技術數據。
            </li>
            <li>
              <!-- 1.1.3. -->
              您與我們的互動記錄，例如客服對話、回饋意見。
            </li>
          </ol>
        </li>
        <li class="mt-2">
          <!-- 1.2. -->
          蒐集的使用目的
          <ol>
            <li>
              <!-- 1.2.1. -->
              提供、維護及改進我們的服務。
            </li>
            <li>
              <!-- 1.2.2. -->
              回應您的查詢及提供客戶支援。
            </li>
            <li>
              <!-- 1.2.3. -->
              實現行銷與廣告相關活動。
            </li>
            <li>
              <!-- 1.2.4. -->
              符合法律法規的要求。
            </li>
          </ol>
        </li>
      </ol>
      <h1 class="text-lg mt-4">二、個人資料的保護</h1>

      <ol>
        <li>
          <!-- 2.1. -->
          我們採取以下措施保護您的個人資料
          <ol>
            <li>
              <!-- 2.1.1. -->
              使用加密技術確保傳輸過程中的數據安全。
            </li>
            <li>
              <!-- 2.1.2. -->
              採用權限控管，限制未經授權的人員訪問您的資料。
            </li>
            <li>
              <!-- 2.1.3. -->
              定期檢視系統安全，修補潛在的安全漏洞。
            </li>
          </ol>
        </li>
      </ol>
      <h1 class="text-lg mt-4">三、瀏覽外部連結時的資料處理</h1>
      <ol><li>本網站可能包含指向其他網站的連結。
        我們無法控制這些第三方網站的隱私政策及其資料處理方式。
        </li>
      <li>建議您在使用這些網站時，查閱其隱私政策以了解如何保護您的個人資料。</li>
      </ol>

      <h1 class="text-lg mt-4">四、個人資料的刪除與更正</h1>
      <ol>
        <li>
          <!-- 4.1. -->
          刪除個人資料
          <ol>
            <li>
              <!-- 4.1.1. -->
              您可隨時聯繫我們要求刪除您的個人資料，除非法律另有規定或為完成交易必要，我們將在合理時間內處理您的請求。
            </li>
          </ol>
        </li>
        <li>
          <!-- 4.2. -->
          更正個人資料
          <ol>
            <li>
              <!-- 4.2.1. -->
              若您的個人資料有誤，您可透過我們提供的聯絡方式要求更正，我們將於核實後立即進行更新。
            </li>
          </ol>
        </li>
      </ol>
      <h1 class="text-lg mt-4">五、與第三方共用個人資料</h1>
      <ol>
        <li>
          <!-- 5.1. -->
          共用情境
          <ol>
            <li>
              <!-- 5.1.1. -->
              為提供服務必要，可能與合作夥伴或服務提供商共用，例如支付處理或物流服務。
            </li>
            <li>
              <!-- 5.1.2. -->
              在法律要求下或保護我們權益時提供給相關機構。
            </li>
          </ol>
        </li>
        <li>
          <!-- 2. -->
          共用原則：
          <ol>
            <!-- 2.1. -->
            僅共用必要的資料，並要求第三方遵守本政策或更高的隱私標準。
          </ol>
        </li>
      </ol>
      <h1 class="text-lg mt-4">六、隱私政策的更新</h1>
      <span class="ml-60">
        我們保留隨時修改本隱私政策的權利。若有重大變更，我們將透過電子郵件或網站公告通知您。建議您定期查閱本政策，以確保瞭解最新內容。
      </span>
      <h1 class="text-lg mt-4">七、聯絡方式</h1>
      <div class="ml-60">
        若您對本隱私政策有任何疑問，或需行使您的個人資料權利，請使用電子郵件，或前往「聯絡我們」分頁尋找網站管理人員的Discord，我們將竭誠為您服務。
      </div>
      <div class="ml-60 mt-4">
        電子郵件信箱：
        <VaInput v-model="email" readonly @click="copyToClipboard(email)" />
        <VaButton
          href="mailto:0.flunky-sojourn@icloud.com"
          preset="secondary"
          color="info"
          border-color="info"
          class="ml-3 -mt-1"
        >
          送信
        </VaButton>
        <VaButton
          class="ml-2 -mt-1"
          preset="secondary"
          color="textPrimary"
          border-color="textPrimary"
          to="contact"
        >
          前往「聯絡我們」
        </VaButton>
      </div>
      <div class="mt-10 mb-20 text-2xl text-center">
        感謝您信任我們，我們將竭力保護您的個人資料及隱私。
      </div>
    </div>
  </div>
</template>

<style scoped>
ol li {
  list-style: decimal;
  list-style-position: inside;
}

ol li ol li {
  list-style: lower-alpha;
  list-style-position: inside;
}
</style>
