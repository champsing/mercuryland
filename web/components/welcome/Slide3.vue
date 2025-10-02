<script setup lang="ts">
import { VaDivider } from "vuestic-ui";
import NextPageButton from "./NextPageButton.vue";
</script>

<template>
  <div class="h-screen overflow-hidden bg-teal-800 relative">
    <NextPageButton :page="3" />
    <div
      class="grid grid-cols-3 grid-rows-3 gap-4 absolute x-center w-10/12 h-full"
    >
      <div class="relative row-start-1 row-end-4">
        <img
          src="/images/welcome/statue_of_liberty.webp"
          class="absolute y-center x-center h-5/6 object-contain"
          alt="Welcome"
        />
      </div>
      <div class="relative col-start-3 row-start-2 -mt-8">
        <div class="text-yellow-400 font-bold text-6xl">
          發揮您的創意，建造您夢想中的建築。
        </div>
        <VaDivider class="mt-6 mb-6" />
        <div class="text-yellow-600 font-bold text-3xl">
          絕無思想審查制度。您可以在我們的伺服器復刻世界上的有名建築，或單單只是在這個世界蓋出專屬於您的根據地。
        </div>
      </div>
    </div>
  </div>
</template>
