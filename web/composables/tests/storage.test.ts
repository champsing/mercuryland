import { describe, expect, it, beforeEach } from "vitest";
import { readStorage, writeStorage } from "../storage";

describe("readStorage / writeStorage", () => {
    beforeEach(() => localStorage.clear());

    it("round-trips a value", () => {
        writeStorage("some-key", "some-value");
        expect(readStorage("some-key")).toBe("some-value");
    });

    it("returns null for a missing key", () => {
        expect(readStorage("missing")).toBeNull();
    });

    it("overwrites an existing value", () => {
        writeStorage("some-key", "a");
        writeStorage("some-key", "b");
        expect(readStorage("some-key")).toBe("b");
    });
});
