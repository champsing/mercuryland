import { describe, expect, it } from "vitest";
import {
    availableHeight,
    FOOTNOTE_HEIGHT,
    MIN_CONTENT_HEIGHT,
} from "../viewportHeight";

describe("availableHeight", () => {
    it("fills the space between the wrapper and the footer", () => {
        expect(
            availableHeight({ viewportHeight: 1000, scrollY: 0, top: 200 }),
        ).toBe(1000 - 200 - FOOTNOTE_HEIGHT);
    });

    it("accounts for the current scroll position", () => {
        const scrolled = availableHeight({
            viewportHeight: 1000,
            scrollY: 120,
            top: 80,
        });

        expect(scrolled).toBe(1000 - 120 - 80 - FOOTNOTE_HEIGHT);
    });

    it("stays the same while scrolling, because `top` is viewport relative", () => {
        const atTop = availableHeight({
            viewportHeight: 900,
            scrollY: 0,
            top: 300,
        });
        const scrolled = availableHeight({
            viewportHeight: 900,
            scrollY: 150,
            top: 150,
        });

        expect(scrolled).toBe(atTop);
    });

    it("clamps to the minimum when the page above is taller than the viewport", () => {
        expect(
            availableHeight({ viewportHeight: 667, scrollY: 0, top: 830 }),
        ).toBe(MIN_CONTENT_HEIGHT);
    });

    it("respects a custom minimum", () => {
        expect(
            availableHeight({
                viewportHeight: 500,
                scrollY: 0,
                top: 600,
                minHeight: 200,
            }),
        ).toBe(200);
    });

    it("respects a custom footer size", () => {
        expect(
            availableHeight({
                viewportHeight: 800,
                scrollY: 0,
                top: 100,
                footnoteHeight: 0,
            }),
        ).toBe(700);
    });

    it("rounds fractional results", () => {
        expect(
            availableHeight({
                viewportHeight: 800.4,
                scrollY: 0.2,
                top: 100,
                footnoteHeight: 0,
            }),
        ).toBe(700);
    });

    it("adds `extraHeight` back to the result", () => {
        const base = availableHeight({
            viewportHeight: 1000,
            scrollY: 0,
            top: 200,
        });
        const withExtra = availableHeight({
            viewportHeight: 1000,
            scrollY: 0,
            top: 200,
            extraHeight: 150,
        });

        expect(withExtra).toBe(base + 150);
    });

    it("keeps the area stable when a collapsible panel above is opened", () => {
        const collapsed = availableHeight({
            viewportHeight: 1000,
            scrollY: 0,
            top: 250,
        });
        const expanded = availableHeight({
            viewportHeight: 1000,
            scrollY: 0,
            top: 400,
            extraHeight: 150,
        });

        expect(expanded).toBe(collapsed);
    });

    it("still clamps to the minimum when `extraHeight` is supplied", () => {
        expect(
            availableHeight({
                viewportHeight: 667,
                scrollY: 0,
                top: 830,
                extraHeight: 20,
            }),
        ).toBe(MIN_CONTENT_HEIGHT);
    });
});
