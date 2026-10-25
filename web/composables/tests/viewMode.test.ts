import { beforeEach, describe, expect, it } from "vitest";
import {
    blockViewModeStorageKey,
    isBlockViewMode,
    readBlockViewMode,
    useBlockViewMode,
    writeBlockViewMode,
} from "../viewMode";

describe("isBlockViewMode", () => {
    it("accepts the known modes", () => {
        expect(isBlockViewMode("grid")).toBe(true);
        expect(isBlockViewMode("list")).toBe(true);
    });

    it("rejects everything else", () => {
        expect(isBlockViewMode("table")).toBe(false);
        expect(isBlockViewMode(null)).toBe(false);
        expect(isBlockViewMode(undefined)).toBe(false);
        expect(isBlockViewMode(3)).toBe(false);
    });
});

describe("readBlockViewMode", () => {
    beforeEach(() => localStorage.clear());

    it("returns the fallback when nothing is stored", () => {
        expect(readBlockViewMode("penalty")).toBe("grid");
        expect(readBlockViewMode("penalty", "list")).toBe("list");
    });

    it("returns the stored mode", () => {
        localStorage.setItem(blockViewModeStorageKey("vod"), "list");
        expect(readBlockViewMode("vod")).toBe("list");
    });

    it("ignores unknown stored values", () => {
        localStorage.setItem(blockViewModeStorageKey("vod"), "wat");
        expect(readBlockViewMode("vod", "grid")).toBe("grid");
    });

    it("keeps keys independent", () => {
        writeBlockViewMode("penalty", "list");
        expect(readBlockViewMode("penalty")).toBe("list");
        expect(readBlockViewMode("vod")).toBe("grid");
    });
});

describe("writeBlockViewMode", () => {
    beforeEach(() => localStorage.clear());

    it("round-trips through localStorage", () => {
        writeBlockViewMode("penalty", "list");
        expect(localStorage.getItem(blockViewModeStorageKey("penalty"))).toBe(
            "list",
        );
    });
});

describe("useBlockViewMode", () => {
    beforeEach(() => localStorage.clear());

    it("starts from the persisted value", () => {
        writeBlockViewMode("penalty", "list");
        expect(useBlockViewMode("penalty").value).toBe("list");
    });

    it("starts from the fallback when nothing is persisted", () => {
        expect(useBlockViewMode("vod").value).toBe("grid");
        expect(useBlockViewMode("vod", "list").value).toBe("list");
    });

    it("persists changes as soon as they are assigned", () => {
        const mode = useBlockViewMode("vod");
        mode.value = "list";
        expect(mode.value).toBe("list");
        expect(readBlockViewMode("vod")).toBe("list");
    });

    it("ignores unknown values", () => {
        const mode = useBlockViewMode("vod");
        mode.value = "nope" as never;
        expect(mode.value).toBe("grid");
        expect(readBlockViewMode("vod")).toBe("grid");
    });
});
