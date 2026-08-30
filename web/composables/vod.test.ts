import { describe, expect, it } from "vitest";
import { formatHMS, parseHMS } from "./vod";

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
