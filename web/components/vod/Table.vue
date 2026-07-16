<script setup lang="ts">
import { useAuthState } from "@/composables/authState";
import { VodItem } from "@/composables/vod";
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import {
    VaButton,
    VaCard,
    VaCardContent,
    VaChip,
    VaDataTable,
    VaIcon,
    VaModal,
    VaScrollContainer,
} from "vuestic-ui";

const props = defineProps<{
    dateRange: { start: Date; end: Date };
    selectedTags: string[];
    strictFiltering: boolean;
    vodData: VodItem[];
}>();
const emit = defineEmits<{
    (e: "updateTag", tag: string): void;
    (e: "editVod", vod: VodItem): void;
    (e: "addVod"): void;
}>();

const authState = useAuthState();
const showActions = computed(() => authState.isAuthenticated);

const YOUTUBE_LIVE = "https://youtube.com/live/";

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
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
});

const isMdUp = ref(true);
const mql =
    typeof window !== "undefined"
        ? window.matchMedia("(min-width: 768px)")
        : null;

function handleBreakpoint() {
    isMdUp.value = mql?.matches ?? true;
}

onMounted(() => {
    handleBreakpoint();
    mql?.addEventListener("change", handleBreakpoint);
});

onBeforeUnmount(() => {
    mql?.removeEventListener("change", handleBreakpoint);
});

const baseColumns = [
    {
        key: "date",
        label: "日期",
        thAlign: "left" as const,
        tdAlign: "left" as const,
        sortable: true,
        sortingOptions: ["desc" as const, "asc" as const, null],
        width: "132px",
    },
    {
        key: "title",
        label: "標題",
        thAlign: "left" as const,
        tdAlign: "left" as const,
    },
    {
        key: "tags",
        label: "TAG",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        width: "220px",
    },
    {
        key: "duration",
        label: "時長",
        thAlign: "center" as const,
        tdAlign: "center" as const,
        sortable: true,
        width: "104px",
    },
];

const columns = computed(() => {
    let result = [...baseColumns];

    if (!isMdUp.value) {
        result = result.filter(
            (column) => column.key === "date" || column.key === "title",
        );
    }

    if (showActions.value) {
        result.push({
            key: "actions",
            label: "操作",
            thAlign: "center" as const,
            tdAlign: "center" as const,
            width: "76px",
        });
    }

    return result;
});

const headerColumns = computed(() =>
    columns.value.filter((column) => column.key !== "actions"),
);

const modalVod = ref<VodItem | null>(null);

function handleTitleClick(vod: VodItem) {
    if (isMdUp.value) {
        window.open(
            `${YOUTUBE_LIVE}${vod.link}`,
            "_blank",
            "noreferrer noopener",
        );
        return;
    }

    modalVod.value = vod;
}
</script>

<template>
    <VaCard
        class="h-[36rem] min-h-[30rem] md:h-[42rem] lg:h-full lg:min-h-[34rem] overflow-hidden border border-white/10 rounded-xl !bg-[#12151b]/90"
        style="--va-card-padding: 0"
    >
        <VaCardContent class="flex flex-col h-full !p-0 bg-zinc-700/40">
            <VaScrollContainer
                vertical
                horizontal
                color="#aec789"
                size="medium"
                class="flex-1 min-h-0"
            >
                <VaDataTable
                    :items="data"
                    :columns="columns"
                    class="w-full h-full [&_.va-data-table__table-tr]:border-b [&_.va-data-table__table-tr]:border-white/5 [&_.va-data-table__table-td]:p-[0.72rem_0.9rem] [&_.va-data-table__table-th]:p-[0.72rem_0.9rem]"
                    style="
                        --va-data-table-hover-color: rgba(91, 198, 161, 0.12);
                        --va-data-table-thead-background: #0e0f11;
                        --va-data-table-thead-border: 0;
                        height: 100%;
                    "
                    :virtual-scroller="true"
                    sticky-header
                    hoverable
                >
                    <template
                        v-for="column in headerColumns"
                        #[`header(${column.key})`]="{ label }"
                        :key="column.key"
                    >
                        <div
                            class="text-[#aeb7c7] text-[0.76rem] font-extrabold tracking-normal uppercase"
                        >
                            {{ label }}
                        </div>
                    </template>

                    <template v-if="showActions" #header(actions)>
                        <VaButton
                            class="rounded-lg font-extrabold [&_.va-button__content]:gap-[0.35rem]"
                            color="#5bc6a1"
                            aria-label="新增直播"
                            @click="$emit('addVod')"
                        >
                            <VaIcon name="add" />
                        </VaButton>
                    </template>

                    <template #cell(date)="{ value }">
                        <div
                            class="text-[#f7f7f8] text-[0.95rem] whitespace-nowrap font-medium"
                        >
                            {{ value }}
                        </div>
                    </template>

                    <template #cell(title)="{ row }">
                        <button
                            type="button"
                            class="inline-flex items-center w-full max-w-[44rem] border-0 bg-transparent py-[0.4rem] px-0 text-left leading-[1.35] cursor-pointer overflow-hidden text-ellipsis whitespace-nowrap text-[#f7f7f8] hover:text-[#5bc6a1]"
                            @click="handleTitleClick(row.rowData)"
                        >
                            {{ row.rowData.title }}
                        </button>
                    </template>

                    <template #cell(tags)="{ row }">
                        <div class="flex flex-wrap gap-1 justify-center">
                            <VaChip
                                v-for="tag in row.rowData.tags"
                                :key="tag"
                                color="#4feed1"
                                outline
                                size="small"
                                class="my-0.5 cursor-pointer"
                                @click="() => emit('updateTag', tag)"
                            >
                                {{ tag }}
                            </VaChip>
                        </div>
                    </template>

                    <template #cell(duration)="{ value }">
                        <div
                            class="text-[#aeb7c7] text-center text-[0.95rem] font-medium whitespace-nowrap"
                        >
                            {{ value }}
                        </div>
                    </template>

                    <template v-if="showActions" #cell(actions)="{ row }">
                        <VaButton
                            class="w-9 h-9 rounded-lg bg-[#5bc6a1]/10"
                            preset="plain"
                            color="#5bc6a1"
                            aria-label="編輯直播"
                            @click="emit('editVod', row.rowData)"
                        >
                            <VaIcon name="edit" size="small" />
                        </VaButton>
                    </template>
                </VaDataTable>
            </VaScrollContainer>

            <VaModal
                :model-value="modalVod !== null"
                @update:model-value="modalVod = $event ? modalVod : null"
                hide-default-actions
                close-button
                :mobile-fullscreen="false"
                title-class="hidden"
                class="vod-detail-modal"
            >
                <div
                    v-if="modalVod"
                    class="flex flex-col gap-4 w-full min-w-0 max-w-[26rem]"
                >
                    <div
                        class="text-xl font-extrabold tracking-tight leading-tight text-transparent bg-clip-text bg-gradient-to-r from-[#5bc6a1] to-[#3444a2] break-words"
                    >
                        {{ modalVod.title }}
                    </div>

                    <div
                        class="pb-3 border-b border-white/5 flex flex-col gap-3"
                    >
                        <div class="flex flex-col gap-1.5">
                            <div
                                class="text-[#aeb7c7] text-[0.72rem] font-extrabold tracking-normal uppercase"
                            >
                                TAG
                            </div>
                            <div
                                class="flex flex-wrap gap-1.5 text-gray-400 text-2xl"
                            >
                                <VaChip
                                    v-for="tag in modalVod.tags"
                                    :key="tag"
                                    color="#90dc52"
                                    outline
                                    size="small"
                                    class="my-1"
                                    @click="() => emit('updateTag', tag)"
                                >
                                    {{ tag }}
                                </VaChip>
                                <span
                                    v-if="!modalVod.tags.length"
                                    class="text-2xl text-gray-400"
                                    >無 TAG</span
                                >
                            </div>
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <div
                                class="text-[#aeb7c7] text-[0.72rem] font-extrabold tracking-normal uppercase"
                            >
                                時長
                            </div>
                            <div class="text-2xl text-gray-400 font-semibold">
                                {{ modalVod.duration }}
                            </div>
                        </div>
                    </div>

                    <VaButton
                        class="rounded-lg font-extrabold w-full"
                        color="#5bc6a1"
                        :href="`${YOUTUBE_LIVE}${modalVod.link}`"
                        target="_blank"
                        rel="noreferrer noopener"
                    >
                        <VaIcon name="play_circle" size="small" />
                        <span>觀看直播</span>
                    </VaButton>
                </div>
            </VaModal>
        </VaCardContent>
    </VaCard>
</template>

<style scoped>
:deep(.va-data-table__td) {
    display: flex;
    align-items: center;
}
</style>

<style>
.vod-detail-modal .va-modal__dialog {
    background: rgba(15, 23, 42, 0.85);
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    border-radius: 20px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow:
        0 25px 50px -12px rgba(0, 0, 0, 0.35),
        0 0 0 1px rgba(255, 255, 255, 0.04);
    animation: vod-modal-in 0.25s ease;
}

@media (max-width: 767px) {
    .vod-detail-modal .va-modal__dialog {
        max-width: calc(100vw - 1.5rem) !important;
        width: calc(100vw - 1.5rem) !important;
        margin-inline: 0.75rem !important;
    }
}

@keyframes vod-modal-in {
    from {
        opacity: 0;
        transform: translateY(8px) scale(0.98);
    }
    to {
        opacity: 1;
        transform: translateY(0) scale(1);
    }
}
</style>
