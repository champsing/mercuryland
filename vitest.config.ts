import { fileURLToPath, URL } from "url";
import { defineConfig } from "vitest/config";

// Vitest reuses the same path aliases as the Vite app build so that
// `@/…`, `@assets/…` and `@composables/…` imports resolve identically in tests.
export default defineConfig({
    resolve: {
        alias: {
            "@": fileURLToPath(new URL("./web", import.meta.url)),
            "@assets": fileURLToPath(new URL("./web/assets", import.meta.url)),
            "@styles": fileURLToPath(new URL("./styles", import.meta.url)),
            "@components": fileURLToPath(
                new URL("./web/components", import.meta.url),
            ),
            "@composables": fileURLToPath(
                new URL("./web/composables", import.meta.url),
            ),
        },
    },
    test: {
        environment: "jsdom",
        setupFiles: ["./web/test/setup.ts"],
        include: ["web/**/*.test.ts"],
    },
});
