<script setup lang="ts">
import axios from "axios";
import { BASE_URL } from "@/composables/utils";
import { onMounted, ref, Ref } from "vue";
import { UseElementBounding } from "@vueuse/components";
import {
  VaDataTable,
  VaButton,
  VaIcon,
  VaDivider,
  VaScrollContainer,
  VaCard,
} from "vuestic-ui";
import { ArrowClockwise24Filled } from "@vicons/fluent";
import { calcHeight } from "@/composables/style";

document.title = "水星排行 - 水星人的夢幻樂園";

const leaderboard: Ref<UserRank[]> = ref([]);

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
            updated_at: u.updated_at,
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

onMounted(loadLeaderboard);

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
    tdAlign: "center" as const,
    thAlign: "center" as const,
    sortable: true,
    sortingOptions: ["desc" as const, "asc" as const, null],
  },
  {
    key: "display",
    label: "名稱",
    tdAlign: "center" as const,
    thAlign: "center" as const,
  },
  {
    key: "coin",
    label: "水星幣",
    tdAlign: "center" as const,
    thAlign: "center" as const,
  },
  {
    key: "updated_at",
    label: "最近出現",
    tdAlign: "center" as const,
    thAlign: "center" as const,
    sortable: true,
    sortingOptions: ["desc" as const, "asc" as const, null],
  },
];

function rankStyle(rank: number) {
  if (rank === 1) {
    return "text-yellow-400 font-bold text-2xl";
  } else if (rank === 2) {
    return "text-zinc-400 font-bold text-xl";
  } else if (rank === 3) {
    return "text-amber-600 font-bold text-lg";
  } else {
    return "text-white";
  }
}

function rankEmoji(rank: number) {
  if (rank == 1) {
    return "🥇";
  } else if (rank === 2) {
    return "🥈";
  } else if (rank === 3) {
    return "🥉";
  } else {
    return "ㅤ"; // Invisible character to maintain layout
  }
}

const MB_VA_CARD = 8;
</script>

<template>
  <div
    class="flex h-14 w-full flex-row items-center justify-between gap-4 px-2"
  >
    <h1 class="ml-12 text-2xl font-semibold">水星排行</h1>
    <div class="flex items-center">
      <VaButton preset="plain" @click="loadLeaderboard()"
        ><VaIcon><ArrowClockwise24Filled /></VaIcon
      ></VaButton>
      <p class="text-zinc-400 sm:text-right ml-2">
        這裡顯示的是水星幣的排行榜，每次直播獲得的水星幣都會在這裡顯示。
      </p>
    </div>
  </div>
  <VaDivider class="w-full !mt-0 !mb-2" />

  <VaCard class="m-2 overflow-hidden rounded-xl">
    <use-element-bounding v-slot="{ top }">
      <VaScrollContainer
        vertical
        color="#e0feb4"
        size="medium"
        class="h-full"
        :style="calcHeight(top, MB_VA_CARD)"
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
          <template
            v-for="column in columns"
            #[`header(${column.key})`]="{ label }"
            :key="column.key"
          >
            <div class="text-sm text-center">
              {{ label }}
            </div>
          </template>
          <template #cell(rank)="{ value, row }">
            <div class="text-center">
              <div :class="rankStyle(row.rowData.rank)">
                {{ rankEmoji(row.rowData.rank) }}第 {{ value }} 名
              </div>
            </div>
          </template>
          <template #cell(display)="{ value, row }">
            <div class="text-center">
              <VaButton
                :href="`https://www.youtube.com/channel/${row.rowData.youtube}`"
                target="_blank"
                preset="plain"
                rel="noopener noreferrer"
              >
                <div :class="rankStyle(row.rowData.rank)">
                  {{ value }}
                </div>
              </VaButton>
            </div>
          </template>
          <template #cell(coin)="{ value, row }">
            <div class="text-center">
              <div :class="rankStyle(row.rowData.rank)">
                {{ value }}
              </div>
            </div>
          </template>
          <template #cell(updated_at)="{ value }">
            <div class="text-center">
              {{ new Date(value).toDateString() }}
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
