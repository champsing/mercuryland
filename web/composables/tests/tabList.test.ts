import { describe, expect, it } from "vitest";
import { nextTabIndex } from "../tabList";

describe("nextTabIndex", () => {
    it("moves right and down to the next tab", () => {
        expect(nextTabIndex("ArrowRight", 0, 3)).toBe(1);
        expect(nextTabIndex("ArrowDown", 1, 3)).toBe(2);
    });

    it("moves left and up to the previous tab", () => {
        expect(nextTabIndex("ArrowLeft", 2, 3)).toBe(1);
        expect(nextTabIndex("ArrowUp", 1, 3)).toBe(0);
    });

    it("wraps at both ends", () => {
        expect(nextTabIndex("ArrowRight", 2, 3)).toBe(0);
        expect(nextTabIndex("ArrowLeft", 0, 3)).toBe(2);
    });

    it("jumps to the first and last tab", () => {
        expect(nextTabIndex("Home", 2, 5)).toBe(0);
        expect(nextTabIndex("End", 1, 5)).toBe(4);
    });

    it("returns null for unrelated keys", () => {
        expect(nextTabIndex("Enter", 0, 3)).toBeNull();
        expect(nextTabIndex("Tab", 0, 3)).toBeNull();
    });

    it("returns null for empty or out-of-range tablists", () => {
        expect(nextTabIndex("ArrowRight", 0, 0)).toBeNull();
        expect(nextTabIndex("ArrowRight", -1, 3)).toBeNull();
        expect(nextTabIndex("ArrowRight", 3, 3)).toBeNull();
    });
});
