import { describe, expect, it } from "vitest";
import { samplePenalties, sampleVods, withDevSamples } from "../sampleData";

const TODAY = new Date(2026, 10, 23); // 2026-11-23, local time.

describe("samplePenalties", () => {
    it("returns clearly-marked rows dated from today backwards", () => {
        const items = samplePenalties(TODAY);

        expect(items.length).toBeGreaterThan(0);
        expect(items[0].date).toBe("2026-11-23");
        expect(items[1].date).toBe("2026-11-22");
        expect(
            items.every((item) => item.name.startsWith("測試懲罰項目")),
        ).toBe(true);
        expect(items.every((item) => item.id > 0)).toBe(true);
    });

    it("covers several states so the palette can be previewed", () => {
        const states = new Set(
            samplePenalties(TODAY).map((item) => item.state),
        );

        expect(states.size).toBeGreaterThan(1);
    });
});

describe("sampleVods", () => {
    it("returns rows with the fields the VOD list renders", () => {
        const items = sampleVods(TODAY);

        expect(items.length).toBeGreaterThan(0);
        for (const item of items) {
            expect(item.date <= "2026-11-23").toBe(true);
            expect(item.title.startsWith("測試直播")).toBe(true);
            expect(item.link).not.toBe("");
            expect(item.duration).not.toBe("");
            expect(Array.isArray(item.tags)).toBe(true);
        }
    });

    it("includes tagged and untagged rows so filtering can be previewed", () => {
        const items = sampleVods(TODAY);

        expect(items.some((item) => item.tags.length > 0)).toBe(true);
        expect(items.some((item) => item.tags.length === 0)).toBe(true);
    });
});

describe("withDevSamples", () => {
    const samples = ["sample"];

    it("falls back to samples in development when there is no data", () => {
        expect(withDevSamples([], samples, true)).toBe(samples);
    });

    it("keeps real data in development", () => {
        const items = ["real"];

        expect(withDevSamples(items, samples, true)).toBe(items);
    });

    it("never shows samples in production", () => {
        expect(withDevSamples([], samples, false)).toEqual([]);
    });
});
