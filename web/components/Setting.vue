<script setup lang="ts">
// TODO: Make the wheel password into the setting page
// TODO: Make vote channel, exchange channel & penalty channel into setting page
import { ref, watch } from "vue";
import axios from "axios";
import {
  VaButton,
  VaCard,
  VaCardContent,
  VaCardTitle,
  VaDivider,
  VaModal,
} from "vuestic-ui";
import { BASE_URL } from "@/composables/utils";

document.title = "系统设置 - 水星人的夢幻樂園";

const showVodUploadModal = ref(false);
const vodUploadFile = ref<File | null>(null);
const vodUploadMessage = ref<string | null>(null);
const vodUploadError = ref<string | null>(null);
const vodUploadInputResetKey = ref(0);
const isVodUploading = ref(false);
const isVodDownloading = ref(false);

const showImageUploadModal = ref(false);
const imageUploadFile = ref<File | null>(null);
const imageUploadMessage = ref<string | null>(null);
const imageUploadError = ref<string | null>(null);
const imageUploadInputResetKey = ref(0);
const isImageUploading = ref(false);
const uploadedImageUrl = ref<string | null>(null);

watch(showVodUploadModal, (visible) => {
  if (!visible) {
    resetVodUploadState();
  }
});

watch(showImageUploadModal, (visible) => {
  if (!visible) {
    resetImageUploadState();
  }
});

function resetVodUploadState() {
  vodUploadFile.value = null;
  vodUploadMessage.value = null;
  vodUploadError.value = null;
  vodUploadInputResetKey.value += 1;
  isVodUploading.value = false;
}

function resetImageUploadState() {
  imageUploadFile.value = null;
  imageUploadMessage.value = null;
  imageUploadError.value = null;
  imageUploadInputResetKey.value += 1;
  isImageUploading.value = false;
  uploadedImageUrl.value = null;
}

function openVodUploadModal() {
  vodUploadMessage.value = null;
  vodUploadError.value = null;
  showVodUploadModal.value = true;
}

function openImageUploadModal() {
  imageUploadMessage.value = null;
  imageUploadError.value = null;
  showImageUploadModal.value = true;
}

function onVodFileSelected(event: Event) {
  const target = event.target as HTMLInputElement;
  vodUploadFile.value = target.files?.[0] ?? null;
  vodUploadMessage.value = null;
  vodUploadError.value = null;
}

function onImageFileSelected(event: Event) {
  const target = event.target as HTMLInputElement;
  imageUploadFile.value = target.files?.[0] ?? null;
  imageUploadMessage.value = null;
  imageUploadError.value = null;
}

async function submitVodUpload() {
  if (isVodUploading.value) return;

  if (!vodUploadFile.value) {
    vodUploadError.value = "請選擇要上傳的 JSON 檔案";
    return;
  }

  const token = localStorage.getItem("token");
  if (!token) {
    vodUploadError.value = "請先登入管理員帳號";
    return;
  }

  const formData = new FormData();
  formData.append("token", token);
  formData.append("file", vodUploadFile.value);

  try {
    isVodUploading.value = true;
    vodUploadError.value = null;
    await axios.post(`${BASE_URL}/api/video/upload-json`, formData, {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    });
    vodUploadMessage.value = "上傳完成";
    setTimeout(() => {
      showVodUploadModal.value = false;
    }, 800);
  } catch (error) {
    console.error("VOD JSON upload failed", error);
    vodUploadError.value = "上傳失敗，請稍後再試";
  } finally {
    isVodUploading.value = false;
  }
}

async function submitImageUpload() {
  if (isImageUploading.value) return;

  if (!imageUploadFile.value) {
    imageUploadError.value = "請選擇要上傳的圖片檔案";
    return;
  }

  const token = localStorage.getItem("token");
  if (!token) {
    imageUploadError.value = "請先登入管理員帳號";
    return;
  }

  const formData = new FormData();
  formData.append("token", token);
  formData.append("file", imageUploadFile.value);

  try {
    isImageUploading.value = true;
    imageUploadError.value = null;
    const response = await axios.post(`${BASE_URL}/api/image/upload`, formData, {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    });
    imageUploadMessage.value = "上傳完成";
    uploadedImageUrl.value = response.data.url;
  } catch (error) {
    console.error("Image upload failed", error);
    imageUploadError.value = "上傳失敗，請稍後再試";
  } finally {
    isImageUploading.value = false;
  }
}

async function downloadVodJson() {
  if (isVodDownloading.value) return;

  try {
    isVodDownloading.value = true;
    const response = await axios.get(`${BASE_URL}/api/video/list`, {
      responseType: "blob",
    });

    const blob = new Blob([response.data], { type: "application/json" });
    const url = window.URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = `vod-${new Date().toISOString().slice(0, 10)}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    window.URL.revokeObjectURL(url);
  } catch (error) {
    console.error("VOD JSON download failed", error);
  } finally {
    isVodDownloading.value = false;
  }
}
</script>

<template>
  <div
    class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2"
  >
    <h1 class="ml-12 text-2xl font-semibold">系统设置</h1>
    <p class="text-zinc-400 sm:text-right">
      僅供管理員使用的系統設定頁面。請透過 Discord 驗證登入後再進行相關操作。
    </p>
  </div>
  <VaDivider class="w-full !mt-0 !mb-2" />

  <div class="grid grid-cols-3 gap-2 px-2">
    <VaCard class="rounded-xl border border-zinc-700">
      <VaCardTitle
        class="px-6 pt-6 text-lg font-medium text-zinc-200"
        style="font-size: 20px; justify-content: center"
      >
        直播随选
      </VaCardTitle>
      <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
        <div class="flex gap-2">
          <VaButton
            preset="primary"
            color="info"
            class="flex-1"
            @click="openVodUploadModal"
          >
            上传JSON
          </VaButton>
          <VaButton
            preset="primary"
            color="info"
            class="flex-1"
            :loading="isVodDownloading"
            @click="downloadVodJson"
          >
            下载JSON
          </VaButton>
        </div>
      </VaCardContent>
    </VaCard>
    <VaCard class="rounded-xl border border-zinc-700">
      <VaCardTitle
        class="px-6 pt-6 text-lg font-medium text-zinc-200"
        style="font-size: 20px; justify-content: center"
      >
        圖片上傳
      </VaCardTitle>
      <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
        <VaButton
          preset="primary"
          color="success"
          class="w-full"
          @click="openImageUploadModal"
        >
          上傳圖片
        </VaButton>
      </VaCardContent>
    </VaCard>
    <VaCard class="rounded-xl border border-zinc-700">
      <VaCardTitle
        class="px-6 pt-6 text-lg font-medium text-zinc-200"
        style="font-size: 20px; justify-content: center"
      >
        使用者與權限
      </VaCardTitle>
      <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
        功能尚未開放，後續更新將於此頁面公布。
      </VaCardContent>
    </VaCard>
    <VaCard class="rounded-xl border border-zinc-700">
      <VaCardTitle
        class="px-6 pt-6 text-lg font-medium text-zinc-200"
        style="font-size: 20px; justify-content: center"
      >
        通知與整合
      </VaCardTitle>
      <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
        功能尚未開放，請持續關注開發進度。
      </VaCardContent>
    </VaCard>
    <VaCard class="rounded-xl border border-zinc-700">
      <VaCardTitle
        class="px-6 pt-6 text-lg font-medium text-zinc-200"
        style="font-size: 20px; justify-content: center"
      >
        系統維護
      </VaCardTitle>
      <VaCardContent class="px-6 pb-6 text-sm text-zinc-300">
        功能尚未開放，仍在規畫中，敬請期待。
      </VaCardContent>
    </VaCard>
  </div>

  <VaModal
    v-model="showVodUploadModal"
    hide-default-actions
    close-button
    max-width="420px"
  >
    <div class="flex flex-col gap-4 p-4">
      <div class="text-base text-zinc-200">上傳直播隨選 JSON</div>
      <p class="text-sm text-zinc-400">
        選擇要匯入的 JSON 檔案，系統會逐筆寫入資料庫並略過失敗的項目。
      </p>
      <input
        :key="vodUploadInputResetKey"
        type="file"
        accept="application/json"
        class="text-sm text-zinc-200"
        @change="onVodFileSelected"
      />
      <p v-if="vodUploadError" class="text-sm text-red-400">
        {{ vodUploadError }}
      </p>
      <p v-if="vodUploadMessage" class="text-sm text-emerald-400">
        {{ vodUploadMessage }}
      </p>
      <div class="flex justify-end gap-2">
        <VaButton
          preset="secondary"
          @click="showVodUploadModal = false"
          :disabled="isVodUploading"
        >
          取消
        </VaButton>
        <VaButton
          color="info"
          :loading="isVodUploading"
          @click="submitVodUpload"
        >
          確認上傳
        </VaButton>
      </div>
    </div>
  </VaModal>

  <VaModal
    v-model="showImageUploadModal"
    hide-default-actions
    close-button
    max-width="420px"
  >
    <div class="flex flex-col gap-4 p-4">
      <div class="text-base text-zinc-200">上傳圖片</div>
      <p class="text-sm text-zinc-400">
        選擇要上傳的圖片檔案，支援 JPG、PNG、GIF 等格式，檔案大小限制為 1MB。
      </p>
      <input
        :key="imageUploadInputResetKey"
        type="file"
        accept="image/*"
        class="text-sm text-zinc-200"
        @change="onImageFileSelected"
      />
      <p v-if="imageUploadError" class="text-sm text-red-400">
        {{ imageUploadError }}
      </p>
      <p v-if="imageUploadMessage" class="text-sm text-emerald-400">
        {{ imageUploadMessage }}
      </p>
      <div v-if="uploadedImageUrl" class="mt-2">
        <p class="text-sm text-zinc-300 mb-2">上傳成功！圖片連結：</p>
        <a
          :href="uploadedImageUrl"
          target="_blank"
          class="text-sm text-blue-400 underline break-all"
        >
          {{ uploadedImageUrl }}
        </a>
      </div>
      <div class="flex justify-end gap-2">
        <VaButton
          preset="secondary"
          @click="showImageUploadModal = false"
          :disabled="isImageUploading"
        >
          取消
        </VaButton>
        <VaButton
          color="success"
          :loading="isImageUploading"
          @click="submitImageUpload"
        >
          確認上傳
        </VaButton>
      </div>
    </div>
  </VaModal>
</template>

<style scoped>
section {
  min-height: calc(100vh - 8rem);
}
</style>
