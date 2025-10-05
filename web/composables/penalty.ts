import penaltyStatus from "@assets/data/penalty_status.json";

export function statusOf(status: string): (typeof penaltyStatus)[0] {
    return penaltyStatus.filter((x) => x.name == status)[0];
}

export function stateString(state: number): string {
    const statuses = ["未生效", "未開始", "進行中", "勉強過", "已完成"];
    return statuses[state] || "未知";
}

export function stateColor(state: number, mode: "bg" | "text" | "raw"): string {
    const colors = {
        bg: ["bg-6d8581", "bg-b91c1c", "bg-b45309", "bg-047857", "bg-4d7c0f"],
        text: ["text-6d8581", "text-b91c1c", "text-b45309", "text-047857", "text-4d7c0f"],
        raw: ["#6d8581", "#b91c1c", "#b45309", "#047857", "#4d7c0f"],
    };
    return colors[mode][state] || colors[mode][0];
}