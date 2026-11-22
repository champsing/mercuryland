import type { PenItem } from "@/composables/penalty";
import type { VodItem } from "@/composables/vod";

/** "YYYY-MM-DD" in local time, matching the API's date strings. */
function dateString(date: Date): string {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    return `${year}-${month}-${day}`;
}

function daysBefore(today: Date, days: number): string {
    const date = new Date(today);
    date.setDate(date.getDate() - days);
    return dateString(date);
}

// The names carry a visible 測試 marker so sample rows can never be mistaken
// for real data while developing.

const PENALTY_NAMES = [
    "拍自己恐怖遊戲直播的reaction video",
    "開台前先唱一首歌",
    "用中文玩一局恐怖遊戲",
    "挑戰一小時不看聊天室",
    "介紹自己的收藏",
    "重溫舊精華並吐槽",
    "觀眾點歌環節",
    "限時完成一個小遊戲",
];

/** Placeholder rows for the penalty list, dated from today backwards. */
export function samplePenalties(today = new Date()): PenItem[] {
    return PENALTY_NAMES.map((name, index) => ({
        id: 1001 + index,
        date: daysBefore(today, index),
        name: `測試懲罰項目 ${index + 1}：${name}`,
        detail: "",
        state: index % 5,
        history: [],
    }));
}

const VOD_TITLES = [
    "恐怖遊戲初體驗",
    "跟觀眾一起聊天",
    "週末耐久台",
    "新遊戲試玩",
    "深夜雜談",
    "精華回顧",
];

const VOD_DURATIONS = ["1:12:34", "2:03:45", "3:21:09", "0:58:12", "1:44:56"];

/** Placeholder rows for the VOD list, dated from yesterday backwards. */
export function sampleVods(today = new Date()): VodItem[] {
    return VOD_TITLES.map((title, index) => ({
        id: 2001 + index,
        date: daysBefore(today, index + 1),
        link: "dQw4w9WgXcQ",
        title: `測試直播 ${index + 1}：${title}`,
        tags:
            index % 3 === 0
                ? ["測試"]
                : index % 3 === 1
                  ? ["測試", "遊戲"]
                  : [],
        duration: VOD_DURATIONS[index % VOD_DURATIONS.length],
    }));
}

/**
 * Sample rows are a development aid only. Production keeps the empty state so
 * placeholder data never masquerades as real penalties or VODs.
 */
export function withDevSamples<T>(
    items: T[],
    samples: T[],
    isDev = import.meta.env.DEV,
): T[] {
    if (!isDev || items.length > 0) return items;
    return samples;
}
