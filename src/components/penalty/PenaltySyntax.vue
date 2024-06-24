<script setup lang="ts">
import {
    NCollapse,
    NCollapseItem,
    NList,
    NListItem,
    NThing,
} from "naive-ui";
</script>

<template>
    <n-collapse
        arrow-placement="right"
        style="
            --n-title-font-size: 24px;
            --n-title-text-color: rgb(11, 118, 225);
        "
    >
        <n-collapse-item title="懲罰語法" name="punish_syntax">
            <div style="overflow: auto">
                <n-list bordered>
                    <n-list-item>
                        <n-thing style="text-align: left; font-size: 18px">
                            &lt;日期&gt;: Unix Timestamp<br />
                            &lt;編號&gt;: int &lt;懲罰主文&gt;: string
                            〔詳細資料〕: additionalMetaData（執行狀態）:
                            statusMetaData
                        </n-thing>
                    </n-list-item>
                </n-list>
            </div>
        </n-collapse-item>
        <n-collapse-item title="詳細資料" name="more_information">
            <div>
                <n-list bordered>
                    <n-list-item>
                        <n-thing style="font-size: 18px">
                            🆙增加、🔁重抽、2️⃣備案、📝原主人修改n次、➕其他後來增加的條件
                        </n-thing>
                    </n-list-item>
                </n-list>
            </div>
        </n-collapse-item>
        <n-collapse-item title="完成狀態" name="punish_status">
            <div>
                <n-list bordered>
                    <n-list-item>
                        <n-thing style="font-size: 18px">
                            ✅完成、✅已抽、🏁原主人或投票給過、⏲️
                            ⚔️目前已完成進度
                        </n-thing>
                    </n-list-item>
                </n-list>
            </div>
        </n-collapse-item>
    </n-collapse>
</template>
