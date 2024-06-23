<script setup lang="ts">
import { NDivider, NGrid, NGi } from "naive-ui";
import ContactCard from "./ContactCard.vue";
import { ccMix } from "@/composables/utils";
</script>

<template>
    <div class="text-2xl text-neutral-100 text-center mt-4">
        {{ ccMix("點擊Discord使用者名稱即可複製到剪貼簿。") }}
    </div>
    <n-divider />
    <n-grid x-gap="12" y-gap="12" cols="3" class="w-11/12" item-responsive>
        <n-gi span="3 800:1">
            <ContactCard
                name1="惡靈"
                name2="Oreki"
                :tags="['水星神', '伺服器總管', '爭議協調與排解委員']"
                discord="oreki20"
            >
                <img
                    src="@assets/images/admin_avatars/avatar_oreki20.png"
                    class="w-full h-full object-contain"
                />
            </ContactCard>
        </n-gi>
        <n-gi span="3 800:1">
            <ContactCard
                name1="香榭"
                name2="Champsing"
                :tags="['伺服器維護', '網站設計', '爭議協調與排解委員']"
                discord="champsing"
            >
                <img
                    src="@assets/images/admin_avatars/avatar_champsing.png"
                    class="w-full h-full object-contain"
                />
            </ContactCard>
        </n-gi>
        <n-gi span="3 800:1">
            <ContactCard
                name1="邏輯"
                name2="Logic"
                :tags="['摸鱼专家']"
                discord="logic_f"
            >
                <img
                    src="@assets/images/admin_avatars/avatar_logic.png"
                    class="w-full h-full object-contain"
                />
            </ContactCard>
        </n-gi>
    </n-grid>
    <n-divider class="!mb-2" />
    <iframe
        src="https://www.youtube.com/embed/YTB35De0Bs8?si=cP5rNghSy_WQLl8m" 
        title="YouTube video player" 
        frameborder="100"
        height="400"
        width="800"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
        referrerpolicy="strict-origin-when-cross-origin" 
        allowfullscreen
    >
    </iframe>
</template>
