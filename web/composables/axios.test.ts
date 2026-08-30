import { beforeEach, describe, expect, it } from "vitest";
import api from "./axios";

// Grab the single request interceptor registered at module load.
const interceptor = (
    api.interceptors.request as unknown as {
        handlers: { fulfilled: (config: any) => any }[];
    }
).handlers[0].fulfilled;

beforeEach(() => {
    localStorage.clear();
});

describe("axios request interceptor", () => {
    it("lets whitelisted list endpoints through without a token", () => {
        const config = { url: "/api/penalty/list", headers: {} };
        const result = interceptor(config);
        expect(result.headers.Authorization).toBeUndefined();
    });

    it("lets whitelisted detail endpoints through without a token", () => {
        const config = { url: "/api/penalty/detail/123", headers: {} };
        const result = interceptor(config);
        expect(result.headers.Authorization).toBeUndefined();
    });

    it("lets whitelisted login through without a token", () => {
        const config = { url: "/api/auth/login", headers: {} };
        const result = interceptor(config);
        expect(result.headers.Authorization).toBeUndefined();
    });

    it("does not whitelist the detail update sub-path", async () => {
        const config = { url: "/api/penalty/detail/123/update", headers: {} };
        await expect(interceptor(config)).rejects.toThrow();
    });

    it("adds a Bearer token for protected endpoints", () => {
        localStorage.setItem("token", "abc123");
        const config = { url: "/api/penalty/insert", headers: {} };
        const result = interceptor(config);
        expect(result.headers.Authorization).toBe("Bearer abc123");
    });

    it("rejects protected endpoints when no token is present", async () => {
        const config = { url: "/api/penalty/insert", headers: {} };
        await expect(interceptor(config)).rejects.toThrow();
    });
});
