<script setup lang="ts">
import { ref } from "vue";
import {
  VaButton,
  VaChip,
  VaDivider,
  VaIcon,
  VaModal,
  VaProgressBar,
} from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import vodData from "@assets/data/vod.json";
import { openLinks, ofId } from "@/composables/utils";
import { statusOf } from "@/composables/penalty";

interface PenaltyDataEntry {
  id: number;
  date: string;
  name: string;
  status: string;
  description?: {
    type: string;
    text?: string;
    uri_link?: string;
    uri_imgs?: string[];
    uri_num?: number;
  }[];
  reapply?: { date: string; status: string }[];
  steamID?: number;
  progress?: number;
}

const props = defineProps<{
  modelValue: boolean;
  penalty: PenaltyDataEntry | null;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
  (e: "changePenalty", penalty: PenaltyDataEntry): void;
}>();

const YOUTUBE_LIVE = "https://youtube.com/live/";
const showPenaltyScreenshotModal = ref(false);

function vodLinkOfDate(date: string): string[] {
  let linkIDArray = vodData.filter((x) => x.date == date).map((x) => x.link);
  for (let i = 0; i < linkIDArray.length; i++)
    linkIDArray[i] = YOUTUBE_LIVE + linkIDArray[i];
  return linkIDArray;
}
</script>

<template>
  <VaModal
    :model-value="props.modelValue"
    @update:model-value="emit('update:modelValue', $event)"
    hide-default-actions
    size="small"
    close-button
  >
    <!-- 本體 -->
    <div v-if="props.penalty" class="text-xl">
      {{ props.penalty.name }}
      <VaChip
        readonly
        outline
        size="small"
        :color="`${statusOf(props.penalty.status).color}`"
        class="ml-4"
      >
        ● {{ props.penalty.status }}
      </VaChip>
    </div>

    <!-- 如果尚未生效 -->
    <div v-if="props.penalty && props.penalty.status == '未生效'" class="mt-2">
      <span class="text-sm text-gray-400 font-bold">
        這個懲罰目前尚未生效，請耐心等候惡靈獲得新懲罰
      </span>

      <div class="text-xl text-gray-400 font-bold">
        抽出日期：
        <span class="text-xl text-orange-300">
          {{ props.penalty.date }}
        </span>
      </div>
    </div>

    <!-- 補充說明 -->
    <div
      v-if="props.penalty && props.penalty.description !== undefined"
      class="mt-4"
    >
      <template v-for="block in props.penalty.description">
        <div>
          <span v-if="block.type == 'text'">{{ block.text }}</span>

          <VaButton
            v-if="block.type == 'link'"
            :href="block.uri_link"
            rel="noopener noreferrer"
            preset="plain"
            color="textPrimary"
          >
            <div class="inline-block">{{ block.text }}（連結）</div>
          </VaButton>

          <VaButton
            v-if="block.type == 'vod'"
            :href="YOUTUBE_LIVE + `${ofId(vodData, block.uri_num).link}`"
            target="_blank"
            rel="noopener noreferrer"
            color="#c82828"
            size="small"
            round
            class="mt-2"
          >
            {{ ofId(vodData, block.uri_num).date }}．{{
              ofId(vodData, block.uri_num).title
            }}
          </VaButton>

          <VaButton
            v-if="block.type == 'penalty'"
            @click="emit('changePenalty', ofId(penaltyData, block.uri_num))"
            color="#8fc1ff"
            size="small"
            round
            class="mt-4"
          >
            {{ ofId(penaltyData, block.uri_num).date }}．{{
              ofId(penaltyData, block.uri_num).name
            }}
          </VaButton>

          <VaButton
            v-if="block.type == 'image'"
            @click="showPenaltyScreenshotModal = !showPenaltyScreenshotModal"
            gradient
            color="#0e8110"
            size="medium"
          >
            查看證明圖片
          </VaButton>

          <VaModal
            v-if="block.type == 'image'"
            v-model="showPenaltyScreenshotModal"
            hide-default-actions
            style="--va-modal-padding: 0px; width: max-content; left: 300px"
            ok-text="完成"
          >
            <!-- left need to be calc() -->
            <div class="text-center font-bold">
              <VaIcon name="help_outline" />
              點擊右鍵→[在新分頁開啟]可查看大圖
            </div>
            <div class="flex flex-row gap-4">
              <img
                v-for="img in block.uri_imgs"
                :src="`images/penalty/${img}`"
                class="h-fit"
                :alt="block.text"
              />
            </div>
          </VaModal>

          <br v-if="block.type == 'br'" />
        </div>
      </template>
    </div>

    <!-- 進度條 -->
    <template v-if="props.penalty && props.penalty.progress !== undefined">
      <VaProgressBar
        class="mt-4"
        :model-value="props.penalty.progress"
        content-inside
        show-percent
      />
    </template>

    <!-- 復活 -->
    <template v-if="props.penalty && props.penalty.reapply !== undefined">
      <div class="mt-3">
        <span class="text-base">
          😇&nbsp;復活&ensp;
          <div class="inline text-2xl text-orange-300">
            <!-- prettier-ignore -->
            {{ props.penalty.reapply?.length }}
          </div>
          &ensp;次
        </span>
      </div>
      <VaDivider class="!m-1" />
    </template>

    <!-- 復活次數 -->
    <template v-for="entry in penalty?.reapply">
      <div class="mt-1">
        <VaButton
          @click="openLinks(vodLinkOfDate(entry.date))"
          preset="plain"
          color="textPrimary"
        >
          {{ entry.date }}
        </VaButton>
        &ensp;
        <!-- colorsOfStatus -->
        <div class="inline-block text-sm">
          <div :class="`!text-[${statusOf(entry.status).color}]`">◼</div>
        </div>
        &nbsp;{{ entry.status }}
      </div>
    </template>

    <!-- steam store page -->
    <template v-if="props.penalty && props.penalty.steamID !== undefined">
      <VaDivider class="!mt-4 !mb-2" />
      <iframe
        :src="`https://store.steampowered.com/widget/${props.penalty.steamID}/`"
        frameborder="0"
        width="520"
        height="150"
      />
    </template>
  </VaModal>
</template>
