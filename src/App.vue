<script setup lang="ts">
import { NConfigProvider, darkTheme } from "naive-ui";

import Vod from "./components/Vod.vue";
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <Vod />
    </n-config-provider>
</template>

<style scoped></style>
