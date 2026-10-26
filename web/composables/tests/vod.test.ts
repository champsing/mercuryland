import { describe, expect, it } from "vitest";
import {
    formatHMS,
    groupCalculationEntries,
    groupCalculationMonths,
    isoWeekNumber,
    parseHMS,
    type CalculationEntry,
    type CalculationGroup,
} from "../vod";

describe("parseHMS", () => {
    it("parses seconds only", () => {
        expect(parseHMS("45")).toBe(45);
    });

    it("parses minutes and seconds", () => {
        expect(parseHMS("1:30")).toBe(90);
    });

    it("parses hours, minutes and seconds", () => {
        expect(parseHMS("1:02:03")).toBe(3723);
    });

    it("handles an empty input as zero", () => {
        expect(parseHMS("")).toBe(0);
    });
});

describe("formatHMS", () => {
    it("formats seconds with zero padding", () => {
        expect(formatHMS(45)).toBe("00:00:45");
    });

    it("formats minutes and seconds", () => {
        expect(formatHMS(90)).toBe("00:01:30");
    });

    it("formats hours, minutes and seconds", () => {
        expect(formatHMS(3723)).toBe("01:02:03");
    });

    it("formats zero as all zeros", () => {
        expect(formatHMS(0)).toBe("00:00:00");
    });

    it("formats negative values using their absolute value", () => {
        expect(formatHMS(-90)).toBe("00:01:30");
    });
});

describe("groupCalculationEntries", () => {
    function entry(
        overrides: Partial<CalculationEntry> = {},
    ): CalculationEntry {
        return {
            date: "2026-01-07",
            offset: 0,
            previous: 0,
            reason: "直播",
            divider: false,
            ...overrides,
        };
    }

    function plan(overrides: Partial<CalculationEntry> = {}): CalculationEntry {
        return entry({ reason: "計劃", divider: true, ...overrides });
    }

    it("returns an empty array for empty input", () => {
        expect(groupCalculationEntries([])).toEqual([]);
    });

    it("starts the first group at the first entry", () => {
        const entries = [entry({ date: "2026-01-05", previous: 120 })];

        const groups = groupCalculationEntries(entries);

        expect(groups).toHaveLength(1);
        expect(groups[0].start).toBe("2026-01-05");
        expect(groups[0].previous).toBe(120);
        expect(groups[0].entries).toEqual(entries);
    });

    it("keeps a week's entries after its 計劃 in one group", () => {
        const entries = [
            plan({ date: "2026-01-07", offset: 7200, previous: 0 }),
            entry({ date: "2026-01-08", offset: -3900, previous: 7200 }),
            entry({ date: "2026-01-10", offset: -1200, previous: 3300 }),
        ];

        const groups = groupCalculationEntries(entries);

        expect(groups).toHaveLength(1);
        expect(groups[0].start).toBe("2026-01-07");
        expect(groups[0].entries).toEqual(entries);
    });

    it("starts a new group at each 計劃 entry", () => {
        const entries = [
            plan({ date: "2026-01-07", offset: 7200, previous: 0 }),
            entry({ date: "2026-01-09", offset: -3600, previous: 7200 }),
            plan({ date: "2026-01-14", offset: 7200, previous: 3600 }),
            entry({ date: "2026-01-15", offset: -1800, previous: 10800 }),
        ];

        const groups = groupCalculationEntries(entries);

        expect(groups.map((group) => group.start)).toEqual([
            "2026-01-07",
            "2026-01-14",
        ]);
        expect(groups.map((group) => group.entries)).toEqual([
            [entries[0], entries[1]],
            [entries[2], entries[3]],
        ]);
    });

    it("groups entries before the first 計劃 as a partial week", () => {
        const entries = [
            entry({ date: "2026-01-05", previous: 500 }),
            entry({ date: "2026-01-06", previous: 400 }),
            plan({ date: "2026-01-07", previous: 300 }),
        ];

        const groups = groupCalculationEntries(entries);

        expect(groups.map((group) => group.start)).toEqual([
            "2026-01-05",
            "2026-01-07",
        ]);
        expect(groups[0].entries).toEqual([entries[0], entries[1]]);
    });

    it("takes each group's previous from its first entry, not its last", () => {
        const entries = [
            plan({ date: "2026-01-07", offset: 7200, previous: 100 }),
            entry({ date: "2026-01-08", previous: 7300 }),
            plan({ date: "2026-01-14", offset: 7200, previous: 160 }),
        ];

        const groups = groupCalculationEntries(entries);

        expect(groups[0].previous).toBe(100);
        expect(groups[1].previous).toBe(160);
    });

    it("does not mutate the input array", () => {
        const entries = [
            plan({ date: "2026-01-07", previous: 0 }),
            entry({ date: "2026-01-08", previous: 10 }),
            entry({ date: "2026-01-09", previous: 20 }),
        ];
        const snapshot = JSON.parse(JSON.stringify(entries));

        groupCalculationEntries(entries);

        expect(entries).toEqual(snapshot);
    });
});

describe("groupCalculationMonths", () => {
    function week(start: string): CalculationGroup {
        return { start, previous: 0, entries: [] };
    }

    it("returns an empty array for empty input", () => {
        expect(groupCalculationMonths([])).toEqual([]);
    });

    it("groups consecutive weeks by month", () => {
        const groups = [
            week("2026-10-07"),
            week("2026-10-14"),
            week("2026-10-21"),
        ];

        const months = groupCalculationMonths(groups);

        expect(months).toHaveLength(1);
        expect(months[0].key).toBe("2026-10");
        expect(months[0].label).toBe("2026年10月");
        expect(months[0].groups).toEqual(groups);
    });

    it("starts a new month section when the month changes", () => {
        const groups = [
            week("2026-09-30"),
            week("2026-10-07"),
            week("2026-11-04"),
        ];

        const months = groupCalculationMonths(groups);

        expect(months.map((month) => month.key)).toEqual([
            "2026-09",
            "2026-10",
            "2026-11",
        ]);
    });

    it("labels months without a leading zero", () => {
        const months = groupCalculationMonths([
            week("2026-01-07"),
            week("2027-10-06"),
        ]);

        expect(months.map((month) => month.label)).toEqual([
            "2026年1月",
            "2027年10月",
        ]);
    });

    it("keeps the weeks of each month together", () => {
        const groups = [
            week("2026-12-23"),
            week("2026-12-30"),
            week("2027-01-06"),
        ];

        const months = groupCalculationMonths(groups);

        expect(months[0].groups).toEqual([groups[0], groups[1]]);
        expect(months[1].groups).toEqual([groups[2]]);
    });
});

describe("isoWeekNumber", () => {
    it("returns the ISO week of a weekly plan date", () => {
        expect(isoWeekNumber("2023-10-18")).toBe(42);
        expect(isoWeekNumber("2026-10-21")).toBe(43);
    });

    it("puts the week containing the first Thursday in week 1", () => {
        expect(isoWeekNumber("2026-01-01")).toBe(1);
        expect(isoWeekNumber("2026-01-04")).toBe(1);
    });

    it("rolls over to week 2 on the following Monday", () => {
        expect(isoWeekNumber("2026-01-05")).toBe(2);
    });

    it("uses the ISO week-year at a year boundary", () => {
        expect(isoWeekNumber("2024-12-30")).toBe(1);
        expect(isoWeekNumber("2025-01-01")).toBe(1);
    });
});
