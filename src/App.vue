<script setup lang="ts">
import { NConfigProvider, darkTheme } from "naive-ui";

import HelloWorld from "./components/HelloWorld.vue";
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <HelloWorld />
    </n-config-provider>
</template>

<style scoped></style>
