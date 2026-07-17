<script setup lang="ts">
document.title = "聯絡我們 - 水星人的夢幻樂園";

import { VaButton, VaDivider } from "vuestic-ui";
import ContactCard from "./ContactCard.vue";
</script>

<template>
    <div class="mt-4 m-auto w-11/12">
        <div class="flex flex-row justify-center gap-6 pt-10">
            <ContactCard
                name1="惡靈"
                name2="Oreki"
                :tags="['水星神', '伺服器總管', '爭議協調與排解委員']"
                discord="oreki20"
            >
                <template #avatar>
                    <img
                        src="/images/admin_avatars/oreki20.webp"
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
                        <div class="text-base text-orange-300">🎵音樂欣賞</div>
                    </VaButton>
                </template>
            </ContactCard>
        </div>
        <VaDivider class="!mb-2" />
        <div class="w-full mb-6">
            <iframe
                class="m-auto"
                src="https://www.youtube.com/embed/YTB35De0Bs8?si=cP5rNghSy_WQLl8m"
                title="YouTube video player"
                frameborder="0"
                height="400"
                width="800"
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
    </div>
</template>
