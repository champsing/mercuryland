import { beforeEach, describe, expect, it, vi } from "vitest";

vi.mock("@composables/axios", () => ({
    default: { get: vi.fn() },
}));

import api from "@composables/axios";
import { BASE_URL } from "./utils";
import {
    formatDetailHtml,
    stateColor,
    stateString,
    statusOf,
    usePenaltyDetail,
} from "./penalty";

const get = api.get as unknown as ReturnType<typeof vi.fn>;

describe("statusOf", () => {
    it("returns the matching status object", () => {
        expect(statusOf("未生效")).toEqual({
            color: "#6d8581",
            name: "未生效",
        });
    });

    it("returns undefined for an unknown status", () => {
        expect(statusOf("不存在")).toBeUndefined();
    });
});

describe("stateString", () => {
    it("maps every known state", () => {
        expect(stateString(0)).toBe("未生效");
        expect(stateString(1)).toBe("未完成");
        expect(stateString(2)).toBe("進行中");
        expect(stateString(3)).toBe("勉強過");
        expect(stateString(4)).toBe("已完成");
    });

    it("falls back to 未知 for out-of-range states", () => {
        expect(stateString(99)).toBe("未知");
        expect(stateString(-1)).toBe("未知");
    });
});

describe("stateColor", () => {
    it("returns the raw color for each state", () => {
        expect(stateColor(0, "raw")).toBe("#6d8581");
        expect(stateColor(4, "raw")).toBe("#4d7c0f");
    });

    it("returns the bg/text classes", () => {
        expect(stateColor(1, "bg")).toBe("bg-b91c1c");
        expect(stateColor(2, "text")).toBe("text-b45309");
    });

    it("falls back to the first color for an out-of-range state", () => {
        expect(stateColor(99, "raw")).toBe("#6d8581");
    });
});

describe("formatDetailHtml", () => {
    const base = BASE_URL.replace(/\/$/, "");

    it("returns an empty string for empty input", () => {
        expect(formatDetailHtml(undefined)).toBe("");
        expect(formatDetailHtml("")).toBe("");
    });

    it("rewrites relative image srcs to absolute urls", () => {
        expect(formatDetailHtml('<img src="/api/image/abc">')).toBe(
            `<img src="${base}/api/image/abc">`,
        );
    });

    it("preserves the quote style of the original src", () => {
        expect(formatDetailHtml("<img src='/api/image/abc'>")).toBe(
            `<img src='${base}/api/image/abc'>`,
        );
    });

    it("leaves non-image srcs untouched", () => {
        expect(formatDetailHtml('<img src="/other/path">')).toBe(
            '<img src="/other/path">',
        );
    });
});

describe("usePenaltyDetail", () => {
    beforeEach(() => {
        get.mockReset();
    });

    it("loads a penalty and exposes it", async () => {
        get.mockResolvedValue({
            status: 200,
            data: { id: 7, detail: "<p>hi</p>" },
        });

        const { penalty, isLoading, loadPenalty } = usePenaltyDetail();
        await loadPenalty(7);

        expect(get).toHaveBeenCalledWith("/api/penalty/detail/7");
        expect(penalty.value?.id).toBe(7);
        expect(isLoading.value).toBe(false);
    });

    it("keeps the penalty null when the request fails", async () => {
        get.mockRejectedValue(new Error("boom"));
        const errorSpy = vi
            .spyOn(console, "error")
            .mockImplementation(() => {});

        const { penalty, isLoading, loadPenalty } = usePenaltyDetail();
        await loadPenalty(7);

        expect(penalty.value).toBeNull();
        expect(isLoading.value).toBe(false);
        expect(errorSpy).toHaveBeenCalled();
    });
});
