<script setup lang="ts">
document.title = "聯絡我們 - 水星人的夢幻樂園";

import { VaButton, VaDivider } from "vuestic-ui";
import ContactCard from "./ContactCard.vue";
</script>

<template>
    <main
        class="contact-page min-h-[calc(100vh-48px)] text-[#f7f7f8] pt-10 pb-5 px-4 max-md:pt-16 max-md:pb-4 max-md:px-3"
    >
        <div class="page-content">
            <h1 class="mb-6 text-3xl font-bold text-neutral-100">聯絡我們</h1>
            <div class="flex flex-col md:flex-row justify-center gap-6">
                <ContactCard
                    name1="惡靈"
                    name2="Oreki"
                    :tags="['水星神', '伺服器總管', '爭議協調與排解委員']"
                    discord="oreki20"
                >
                    <template #avatar>
                        <img
                            src="/images/admin_avatars/oreki20.webp"
                            alt="惡靈 Oreki 的頭像"
                            class="w-full h-full object-contain"
                            slot="avatar"
                        />
                    </template>
                </ContactCard>
                <ContactCard
                    name1="香榭"
                    name2="Champsing"
                    :tags="['伺服器維護', '網站設計', '爭議協調與排解委員']"
                    discord="champsing"
                >
                    <template #avatar>
                        <img
                            src="/images/admin_avatars/champsing.webp"
                            alt="香榭 Champsing 的頭像"
                            class="w-full h-full object-contain"
                            slot="avatar"
                        />
                    </template>
                    <template #rcm-link>
                        <VaButton
                            preset="primary"
                            color="warning"
                            href="https://tslyric.com/"
                            rel="noopener noreferrer"
                            target="_blank"
                        >
                            <div class="text-base text-orange-300">
                                🎵音樂欣賞
                            </div>
                        </VaButton>
                    </template>
                </ContactCard>
            </div>
            <VaDivider class="!mb-2" />
            <div class="w-full mb-6">
                <div
                    class="mx-auto w-full max-w-[800px] overflow-hidden rounded-xl border border-white/10 bg-zinc-900"
                >
                    <iframe
                        class="block w-full aspect-video"
                        src="https://www.youtube.com/embed/YTB35De0Bs8?si=cP5rNghSy_WQLl8m"
                        title="YouTube video player"
                        frameborder="0"
                        loading="lazy"
                        allow="
                            accelerometer;
                            autoplay;
                            clipboard-write;
                            encrypted-media;
                            gyroscope;
                            picture-in-picture;
                            web-share;
                        "
                        referrerpolicy="strict-origin-when-cross-origin"
                        allowfullscreen
                    ></iframe>
                </div>
                <p class="mt-2 text-center text-xs text-zinc-500">
                    影片無法載入？
                    <a
                        class="text-sky-300 underline underline-offset-2 hover:text-sky-200"
                        href="https://www.youtube.com/watch?v=YTB35De0Bs8"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        在 YouTube 開啟
                    </a>
                </p>
            </div>
        </div>
    </main>
</template>

<style>
.contact-page {
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
