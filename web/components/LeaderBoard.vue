<script setup lang="ts">
import { ref } from 'vue';
import axios from 'axios';
import { BASE_URL } from '@/composables/utils';

const leaderboard = ref(null);

axios.get(BASE_URL + "/api/leaderboard").then((response) => {
    leaderboard.value = response;
    console.log(response);
});
</script>

<template>
{{ leaderboard }}
</template>