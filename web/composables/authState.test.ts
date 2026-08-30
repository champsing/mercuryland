import { describe, expect, it } from "vitest";
import { useAuthState } from "./authState";

describe("useAuthState", () => {
    it("returns the same reactive singleton on every call", () => {
        const a = useAuthState();
        const b = useAuthState();

        expect(a).toBe(b);
    });

    it("starts unauthenticated", () => {
        expect(useAuthState().isAuthenticated).toBe(false);
    });

    it("shares mutations across callers", () => {
        const a = useAuthState();
        a.isAuthenticated = true;
        expect(useAuthState().isAuthenticated).toBe(true);

        // Reset so this test does not leak state into others.
        a.isAuthenticated = false;
    });
});
