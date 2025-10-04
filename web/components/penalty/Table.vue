<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import {
  VaButton,
  VaChip,
  VaDataTable,
  VaDivider,
  VaIcon,
  VaModal,
  VaProgressBar,
} from "vuestic-ui";
import penaltyData from "@assets/data/penalty.json";
import vodData from "@assets/data/vod.json";
import { openLinks, ofId, truncateString } from "@/composables/utils";
import { statusOf } from "@/composables/penalty";

const props = defineProps<{
  dateRange: { start: Date; end: Date };
  status?: string;
  search: string;
}>();

const emit = defineEmits<{
  (e: "updateStatus", status: string): void;
}>();

class PenaltyDataEntry {
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

const YOUTUBE_LIVE = "https://youtube.com/live/";

const showPEM = ref(false); // showPenaltyEntryModal
const showPenaltyScreenshotModal = ref(false);

const PEMContent: Ref<PenaltyDataEntry> = defineModel("PEMContent", {
  default: null,
  set(value) {
    showPEM.value = !showPEM.value;
    return value;
  },
}); // penaltyEntryModalContent

// [DONE] 修正成跟 DataTable.vue 裡面一樣使用 columns {row} 形式
const items = computed(() =>
  filterPenaltyData(props.dateRange, props.status, props.search).slice(),
);

const columns = [
  {
    key: "date",
    label: "日期",
    tdAlign: "center" as const,
    thAlign: "center" as const,
    sortable: true,
    sortingOptions: ["desc" as const, "asc" as const, null],
  },
  {
    key: "name",
    label: "內容",
    tdAlign: "center" as const,
    thAlign: "center" as const,
  },
  {
    key: "status",
    label: "完成狀態",
    tdAlign: "center" as const,
    thAlign: "center" as const,
  },
];

function vodLinkOfDate(date: string): string[] {
  let linkIDArray = vodData.filter((x) => x.date == date).map((x) => x.link);
  for (let i = 0; i < linkIDArray.length; i++)
    linkIDArray[i] = YOUTUBE_LIVE + linkIDArray[i];
  return linkIDArray;
}

function filterPenaltyData(
  dateRange: { start: Date; end: Date },
  status: string,
  search: string,
): typeof penaltyData {
  return penaltyData
    .filter(
      (v) =>
        v.date >= dateRange.start.toISOString().slice(0, 10) &&
        v.date <=
          new Date(dateRange.end.getTime() + 28800000)
            .toISOString()
            .slice(0, 10),
    )
    .filter((v) => status == null || status == v.status)
    .filter(
      (v) =>
        search == "" || v.name.toLowerCase().includes(search.toLowerCase()),
    )
    .sort((lhs, rhs) => lhs.date.localeCompare(rhs.date));
}
</script>

<template>
  <!-- !bg-[#6d8581] !bg-[#b91c1c] !bg-[#4d7c0f] !bg-[#047857] !bg-[#b45309] -->
  <!-- TAILWIND CSS: DO NOT REMOVE ABOVE COMMENT -->
  <VaDataTable
    :items="items"
    :columns="columns"
    class="w-full"
    style="--va-data-table-hover-color: #357286"
    virtual-scroller
    sticky-header
    hoverable
  >
    <template
      v-for="column in columns"
      #[`header(${column.key})`]="{ label }"
      :key="column.key"
    >
      <div class="text-sm text-center">
        {{ label }}
      </div>
    </template>

    <!-- check day of week:  {{ new Date(value).getDay() }} -->
    <template #cell(date)="{ value, row }">
      <div class="text-center">
        <div v-if="row.rowData.status == '未生效'">----</div>
        <div v-else>
          <VaButton
            color="textPrimary"
            preset="plain"
            class=""
            @click="openLinks(vodLinkOfDate(value))"
          >
            {{ value }}
          </VaButton>
        </div>
      </div>
    </template>

    <template #cell(name)="{ value, row }">
      <div class="text-center">
        <VaButton
          @click="PEMContent = row.rowData"
          preset="plain"
          color="textPrimary"
        >
          {{ truncateString(value, 25) }}
        </VaButton>
      </div>
    </template>
    <template #cell(status)="{ value }">
      <div class="text-center" :class="`!bg-[${statusOf(value).color}]`">
        <VaButton
          @click="() => emit('updateStatus', value)"
          preset="plain"
          color="textPrimary"
        >
          {{ value }}
        </VaButton>
      </div>
    </template>
  </VaDataTable>

  <VaModal v-model="showPEM" hide-default-actions size="small" close-button>
    <!-- 本體 -->
    <div class="text-xl">
      {{ PEMContent.name }}
      <VaChip
        readonly
        outline
        size="small"
        :color="`${statusOf(PEMContent.status).color}`"
        class="ml-4"
      >
        ● {{ PEMContent.status }}
      </VaChip>
    </div>

    <!-- 如果尚未生效 -->
    <div v-if="PEMContent.status == '未生效'" class="mt-2">
      <span class="text-sm text-gray-400 font-bold">
        這個懲罰目前尚未生效，請耐心等候惡靈獲得新懲罰
      </span>

      <div class="text-xl text-gray-400 font-bold">
        抽出日期：
        <span class="text-xl text-orange-300">
          {{ PEMContent.date }}
        </span>
      </div>
    </div>

    <!-- 補充說明 -->
    <div v-if="PEMContent.description !== undefined" class="mt-4">
      <template v-for="block in PEMContent.description">
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
            @click="
              () => {
                PEMContent = ofId(penaltyData, block.uri_num);
                showPEM = !showPEM;
              }
            "
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
    <template v-if="PEMContent.progress !== undefined">
      <VaProgressBar
        class="mt-4"
        :model-value="PEMContent.progress"
        content-inside
        show-percent
      />
    </template>

    <!-- 復活 -->
    <template v-if="PEMContent.reapply !== undefined">
      <div class="mt-3">
        <span class="text-base">
          😇&nbsp;復活&ensp;
          <div class="inline text-2xl text-orange-300">
            <!-- prettier-ignore -->
            {{ PEMContent.reapply?.length }}
          </div>
          &ensp;次
        </span>
      </div>
      <VaDivider class="!m-1" />
    </template>

    <!-- 復活次數 -->
    <template v-for="entry in PEMContent.reapply">
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
    <template v-if="PEMContent.steamID !== undefined">
      <VaDivider class="!mt-4 !mb-2" />
      <iframe
        :src="`https://store.steampowered.com/widget/${PEMContent.steamID}/`"
        frameborder="0"
        width="520"
        height="150"
      />
    </template>
  </VaModal>
</template>
