import { describe, expect, it, beforeEach } from "vitest";
import {
    collapsedStorageKey,
    readCollapsed,
    useCollapsed,
    writeCollapsed,
} from "./collapsible";

describe("readCollapsed", () => {
    beforeEach(() => localStorage.clear());

    it("returns the fallback when nothing is stored", () => {
        expect(readCollapsed("penalty-panels")).toBe(false);
        expect(readCollapsed("penalty-panels", true)).toBe(true);
    });

    it("reads stored true/false", () => {
        localStorage.setItem(collapsedStorageKey("penalty-panels"), "true");
        expect(readCollapsed("penalty-panels")).toBe(true);

        localStorage.setItem(collapsedStorageKey("penalty-panels"), "false");
        expect(readCollapsed("penalty-panels", true)).toBe(false);
    });

    it("ignores unknown stored values", () => {
        localStorage.setItem(collapsedStorageKey("penalty-panels"), "maybe");
        expect(readCollapsed("penalty-panels", true)).toBe(true);
        expect(readCollapsed("penalty-panels")).toBe(false);
    });
});

describe("writeCollapsed", () => {
    beforeEach(() => localStorage.clear());

    it("round-trips through localStorage", () => {
        writeCollapsed("penalty-panels", true);
        expect(
            localStorage.getItem(collapsedStorageKey("penalty-panels")),
        ).toBe("true");
        expect(readCollapsed("penalty-panels")).toBe(true);
    });

    it("keeps keys independent", () => {
        writeCollapsed("penalty-panels", true);
        expect(readCollapsed("vod-panels")).toBe(false);
    });
});

describe("useCollapsed", () => {
    beforeEach(() => localStorage.clear());

    it("starts from the persisted value", () => {
        writeCollapsed("penalty-panels", true);
        expect(useCollapsed("penalty-panels").value).toBe(true);
    });

    it("starts from the fallback when nothing is persisted", () => {
        expect(useCollapsed("penalty-panels").value).toBe(false);
        expect(useCollapsed("penalty-panels", true).value).toBe(true);
    });

    it("persists changes as soon as they are assigned", () => {
        const collapsed = useCollapsed("penalty-panels");
        collapsed.value = true;
        expect(collapsed.value).toBe(true);
        expect(readCollapsed("penalty-panels")).toBe(true);

        collapsed.value = false;
        expect(collapsed.value).toBe(false);
        expect(readCollapsed("penalty-panels")).toBe(false);
    });

    it("ignores non-boolean values", () => {
        const collapsed = useCollapsed("penalty-panels");
        collapsed.value = "yes" as never;
        expect(collapsed.value).toBe(false);
        expect(readCollapsed("penalty-panels")).toBe(false);
    });
});
