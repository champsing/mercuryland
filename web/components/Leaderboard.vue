<script setup lang="ts">
import axios from "axios";
import { BASE_URL } from "@/composables/utils";
import { ref, Ref } from "vue";
import { VaDataTable, VaButton, VaIcon, VaPopover } from "vuestic-ui";
import { ArrowClockwise24Filled } from "@vicons/fluent";

const leaderboard: Ref<Coin[]> = ref(null);
const lastUpdated: Ref<string> = ref("");
const updateCooldown: Ref<number> = ref(0);

function refreshLeaderboard() {
    if (updateCooldown.value > 0) {
        console.log("更新冷卻中，請稍後再試。");
        return;
    }
    sessionStorage.removeItem("leaderboard");
    sessionStorage.removeItem("lastUpdated");
    axios
        .get(BASE_URL + "/api/leaderboard")
        .then((response) => {
            leaderboard.value = response.data
                .map((x) => {
                    const coin: Coin = {
                        rank: 0, // Placeholder for rank, will be assigned later
                        coin: x.coin,
                        display: x.display,
                        updated_at: new Date(x.updated_at).toLocaleString(),
                    };
                    return coin;
                })
                .sort((a, b) => {
                    return a.coin < b.coin ? 1 : -1; // Sort by coin in descending order
                })
                .map((x, index) => {
                    x.rank = index + 1; // Assign rank based on the sorted order
                    return x;
                });
            console.log(leaderboard.value);
            sessionStorage.setItem(
                "leaderboard",
                JSON.stringify(leaderboard.value)
            );
            lastUpdated.value = new Date().toLocaleString();
            sessionStorage.setItem("lastUpdated", lastUpdated.value);
            console.log("已更新排行榜", lastUpdated.value);
        })
        .catch((error) => {
            console.error("更新排行榜失敗:", error);
        })
        .finally(() => {
            updateCooldown.value = 20; // Set cooldown to 20 seconds
            const cooldownInterval = setInterval(() => {
                if (updateCooldown.value > 0) {
                    updateCooldown.value--;
                } else {
                    clearInterval(cooldownInterval);
                }
            }, 1000); // Decrease cooldown every second
        });
}

function within5mins(record: string): boolean {
    const lastUpdatedTime = new Date(record);
    const currentTime = new Date();
    return currentTime.getTime() - lastUpdatedTime.getTime() <= 5 * 60 * 1000; // 5 minutes in milliseconds
}

if (sessionStorage.getItem("leaderboard")) {
    const lastUpdatedRecord = sessionStorage.getItem("lastUpdated");
    if (lastUpdatedRecord && within5mins(lastUpdatedRecord)) {
        // 5分鐘內
        lastUpdated.value = lastUpdatedRecord;
    } else {
        lastUpdated.value = new Date().toLocaleString();
        sessionStorage.setItem("lastUpdated", lastUpdated.value);
    }
    leaderboard.value = JSON.parse(sessionStorage.getItem("leaderboard"));
} else refreshLeaderboard();

class Coin {
    rank: number;
    coin: number;
    display: string;
    updated_at: string;
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
    },
];

setInterval(() => {
    refreshLeaderboard();
}, 1000 * 60 * 5); // 每 5 分鐘自動更新排行榜
</script>

<template>
    <div>
        <div class="text-center text-3xl font-bold mt-10 mb-5">水星排行</div>
        <div class="flex flex-col text-center">
            <div class="text-base text-zinc-400 mb-5">
                這裡顯示的是水星幣的排行榜，數據每 5 分鐘會定期更新。
            </div>
            <div>
                上次更新時間： {{ lastUpdated }}
                <div class="inline ml-4 select-none">
                    <VaButton
                        color="info"
                        preset="plain"
                        @click="refreshLeaderboard()"
                        :disabled="updateCooldown > 0"
                    >
                        <div class="flex flex-row gap-2">
                            <VaIcon size="medium">
                                <ArrowClockwise24Filled />
                            </VaIcon>
                            立即更新排行榜
                            <span v-if="updateCooldown > 0"
                                >({{ updateCooldown }})</span
                            >
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
                            第 {{ value }} 名
                        </div>
                    </div>
                    <div v-else-if="row.rowData.rank == 2">
                        <div class="text-zinc-400 font-bold text-xl">
                            第 {{ value }} 名
                        </div>
                    </div>
                    <div v-else-if="row.rowData.rank == 3">
                        <div class="text-amber-600 font-bold text-lg">
                            第 {{ value }} 名
                        </div>
                    </div>
                    <div v-else>第 {{ value }} 名</div>
                </div>
            </template>
            <template #cell(display)="{ value, row }">
                <div class="text-center">
                    <div v-if="row.rowData.rank == 1">
                        <div class="text-yellow-400 font-bold text-xl">
                            {{ value }}
                        </div>
                    </div>
                    <div v-else-if="row.rowData.rank == 2">
                        <div class="text-zinc-400 font-bold text-xl">
                            {{ value }}
                        </div>
                    </div>
                    <div v-else-if="row.rowData.rank == 3">
                        <div class="text-amber-600 font-bold text-xl">
                            {{ value }}
                        </div>
                    </div>
                    <div v-else>{{ value }}</div>
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
        </VaDataTable>
    </div>
</template>
