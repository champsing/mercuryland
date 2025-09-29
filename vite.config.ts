import { resolve } from "path";
import { defineConfig } from "vite";
import { fileURLToPath, URL } from "url";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue()],
    css: {
        preprocessorOptions: {
            scss: {
                api: "modern-compiler",
            },
        },
    },
    build: {
        rollupOptions: {
            input: {
                index: resolve(__dirname, "index.html"),
            },
            output: {
                manualChunks(id) {
                    if (id.indexOf("node_modules") != -1) {
                        return id
                            .toString()
                            .split("node_modules/")[1]
                            .split("/")[0]
                            .toString();
                    }
                },
            },
        },
    },
    resolve: {
        alias: {
            "@": fileURLToPath(new URL("./web", import.meta.url)),
            "@assets": fileURLToPath(new URL("./web/assets", import.meta.url)),
            "@components": fileURLToPath(
                new URL("./web/components", import.meta.url)
            ),
            "@composables": fileURLToPath(
                new URL("./web/composables", import.meta.url)
            ),
        },
    },
    esbuild: {
        supported: {
            "top-level-await": true, //browsers can handle top-level-await features
        },
    },
});
