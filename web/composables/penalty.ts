import penaltyStatus from "@assets/data/penalty_status.json";

export function statusOf(status: string): (typeof penaltyStatus)[0] {
    return penaltyStatus.filter((x) => x.name == status)[0];
}

export function stateString(state: number): string {
    const statuses = ["未生效", "未開始", "進行中", "勉強過", "已完成"];
    return statuses[state] || "未知";
}
