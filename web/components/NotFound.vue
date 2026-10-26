<script setup lang="ts">
import { backToTop } from "@composables/utils";
import { VaButton } from "vuestic-ui";

document.title = "找不到頁面 - 水星人的夢幻樂園";
</script>

<template>
    <main
        class="notfound-page flex min-h-[calc(100vh-48px)] flex-col text-[#f7f7f8] pt-10 pb-5 px-4 max-md:pt-16 max-md:pb-4 max-md:px-3"
    >
        <div
            class="page-content flex flex-1 flex-col items-center justify-center text-center"
        >
            <p
                aria-hidden="true"
                class="select-none text-[clamp(6rem,22vw,11rem)] font-black leading-none tracking-tighter text-white/[0.08]"
            >
                404
            </p>
            <h1
                class="mt-2 text-3xl font-bold text-neutral-100 max-md:text-2xl"
            >
                找不到頁面
            </h1>
            <p class="mt-3 max-w-[34rem] text-base text-zinc-400">
                這個頁面不存在，可能已經被移除，或是網址打錯了。
            </p>
            <VaButton
                preset="primary"
                color="warning"
                size="large"
                class="mt-8 !normal-case"
                to="/"
                @click="backToTop()"
            >
                返回首頁
            </VaButton>
        </div>
    </main>
</template>

<style>
.notfound-page {
    background:
        radial-gradient(
            circle at 50% 0%,
            rgba(142, 120, 255, 0.09),
            transparent 36rem
        ),
        radial-gradient(
            circle at 15% 0%,
            rgba(88, 166, 255, 0.2),
            transparent 32rem
        ),
        linear-gradient(135deg, #0f1117 0%, #16191f 48%, #101318 100%);
}
</style>
