<script setup lang="ts">
import { VaDivider } from "vuestic-ui";
import ContactCard from "./ContactCard.vue";
</script>

<template>
    <div class="mt-4 m-auto w-11/12">
        <div class="flex">
            <ContactCard
                name1="惡靈"
                name2="Oreki"
                :tags="['水星神', '伺服器總管', '爭議協調與排解委員']"
                discord="oreki20"
            >
                <img
                    src="@assets/images/admin_avatars/oreki20.png"
                    class="w-full h-full object-contain"
                />
            </ContactCard>
            <ContactCard
                name1="香榭"
                name2="Champsing"
                :tags="['伺服器維護', '網站設計', '爭議協調與排解委員']"
                discord="champsing"
            >
                <img
                    src="@assets/images/admin_avatars/champsing.png"
                    class="w-full h-full object-contain"
                />
            </ContactCard>
            <ContactCard
                name1="邏輯"
                name2="Logic"
                :tags="['摸鱼专家']"
                discord="logic_f"
            >
                <img
                    src="@assets/images/admin_avatars/logic.png"
                    class="w-full h-full object-contain"
                />
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
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                referrerpolicy="strict-origin-when-cross-origin"
                allowfullscreen
            ></iframe>
        </div>
    </div>
</template>
