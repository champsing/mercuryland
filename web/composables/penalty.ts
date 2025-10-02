import penaltyStatus from "@assets/data/penalty_status.json";

export function statusOf(status: string): (typeof penaltyStatus)[0] {
  return penaltyStatus.filter((x) => x.name == status)[0];
}
