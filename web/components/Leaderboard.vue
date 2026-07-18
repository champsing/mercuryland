<script setup lang="ts">
import api from "@composables/axios";
import { ArrowClockwise24Filled } from "@vicons/fluent";
import { onMounted, ref, Ref } from "vue";
import { VaButton, VaDivider, VaIcon } from "vuestic-ui";

document.title = "水星排行 - 水星人的夢幻樂園";

const leaderboard: Ref<UserRank[]> = ref([]);

function loadLeaderboard() {
    api.get("/api/leaderboard")
        .then((response) => {
            leaderboard.value = response.data
                .map((u: User) => {
                    return {
                        rank: 0,
                        youtube: u.youtube,
                        coin: u.coin,
                        display: u.display,
                        updated_at: u.updated_at,
                    } as UserRank;
                })
                .sort((a: UserRank, b: UserRank) => {
                    return a.coin < b.coin ? 1 : -1;
                })
                .map((r: UserRank, index: number) => {
                    r.rank = index + 1;
                    return r;
                })
                .slice(0, 50);
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

function chineseDate(ts: number): string {
    const d = new Date(ts);
    return `${d.getFullYear()}年${d.getMonth() + 1}月${d.getDate()}日`;
}

function medalColor(rank: number): string {
    if (rank === 1) return "text-yellow-400";
    if (rank === 2) return "text-zinc-300";
    if (rank === 3) return "text-amber-500";
    return "text-zinc-600";
}

function medalBg(rank: number): string {
    if (rank === 1) return "bg-yellow-400/15";
    if (rank === 2) return "bg-zinc-300/10";
    if (rank === 3) return "bg-amber-500/10";
    return "bg-zinc-700/50";
}

function medalBorder(rank: number): string {
    if (rank === 1) return "border-yellow-400/40";
    if (rank === 2) return "border-zinc-300/25";
    if (rank === 3) return "border-amber-500/30";
    return "border-zinc-700/50";
}

function medalEmoji(rank: number): string {
    if (rank === 1) return "👑";
    if (rank === 2) return "🥈";
    if (rank === 3) return "🥉";
    return "";
}

function podiumOrder(rank: number): string {
    if (rank === 1) return "md:order-2";
    if (rank === 2) return "md:order-1";
    if (rank === 3) return "md:order-3";
    return "";
}
</script>

<template>
    <main
        class="lb-page min-h-[calc(100vh-48px)] text-[#f7f7f8] pt-10 pb-5 px-4 max-md:pt-16 max-md:pb-4 max-md:px-3"
    >
        <div class="w-full max-w-[1200px] mx-auto">
            <!-- Header -->
            <div
                class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 mb-6 px-4"
            >
                <div>
                    <h1 class="text-3xl font-bold text-neutral-100">
                        水星排行
                    </h1>
                    <p class="text-sm text-zinc-400 mt-1">
                        每次直播獲得的水星幣都會在這裡顯示，致敬貢獻最多的水星人
                    </p>
                </div>
                <VaButton
                    preset="plain"
                    color="info"
                    @click="loadLeaderboard()"
                    class="self-start"
                >
                    <VaIcon><ArrowClockwise24Filled /></VaIcon>
                    <span class="ml-1 text-sm">重新整理</span>
                </VaButton>
            </div>

            <!-- Top 3 Podium -->
            <section
                v-if="leaderboard.length > 0"
                class="px-4 mb-8"
                aria-label="前三名榮譽榜"
            >
                <div
                    class="flex flex-col md:flex-row items-center justify-center gap-4 md:gap-6 md:items-end"
                >
                    <div
                        v-for="user in leaderboard.slice(0, 3)"
                        :key="user.rank"
                        :class="[
                            podiumOrder(user.rank),
                            medalBorder(user.rank),
                            medalBg(user.rank),
                        ]"
                        class="flex flex-col items-center rounded-2xl border p-5 md:p-6 w-full max-w-[240px] backdrop-blur-sm transition-all duration-300 hover:scale-105 hover:[box-shadow:0_8px_32px_rgba(0,0,0,0.35)]"
                    >
                        <!-- Rank badge -->
                        <div
                            :class="medalColor(user.rank)"
                            class="text-4xl mb-2"
                        >
                            {{ medalEmoji(user.rank) }}
                        </div>
                        <div
                            :class="medalColor(user.rank)"
                            class="text-lg font-bold tracking-wider"
                        >
                            第 {{ user.rank }} 名
                        </div>

                        <!-- Avatar initial -->
                        <div
                            :class="[
                                medalBg(user.rank),
                                medalBorder(user.rank),
                                medalColor(user.rank),
                            ]"
                            class="w-16 h-16 rounded-full border flex items-center justify-center text-2xl font-bold my-3"
                        >
                            {{ user.display.charAt(0) }}
                        </div>

                        <!-- Name -->
                        <a
                            :href="`https://www.youtube.com/channel/${user.youtube}`"
                            target="_blank"
                            rel="noopener noreferrer"
                            :class="medalColor(user.rank)"
                            class="text-base font-semibold text-center hover:underline truncate max-w-full"
                        >
                            {{ user.display }}
                        </a>

                        <!-- Coin -->
                        <div class="flex items-baseline gap-1 mt-2">
                            <span
                                :class="medalColor(user.rank)"
                                class="text-2xl font-extrabold"
                            >
                                {{ user.coin.toLocaleString() }}
                            </span>
                            <span class="text-xs text-zinc-500">水星幣</span>
                        </div>

                        <!-- Last seen -->
                        <div class="text-xs text-zinc-500 mt-2">
                            {{ chineseDate(user.updated_at) }}
                        </div>
                    </div>
                </div>
            </section>

            <!-- Empty state -->
            <section
                v-if="leaderboard.length === 0"
                class="flex flex-col items-center justify-center py-20 text-zinc-500"
            >
                <div class="text-6xl mb-4">🏆</div>
                <p class="text-lg">尚無排行資料</p>
                <p class="text-sm mt-1">點擊重新整理按鈕載入排行榜</p>
            </section>

            <!-- Remaining ranks (4–50) -->
            <section
                v-if="leaderboard.length > 3"
                class="px-4 pb-8"
                aria-label="其餘排行"
            >
                <div class="flex items-center gap-3 mb-3">
                    <VaDivider class="grow !m-0" />
                    <span class="text-sm text-zinc-500 whitespace-nowrap"
                        >第 4 – {{ leaderboard.length }} 名</span
                    >
                    <VaDivider class="grow !m-0" />
                </div>

                <div
                    class="grid gap-2 [grid-template-columns:repeat(auto-fill,minmax(320px,1fr))] max-md:grid-cols-1"
                >
                    <div
                        v-for="user in leaderboard.slice(3)"
                        :key="user.rank"
                        class="flex items-center gap-4 rounded-xl border border-zinc-700/40 bg-zinc-800/40 backdrop-blur-sm px-4 py-3 transition-colors transition-transform duration-200 hover:translate-x-0.5 hover:bg-zinc-700/40 hover:border-zinc-600/50"
                    >
                        <!-- Rank number -->
                        <div
                            class="w-10 h-10 rounded-full bg-zinc-700/60 flex items-center justify-center text-sm font-bold text-zinc-300 shrink-0"
                        >
                            {{ user.rank }}
                        </div>

                        <!-- Name -->
                        <div class="flex-1 min-w-0">
                            <a
                                :href="`https://www.youtube.com/channel/${user.youtube}`"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-zinc-200 font-medium hover:text-info hover:underline truncate block"
                            >
                                {{ user.display }}
                            </a>
                            <div class="text-xs text-zinc-500">
                                {{ chineseDate(user.updated_at) }}
                            </div>
                        </div>

                        <!-- Coin -->
                        <div class="flex items-baseline gap-1 shrink-0">
                            <span
                                class="text-lg font-bold text-zinc-200 tabular-nums"
                            >
                                {{ user.coin.toLocaleString() }}
                            </span>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    </main>
</template>

<style>
.lb-page {
    background:
        radial-gradient(
            circle at 50% 0%,
            rgba(255, 215, 0, 0.08),
            transparent 36rem
        ),
        radial-gradient(
            circle at 15% 0%,
            rgba(88, 166, 255, 0.2),
            transparent 32rem
        ),
        linear-gradient(135deg, #0f1117 0%, #16191f 48%, #101318 100%);
}
</style>
