import { afterEach, describe, expect, it, vi } from "vitest";
import {
    BASE_URL,
    backToTop,
    copyToClipboard,
    formatDate,
    openLinks,
    parseDate,
} from "./utils";

afterEach(() => {
    vi.restoreAllMocks();
});

describe("formatDate", () => {
    it("zero-pads single-digit months and days", () => {
        expect(formatDate(new Date(2025, 0, 5))).toBe("2025-01-05");
    });

    it("does not pad double-digit months", () => {
        expect(formatDate(new Date(2025, 9, 5))).toBe("2025-10-05");
    });

    it("does not pad double-digit days", () => {
        expect(formatDate(new Date(2025, 0, 15))).toBe("2025-01-15");
    });

    it("formats a full date", () => {
        expect(formatDate(new Date(2024, 11, 15))).toBe("2024-12-15");
    });
});

describe("parseDate", () => {
    it("parses a YYYY-MM-DD string into a local date", () => {
        expect(parseDate("2025-10-05").getTime()).toBe(
            new Date(2025, 9, 5).getTime(),
        );
    });

    it("parses a January date", () => {
        expect(parseDate("2025-01-01").getTime()).toBe(
            new Date(2025, 0, 1).getTime(),
        );
    });
});

describe("openLinks", () => {
    it("opens every link in a new tab with noopener/noreferrer", () => {
        const open = vi.fn().mockReturnValue(null);
        vi.spyOn(window, "open").mockImplementation(
            open as unknown as typeof window.open,
        );

        openLinks(["https://a", "https://b", "https://c"]);

        expect(open).toHaveBeenCalledTimes(3);
        expect(open).toHaveBeenNthCalledWith(
            1,
            "https://a",
            "_blank",
            "noopener noreferrer",
        );
        expect(open).toHaveBeenNthCalledWith(
            3,
            "https://c",
            "_blank",
            "noopener noreferrer",
        );
    });

    it("does nothing for an empty list", () => {
        const open = vi.fn();
        vi.spyOn(window, "open").mockImplementation(
            open as unknown as typeof window.open,
        );

        openLinks([]);
        expect(open).not.toHaveBeenCalled();
    });
});

describe("backToTop", () => {
    it("scrolls the window when no snap container exists", () => {
        const scrollTo = vi.fn();
        vi.spyOn(window, "scrollTo").mockImplementation(scrollTo as never);

        backToTop();
        expect(scrollTo).toHaveBeenCalledWith({ top: 0, behavior: "smooth" });
    });

    it("scrolls the snap container when it exists", () => {
        const scrollTo = vi.fn();
        vi.spyOn(document, "querySelector").mockReturnValue({
            scrollTo,
        } as unknown as Element);

        backToTop();
        expect(scrollTo).toHaveBeenCalledWith({ top: 0, behavior: "smooth" });
    });
});

describe("copyToClipboard", () => {
    it("writes text to the clipboard", async () => {
        const writeText = vi.fn().mockResolvedValue(undefined);
        Object.defineProperty(navigator, "clipboard", {
            value: { writeText },
            configurable: true,
        });

        await copyToClipboard("hello");
        expect(writeText).toHaveBeenCalledWith("hello");
    });

    it("logs an error instead of throwing when the clipboard is unavailable", async () => {
        const writeText = vi.fn().mockRejectedValue(new Error("denied"));
        Object.defineProperty(navigator, "clipboard", {
            value: { writeText },
            configurable: true,
        });
        const errorSpy = vi
            .spyOn(console, "error")
            .mockImplementation(() => {});

        await expect(copyToClipboard("hello")).resolves.toBeUndefined();
        expect(errorSpy).toHaveBeenCalled();
    });
});

describe("BASE_URL", () => {
    it("is a non-empty string", () => {
        expect(typeof BASE_URL).toBe("string");
        expect(BASE_URL.length).toBeGreaterThan(0);
    });
});
