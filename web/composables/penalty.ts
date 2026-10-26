import { BASE_URL } from "@/composables/utils";
import penaltyStatus from "@assets/data/penalty_status.json";
import api from "@composables/axios";
import { computed, ref } from "vue";

export interface PenItem {
    id: number;
    date: string;
    name: string;
    detail: string;
    state: number;
    history: [number, string][];
}

export function statusOf(status: string): (typeof penaltyStatus)[0] {
    return penaltyStatus.filter((x) => x.name == status)[0];
}

export function stateString(state: number): string {
    const statuses = ["未生效", "未完成", "進行中", "勉強過", "已完成"];
    return statuses[state] || "未知";
}

export function stateColor(state: number, mode: "bg" | "text" | "raw"): string {
    const colors = {
        bg: ["bg-5a716e", "bg-b91c1c", "bg-b45309", "bg-047857", "bg-4d7c0f"],
        text: [
            "text-7a938f",
            "text-f87171",
            "text-f59e0b",
            "text-10b981",
            "text-84cc16",
        ],
        raw: ["#7a938f", "#f87171", "#f59e0b", "#10b981", "#84cc16"],
    };
    return colors[mode][state] || colors[mode][0];
}

// ✅ 新增：純函數，專門用來替換 HTML 中的圖片相對路徑
export function formatDetailHtml(detailContent?: string): string {
    if (!detailContent) return "";
    const base = BASE_URL.replace(/\/$/, "");
    return detailContent.replace(
        /src=(['"])(\/api\/image\/[^'"]+)\1/g,
        (_match, quote, path) => `src=${quote}${base}${path}${quote}`,
    );
}

// ✅ 新增：組合式函數，封裝獲取單筆資料與狀態
export function usePenaltyDetail() {
    const penalty = ref<PenItem | null>(null);
    const isLoading = ref(false);

    const renderedDetail = computed(() => {
        return formatDetailHtml(penalty.value?.detail);
    });

    const loadPenalty = async (id: number) => {
        isLoading.value = true;
        try {
            const response = await api.get(`/api/penalty/detail/${id}`);
            if (response.status === 200) {
                penalty.value = response.data;
            }
        } catch (error) {
            console.error("載入懲罰詳情失敗:", error);
        } finally {
            isLoading.value = false;
        }
    };

    return {
        penalty,
        isLoading,
        loadPenalty,
        renderedDetail,
    };
}
