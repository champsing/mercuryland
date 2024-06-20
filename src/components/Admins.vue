<script setup lang="ts">
import { NGrid, NGi } from "naive-ui";
import ContactCard from "./contact/ContactCard.vue";
</script>

<template>
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
</template>
