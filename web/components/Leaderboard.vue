<script setup lang="ts">
// TODO: Update Leaderboard style
import axios from "axios";
import { BASE_URL } from "@/composables/utils";
import { ref, Ref } from "vue";
import { UseElementBounding } from "@vueuse/components";
import { useWindowSize } from "@vueuse/core";
import { VaDataTable, VaButton, VaIcon, VaPopover, VaDivider, VaScrollContainer, VaCard } from "vuestic-ui";
import { ArrowClockwise24Filled } from "@vicons/fluent";

document.title = "水星排行 - 水星人的夢幻樂園";

const vh = useWindowSize().height;
const leaderboard: Ref<UserRank[]> = ref([]);

function calcStyle(top: number) {
  let parentMarginBottom = 8;
  let parentPaddingBottom = 8;
  let footnoteHeight = 48;

  let delta = parentMarginBottom + footnoteHeight + parentPaddingBottom;
  let height = Math.max(
    vh.value * 0.5,
    vh.value - window.scrollY - top - delta,
  );
  return {
    height: "" + height + "px",
  };
}

function loadLeaderboard() {
  axios
    .get(BASE_URL + "/api/leaderboard")
    .then((response) => {
      leaderboard.value = response.data
        .map((u: User) => {
          return {
            rank: 0, // Placeholder for rank, will be assigned later,
            youtube: u.youtube,
            coin: u.coin,
            display: u.display,
            updated_at: new Date(u.updated_at).getTime(),
          } as UserRank;
        })
        .sort((a: UserRank, b: UserRank) => {
          return a.coin < b.coin ? 1 : -1; // Sort by coin in descending order
        })
        .map((r: UserRank, index: number) => {
          r.rank = index + 1; // Assign rank based on the sorted order
          return r;
        })
        .slice(0, 50);
      console.log(leaderboard.value);
    })
    .catch((error) => {
      console.error("更新排行榜失敗:", error);
    });
}

loadLeaderboard();

class User {
  id: number;
  youtube: string;
  discord: number | null;
  display: string;
  coin: number;
  updated_at: number;
}

class UserRank {
  rank: number;
  youtube: string;
  coin: number;
  display: string;
  updated_at: number;
}

const columns = [
  {
    key: "rank",
    label: "名次",
    tdAlign: "center",
    thAlign: "center",
    sortable: true,
    sortingOptions: ["desc" as const, "asc" as const, null],
  },
  {
    key: "display",
    label: "名稱",
    tdAlign: "center",
    thAlign: "center",
  },
  {
    key: "coin",
    label: "水星幣",
    tdAlign: "center",
    thAlign: "center",
  },
  {
    key: "updated_at",
    label: "最近出现",
    tdAlign: "center",
    thAlign: "center",
    sortable: true,
    sortingOptions: ["desc" as const, "asc" as const, null],
  },
];
</script>

<template>
    <div
    class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2"
  >
    <h1 class="ml-12 text-2xl font-semibold">水星排行</h1>
    <div class="flex items-center">
      <VaButton preset="plain" @click="loadLeaderboard()"><VaIcon><ArrowClockwise24Filled /></VaIcon></VaButton>
    <p class="text-zinc-400 sm:text-right ml-2">
       這裡顯示的是水星幣的排行榜，每次直播獲得的水星幣都會在這裡顯示。
    </p>
    </div>
  </div>
    <VaDivider class="w-full !mt-0 !mb-2" />
  
  <VaCard>
    <use-element-bounding v-slot="{ top }" class="mb-2 mx-10">
      <VaScrollContainer
        vertical
        color="#e0feb4"
        size="medium"
        class="h-full"
        :style="calcStyle(top)"
      >
        <VaDataTable
          :items="leaderboard"
          :columns="columns"
          style="
            --va-data-table-hover-color: #357286;
            --va-data-table-thead-background: var(--va-background-element);
            --va-data-table-thead-border: 0;
            height: 100%;
          "
          :virtual-scroller="false"
          sticky-header
          hoverable
        >
      <template #header(rank)="{ label }">
        <div class="text-sm">
          <VaPopover icon="info" message="點擊可排序">
            {{ label }}
            <VaIcon name="help_outline" />
          </VaPopover>
        </div>
      </template>
      <template #header(display)="{ label }">
        <div class="text-sm">
          <VaPopover
            icon="info"
            message="點擊用戶名稱可以開啟該用戶的 YouTube 頻道頁面"
          >
            {{ label }}
            <VaIcon name="help_outline" />
          </VaPopover>
        </div>
      </template>
      <template #header(updated_at)="{ label }">
        <div class="text-sm">
          <VaPopover icon="info" message="點擊可排序">
            {{ label }}
            <VaIcon name="help_outline" />
          </VaPopover>
        </div>
      </template>
      <template #cell(rank)="{ value, row }">
        <div class="text-center">
          <div v-if="row.rowData.rank == 1">
            <div class="text-yellow-400 font-bold text-2xl">
              🥇第 {{ value }} 名
            </div>
          </div>
          <div v-else-if="row.rowData.rank == 2">
            <div class="text-zinc-400 font-bold text-xl">
              🥈第 {{ value }} 名
            </div>
          </div>
          <div v-else-if="row.rowData.rank == 3">
            <div class="text-amber-600 font-bold text-lg">
              🥉第 {{ value }} 名
            </div>
          </div>
          <div v-else>第 {{ value }} 名</div>
        </div>
      </template>
      <template #cell(display)="{ value, row }">
        <div class="text-center">
          <div v-if="row.rowData.rank == 1">
            <VaButton
              :href="`https://www.youtube.com/channel/${row.rowData.youtube}`"
              target="_blank"
              preset="plain"
              rel="noopener noreferrer"
            >
              <div class="text-yellow-400 font-bold text-xl">
                {{ value }}
              </div>
            </VaButton>
          </div>
          <div v-else-if="row.rowData.rank == 2">
            <VaButton
              :href="`https://www.youtube.com/channel/${row.rowData.youtube}`"
              target="_blank"
              preset="plain"
              rel="noopener noreferrer"
            >
              <div class="text-zinc-400 font-bold text-xl">
                {{ value }}
              </div>
            </VaButton>
          </div>
          <div v-else-if="row.rowData.rank == 3">
            <VaButton
              :href="`https://www.youtube.com/channel/${row.rowData.youtube}`"
              target="_blank"
              preset="plain"
              rel="noopener noreferrer"
            >
              <div class="text-amber-600 font-bold text-xl">
                {{ value }}
              </div>
            </VaButton>
          </div>
          <div v-else>
            <VaButton
              :href="`https://www.youtube.com/channel/${row.rowData.youtube}`"
              target="_blank"
              preset="plain"
              color="textPrimary"
              rel="noopener noreferrer"
            >
              {{ value }}
            </VaButton>
          </div>
        </div>
      </template>
      <template #cell(coin)="{ value, row }">
        <div class="text-center">
          <div v-if="row.rowData.rank == 1">
            <div class="text-yellow-400 text-base font-bold">
              {{ value }}
            </div>
          </div>
          <div v-else-if="row.rowData.rank == 2">
            <div class="text-zinc-400 text-base font-bold">
              {{ value }}
            </div>
          </div>
          <div v-else-if="row.rowData.rank == 3">
            <div class="text-amber-600 text-base font-bold">
              {{ value }}
            </div>
          </div>
          <div v-else>{{ value }}</div>
        </div>
      </template>
      <template #cell(updated_at)="{ value }">
        <div class="text-center">
          {{ new Date(parseInt(value)).toLocaleString() }}
        </div>
      </template>
    </VaDataTable>
  </VaScrollContainer>
</use-element-bounding>
  </VaCard>
</template>

<style scoped>
:deep(.va-data-table__thead) {
  background-color: var(--va-background-element);
}
</style>
