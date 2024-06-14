<script setup lang="ts">
import { computed } from "vue";
import {
    NButton,
    NTable,
    NDivider,
} from "naive-ui";
import { openLink } from "@composables/utils.ts";
import vodLinkData from "@assets/data/vod.json";

const props = defineProps({
    dateMin: Number,
    dateMax: Number,
    tagOption: String,
});

let data = computed(() => {
    return vodLinkData
        .filter(
            (v) =>
                v.date >=
                    new Date(props.dateMin).toISOString().slice(0, 10) &&
                v.date <=
                    new Date(props.dateMax).toISOString().slice(0, 10)
        )
        .filter(
            (v) => props.tagOption == null || v.tags.includes(props.tagOption)
        )
        .sort((lhs, rhs) => rhs.date.localeCompare(lhs.date));
});
</script>

<template>
    <n-table :bordered="true" size="small" class="text-center">
        <thead>
            <tr>
                <td class="font-bold">日期</td>
                <td class="font-bold">直播連結</td>
                <td class="font-bold">TAG</td>
                <td class="font-bold">直播时数</td>
            </tr>
        </thead>

        <tbody>
            <tr v-for="item in data">
                <td>
                    {{ item.date }}
                </td>
                <td>
                    <n-button
                        @click="openLink(item.link)"
                        :text="true"
                        :focusable="false"
                    >
                        {{ item.title }}
                    </n-button>
                </td>
                <td>
                    <template v-for="(tag, index) in item.tags">
                        <template v-if="index > 0">
                            <n-divider vertical
                        /></template>
                        <span
                            ><n-button
                                @click="$emit('update_tag', tag)"
                                :text="true"
                                :focusable="false"
                            >
                                {{ tag }}
                            </n-button>
                        </span>
                    </template>
                </td>
                <td>
                    {{ item.duration }}
                </td>
            </tr>
        </tbody>
    </n-table>
</template>
