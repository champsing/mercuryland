// TODO: Update Leaderboard style
<script setup lang="ts">
import axios from "axios";
import { BASE_URL } from "@/composables/utils";
import { ref, Ref } from "vue";
import { VaDataTable, VaButton, VaIcon, VaPopover } from "vuestic-ui";
import { ArrowClockwise24Filled } from "@vicons/fluent";

document.title = "水星排行 - 水星人的夢幻樂園";

const leaderboard: Ref<Coin[]> = ref([]);
const lastUpdated: Ref<string> = ref("載入中...");

function refreshLeaderboard() {
  axios
    .get(BASE_URL + "/api/leaderboard")
    .then((response) => {
      leaderboard.value = response.data
        .map((x) => {
          const coin: Coin = {
            rank: 0, // Placeholder for rank, will be assigned later,
            id: x.id,
            coin: x.coin,
            display: x.display,
            updated_at: new Date(x.updated_at).getTime(),
          };
          return coin;
        })
        .sort((a, b) => {
          return a.coin < b.coin ? 1 : -1; // Sort by coin in descending order
        })
        .sort((a, b) => {
          return a.id.localeCompare(b.id, "en") ? 1 : -1; // Sort by updated_at in descending order
        })
        .map((x, index) => {
          x.rank = index + 1; // Assign rank based on the sorted order
          return x;
        })
        .slice(0, 50);
      console.log(leaderboard.value);
      lastUpdated.value = new Date().toLocaleString();
      console.log("已更新排行榜", lastUpdated.value);
    })
    .catch((error) => {
      console.error("更新排行榜失敗:", error);
    });
}

refreshLeaderboard();

class Coin {
  rank: number;
  id: string;
  coin: number;
  display: string;
  updated_at: number;
}

const CENTER = "center" as const;

const columns = [
  {
    key: "rank",
    label: "名次",
    tdAlign: CENTER,
    thAlign: CENTER,
    width: 140,
    sortable: true,
    sortingOptions: ["desc" as const, "asc" as const, null],
  },
  {
    key: "display",
    label: "帳號顯示名稱",
    tdAlign: CENTER,
    thAlign: CENTER,
    width: 250,
    sortable: true,
    sortingOptions: ["desc" as const, "asc" as const, null],
  },
  {
    key: "coin",
    label: "水星幣數量",
    tdAlign: CENTER,
    thAlign: CENTER,
    width: 200,
  },
  {
    key: "updated_at",
    label: "最後更新時間",
    tdAlign: CENTER,
    thAlign: CENTER,
    width: 140,
    sortable: true,
    sortingOptions: ["desc" as const, "asc" as const, null],
  },
];
</script>

<template>
  <div class="flex flex-row text-center justify-center gap-10 mt-4">
    <div class="text-center text-3xl font-bold justify-center mt-5 mb-5">
      水星排行
    </div>
    <div class="flex flex-col text-center justify-center gap-4">
      <div class="text-base text-zinc-400">
        這裡顯示的是水星幣的排行榜，每次直播獲得的水星幣都會在這裡顯示。
      </div>
      <div>
        上次更新時間： {{ lastUpdated }}
        <div class="inline ml-4 select-none">
          <VaButton color="info" preset="plain" @click="refreshLeaderboard()">
            <div class="flex flex-row gap-2">
              <VaIcon size="medium">
                <ArrowClockwise24Filled />
              </VaIcon>
              立即更新排行榜
            </div>
          </VaButton>
        </div>
      </div>
    </div>
  </div>
  <div class="h-80vh mx-10">
    <VaDataTable
      :items="leaderboard"
      :columns="columns"
      class="mt-5 mb-3"
      style="--va-data-table-hover-color: #357286"
      virtual-scroller
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
              :href="`https://www.youtube.com/channel/${row.rowData.id}`"
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
              :href="`https://www.youtube.com/channel/${row.rowData.id}`"
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
              :href="`https://www.youtube.com/channel/${row.rowData.id}`"
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
              :href="`https://www.youtube.com/channel/${row.rowData.id}`"
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
  </div>
</template>
