<script setup lang="ts">
import { useAuthState } from "@/composables/authState";
import { BlockViewMode } from "@/composables/viewMode";
import { VodItem } from "@/composables/vod";
import { Play24Filled } from "@vicons/fluent";
import { computed, ref } from "vue";
import { VaButton, VaChip, VaIcon, VaScrollContainer } from "vuestic-ui";
import SortMenu from "../SortMenu.vue";
import ViewModeToggle from "../ViewModeToggle.vue";

const props = defineProps<{
    dateRange: { start: Date; end: Date };
    selectedTags: string[];
    strictFiltering: boolean;
    vodData: VodItem[];
}>();

const viewMode = defineModel<BlockViewMode>("viewMode", { default: "grid" });
const emit = defineEmits<{
    (e: "updateTag", tag: string): void;
    (e: "editVod", vod: VodItem): void;
    (e: "addVod"): void;
}>();

const authState = useAuthState();
const showActions = computed(() => authState.isAuthenticated);

const YOUTUBE_LIVE = "https://youtube.com/live/";

const sortField = ref<string>("date");
const sortDirection = ref<"asc" | "desc">("desc");

const sortOptions = [
    { value: "date", label: "日期", defaultDirection: "desc" as const },
    { value: "title", label: "名稱", defaultDirection: "asc" as const },
];

function compareVod(lhs: VodItem, rhs: VodItem): number {
    const result =
        sortField.value === "title"
            ? lhs.title.localeCompare(rhs.title, "zh-Hant")
            : lhs.date.localeCompare(rhs.date);
    return sortDirection.value === "asc" ? result : -result;
}

const data = computed(() => {
    return props.vodData
        .filter(
            (v) =>
                v.date >=
                    //prettier-ignore
                    props.dateRange.start.toISOString().slice(0, 10) &&
                v.date <=
                    //prettier-ignore
                    new Date(props.dateRange.end.getTime() + 28800000).toISOString().slice(0, 10),
        )
        .filter((v) => {
            if (props.selectedTags.length === 0) return true;

            if (props.strictFiltering === true) {
                return props.selectedTags.every((tag) => v.tags.includes(tag));
            }

            return v.tags.some((tag) => props.selectedTags.includes(tag));
        })
        .sort(compareVod);
});

function videoUrl(vod: VodItem): string {
    return `${YOUTUBE_LIVE}${vod.link}`;
}

const TAG_LIMIT = 3;
const expandedTags = ref<Set<number>>(new Set());

function isTagsExpanded(vod: VodItem): boolean {
    return expandedTags.value.has(vod.id);
}

function visibleTags(vod: VodItem): string[] {
    return isTagsExpanded(vod) ? vod.tags : vod.tags.slice(0, TAG_LIMIT);
}

function hiddenTagCount(vod: VodItem): number {
    return Math.max(vod.tags.length - TAG_LIMIT, 0);
}

function toggleTags(vod: VodItem) {
    const next = new Set(expandedTags.value);
    if (next.has(vod.id)) {
        next.delete(vod.id);
    } else {
        next.add(vod.id);
    }
    expandedTags.value = next;
}
</script>

<template>
    <div class="flex h-full min-h-0 flex-col">
        <div
            class="mb-3 flex w-full flex-wrap items-center justify-between gap-2 px-1 md:grid md:grid-cols-[1fr_auto_1fr] md:items-center md:gap-3"
        >
            <span aria-hidden="true" class="hidden md:block" />
            <p
                class="flex items-center gap-1 text-sm text-zinc-400 md:justify-center"
            >
                <span>共</span>
                <span
                    class="text-base font-bold leading-none text-zinc-100 tabular-nums"
                >
                    {{ data.length }}
                </span>
                <span>筆直播</span>
            </p>
            <div class="ml-auto flex flex-wrap items-center justify-end gap-2">
                <ViewModeToggle v-model="viewMode" />
                <SortMenu
                    v-model:field="sortField"
                    v-model:direction="sortDirection"
                    :options="sortOptions"
                />
                <VaButton
                    v-if="showActions"
                    class="rounded-lg font-extrabold [&_.va-button__content]:gap-[0.35rem]"
                    color="#5bc6a1"
                    aria-label="新增直播"
                    @click="$emit('addVod')"
                >
                    <VaIcon name="add" />
                    <span class="max-md:hidden">新增直播</span>
                </VaButton>
            </div>
        </div>

        <VaScrollContainer
            vertical
            color="#aec789"
            size="medium"
            class="flex-1 min-h-0"
        >
            <div
                v-if="data.length && viewMode === 'grid'"
                class="grid w-full grid-cols-1 gap-3 pb-24 pr-1 md:grid-cols-2 lg:grid-cols-3"
            >
                <article
                    v-for="vod in data"
                    :key="vod.id"
                    class="flex flex-col gap-3 rounded-2xl border border-zinc-700/40 bg-zinc-800/40 p-8 backdrop-blur-sm transition-all duration-200 hover:-translate-y-0.5 hover:border-[#5bc6a1]/40 hover:bg-zinc-700/40 hover:shadow-[0_8px_32px_rgba(0,0,0,0.35)]"
                >
                    <header class="flex min-w-0 flex-col gap-1.5">
                        <span
                            class="text-base font-semibold tracking-wide text-[#5bc6a1] tabular-nums"
                        >
                            {{ vod.date }}
                        </span>
                        <h3
                            class="line-clamp-2 break-words text-[22px] font-bold leading-snug text-white"
                        >
                            <a
                                :href="videoUrl(vod)"
                                target="_blank"
                                rel="noreferrer noopener"
                                :title="vod.title"
                                class="text-white transition-colors hover:text-[#5bc6a1]"
                            >
                                {{ vod.title }}
                            </a>
                        </h3>
                    </header>

                    <div class="flex flex-wrap items-center gap-1.5">
                        <VaChip
                            v-for="tag in visibleTags(vod)"
                            :key="tag"
                            color="#4feed1"
                            outline
                            size="small"
                            class="cursor-pointer"
                            @click="emit('updateTag', tag)"
                        >
                            {{ tag }}
                        </VaChip>
                        <button
                            v-if="hiddenTagCount(vod) && !isTagsExpanded(vod)"
                            type="button"
                            class="inline-flex items-center rounded-full border border-[#4feed1] px-2 py-0.5 text-xs font-semibold text-[#4feed1] transition-colors hover:bg-[#4feed1]/10"
                            :aria-label="`展開其餘 ${hiddenTagCount(vod)} 個 TAG`"
                            @click="toggleTags(vod)"
                        >
                            +{{ hiddenTagCount(vod) }}
                        </button>
                        <button
                            v-else-if="isTagsExpanded(vod)"
                            type="button"
                            class="inline-flex h-6 w-6 items-center justify-center rounded-full border border-[#4feed1] text-[#4feed1] transition-colors hover:bg-[#4feed1]/10"
                            aria-label="收合 TAG"
                            @click="toggleTags(vod)"
                        >
                            <VaIcon name="expand_less" size="small" />
                        </button>
                        <span
                            v-if="!vod.tags.length"
                            class="text-xs text-zinc-500"
                        >
                            無 TAG
                        </span>
                    </div>

                    <footer
                        class="mt-auto flex items-end justify-between gap-3 pt-1"
                    >
                        <div
                            class="flex items-center gap-1.5 text-xs font-medium text-[#aeb7c7]"
                        >
                            <VaIcon name="schedule" size="small" />
                            <span class="tabular-nums">{{ vod.duration }}</span>
                        </div>

                        <div class="flex items-center gap-2">
                            <button
                                v-if="showActions"
                                type="button"
                                class="inline-flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-[#5bc6a1] text-white shadow-md shadow-black/20 transition-colors hover:bg-[#3db088] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#5bc6a1]"
                                aria-label="編輯直播"
                                title="編輯直播"
                                @click="emit('editVod', vod)"
                            >
                                <VaIcon name="edit" size="small" />
                            </button>
                            <a
                                :href="videoUrl(vod)"
                                target="_blank"
                                rel="noreferrer noopener"
                                :aria-label="`觀看直播：${vod.title}`"
                                :title="`觀看直播：${vod.title}`"
                                class="inline-flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-[#ff0000] text-white shadow-lg shadow-black/30 transition-all duration-200 hover:scale-105 hover:bg-[#e60000] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#ff0000]"
                            >
                                <VaIcon color="#ffffff" size="1.5rem">
                                    <Play24Filled />
                                </VaIcon>
                            </a>
                        </div>
                    </footer>
                </article>
            </div>

            <div v-else-if="data.length" class="w-full pb-24 pr-1">
                <div
                    class="mb-1 hidden grid-cols-[8rem_minmax(0,1fr)_13rem_5rem_2.25rem] items-center gap-3 px-4 text-xs font-semibold tracking-wide text-zinc-500 md:grid"
                    aria-hidden="true"
                >
                    <span>日期</span>
                    <span>名稱</span>
                    <span>TAG</span>
                    <span class="text-right">時長</span>
                    <span />
                </div>

                <div class="flex w-full flex-col gap-1.5">
                    <article
                        v-for="vod in data"
                        :key="vod.id"
                        class="grid grid-cols-[minmax(0,1fr)_auto_auto] items-center gap-x-3 gap-y-1.5 rounded-xl border border-zinc-700/40 bg-zinc-800/40 px-4 py-2.5 backdrop-blur-sm transition-colors hover:border-[#5bc6a1]/40 hover:bg-zinc-700/40 md:grid-cols-[8rem_minmax(0,1fr)_13rem_5rem_2.25rem] md:gap-3"
                    >
                        <div
                            class="col-start-1 row-start-1 flex min-w-0 items-center gap-2"
                        >
                            <span
                                class="text-sm font-semibold tracking-wide text-[#5bc6a1] tabular-nums"
                            >
                                {{ vod.date }}
                            </span>
                        </div>

                        <div
                            class="col-span-3 row-start-2 flex min-w-0 items-center gap-2 md:col-span-1 md:col-start-2 md:row-start-1"
                        >
                            <a
                                :href="videoUrl(vod)"
                                target="_blank"
                                rel="noreferrer noopener"
                                :aria-label="`觀看直播：${vod.title}`"
                                :title="`觀看直播：${vod.title}`"
                                class="inline-flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-[#ff0000] text-white shadow-sm shadow-black/25 transition-colors hover:bg-[#e60000] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#ff0000]"
                            >
                                <VaIcon color="#ffffff" size="0.9rem">
                                    <Play24Filled />
                                </VaIcon>
                            </a>
                            <span
                                class="truncate text-base font-bold text-white"
                                :title="vod.title"
                            >
                                {{ vod.title }}
                            </span>
                        </div>

                        <div
                            class="col-span-3 row-start-3 flex min-w-0 flex-wrap items-center gap-1.5 md:col-span-1 md:col-start-3 md:row-start-1"
                        >
                            <VaChip
                                v-for="tag in visibleTags(vod)"
                                :key="tag"
                                color="#4feed1"
                                outline
                                size="small"
                                class="cursor-pointer"
                                @click="emit('updateTag', tag)"
                            >
                                {{ tag }}
                            </VaChip>
                            <button
                                v-if="
                                    hiddenTagCount(vod) && !isTagsExpanded(vod)
                                "
                                type="button"
                                class="inline-flex cursor-pointer items-center rounded-full border border-[#4feed1] px-2 py-0.5 text-xs font-semibold text-[#4feed1] transition-colors hover:bg-[#4feed1]/10"
                                :aria-label="`展開其餘 ${hiddenTagCount(vod)} 個 TAG`"
                                @click="toggleTags(vod)"
                            >
                                +{{ hiddenTagCount(vod) }}
                            </button>
                            <button
                                v-else-if="isTagsExpanded(vod)"
                                type="button"
                                class="inline-flex h-6 w-6 cursor-pointer items-center justify-center rounded-full border border-[#4feed1] text-[#4feed1] transition-colors hover:bg-[#4feed1]/10"
                                aria-label="收合 TAG"
                                @click="toggleTags(vod)"
                            >
                                <VaIcon name="expand_less" size="small" />
                            </button>
                            <span
                                v-if="!vod.tags.length"
                                class="text-xs text-zinc-500"
                            >
                                無 TAG
                            </span>
                        </div>

                        <div
                            class="col-start-2 row-start-1 flex items-center gap-1.5 justify-self-end text-xs font-medium text-[#aeb7c7] md:col-start-4 md:row-start-1"
                        >
                            <VaIcon name="schedule" size="small" />
                            <span class="tabular-nums">{{ vod.duration }}</span>
                        </div>

                        <button
                            v-if="showActions"
                            type="button"
                            class="col-start-3 row-start-1 inline-flex h-9 w-9 shrink-0 cursor-pointer items-center justify-center justify-self-end rounded-full bg-[#5bc6a1] text-white shadow-md shadow-black/20 transition-colors hover:bg-[#3db088] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#5bc6a1] md:col-start-5 md:row-start-1"
                            aria-label="編輯直播"
                            title="編輯直播"
                            @click="emit('editVod', vod)"
                        >
                            <VaIcon name="edit" size="small" />
                        </button>
                    </article>
                </div>
            </div>

            <div
                v-else
                class="flex flex-col items-center justify-center py-20 text-zinc-500"
            >
                <VaIcon name="live_tv" size="large" class="mb-3 opacity-60" />
                <p class="text-lg">沒有符合條件的直播</p>
                <p class="mt-1 text-sm">試著調整日期或 TAG 篩選</p>
            </div>
        </VaScrollContainer>
    </div>
</template>
