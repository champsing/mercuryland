<script setup lang="ts">
import { ref, computed } from "vue";
import {
  VaButton,
  VaIcon,
  VaModal,
  VaDivider,
  VaTabs,
  VaTab,
  VaChip,
  VaCard,
  VaCardTitle,
  VaCardContent,
} from "vuestic-ui";
import { statusOf } from "@/composables/penalty";

function colorOfStatus(status: string): string {
  return statusOf(status).color;
}

const statusColorSet = [
  colorOfStatus("未開始"),
  colorOfStatus("已完成"),
  colorOfStatus("勉強過"),
  colorOfStatus("進行中"),
  colorOfStatus("未生效"),
];

const showAdditionalSyntax = ref(false);
const additionalSyntaxTabs = [
  {
    title: "🆙增加",
    content: "符合以下條件而新增的懲罰會被標上此標籤。",
  },
  {
    title: "🔁重抽",
    content: "抽中「重抽」懲罰並重新抽選過的懲罰，會被標上此標籤",
  },
  {
    title: "2️⃣備案",
    content:
      "惡靈怕自己電腦不好或其他突發狀況而抽出的備案。<br>生效時將取代原懲罰並移除此標籤，標上🆙增加。",
  },
  {
    title: "😇復活",
    content: "抽中「復活」懲罰並被復活的懲罰，會被標上此標籤",
  },
  {
    title: "📝原主人修改",
    content:
      "原主人要求修改懲罰主文的內容。<br>以「📝原主人修改n次」的方式表示，n為修改次數。",
  },
  {
    title: "➕其他",
    content: "其他後來增加的條件",
  },
];
const addtionalTabValue = ref(additionalSyntaxTabs[0].title);

const showStatusSyntax = ref(false);
const statusSyntaxTabs = [
  {
    title: "四大完成狀態",
    content: "",
  },
  {
    title: "未生效",
    content:
      "多抽出預備的懲罰，在沒被加新懲罰之前不會生效；<br>加懲罰時依時間順序優先成為懲罰。",
  },
  {
    title: "✅已抽",
    content: "該懲罰中所產生的內容已經抽出",
  },
  {
    title: "🏁給過",
    content:
      "「勉強過」成立的原因，例如原主人放過惡靈一馬，<br>或群組投票通過惡靈可以勉強通過懲罰。<br>不清楚原主人時則由群組投票，<br>若之後惡靈補完成全部的條件，則一樣能夠獲得「已完成」。",
  },
  {
    title: "⏲️⚔️目前進度",
    content: "「進行中」懲罰目前的進度",
  },
];
const statusTabValue = ref(statusSyntaxTabs[0].title);

const showExtraConditionDesc = ref(false);

const currentTab = (
  tab: string,
  syntaxTabList: { title: string; content: string }[],
) => computed(() => syntaxTabList.find(({ title }) => title === tab));
</script>

<template>
  <!-- !text-[#6d8581] !text-[#b91c1c] !text-[#4d7c0f] !text-[#047857] !text-[#b45309] -->
  <!-- TAILWIND CSS: DO NOT REMOVE ABOVE COMMENT -->
  <div>
    <VaCard style="--va-card-padding: 1rem" class="rounded-xl">
      <VaCardTitle
        class="px-6 pt-6 text-lg font-medium text-zinc-200"
        style="font-size: 20px; justify-content: center"
      >
        圖例
      </VaCardTitle>
      <VaCardContent>
        <div class="flex justify-center gap-4">
          <VaButton
            class="w-full"
            gradient
            color="#28c9c7"
            @click="showAdditionalSyntax = !showAdditionalSyntax"
          >
            <div class="text-xl">詳細資料類</div>
          </VaButton>
          <VaButton
            class="w-full"
            gradient
            color="#005c99"
            @click="showStatusSyntax = !showStatusSyntax"
          >
            <div class="text-xl">完成狀態類</div>
          </VaButton>
        </div>
      </VaCardContent>
    </VaCard>

    <VaModal
      v-model="showAdditionalSyntax"
      hide-default-actions
      close-button
      size="medium"
    >
      <template #header>
        <div class="text-xl mb-4 font-bold">懲罰圖例：詳細資料類</div>
      </template>

      <VaTabs v-model="addtionalTabValue" color="info" vertical grow>
        <template #tabs>
          <VaTab
            v-for="tab in additionalSyntaxTabs"
            :key="tab.title"
            :name="tab.title"
            @click="currentTab(tab.title, additionalSyntaxTabs)"
          >
            {{ tab.title }}
          </VaTab>
        </template>

        <div class="text-xl font-bold">
          {{ currentTab(addtionalTabValue, additionalSyntaxTabs).value.title }}
        </div>
        <VaDivider />
        <div
          v-html="
            currentTab(addtionalTabValue, additionalSyntaxTabs).value.content
          "
          class="text-xl mt-2"
        />

        <!-- 如果現在標籤頁是🆙增加 -->
        <div class="text-base mt-2" v-if="addtionalTabValue === '🆙增加'">
          <ol>
            <li>抽中「懲罰 + X」轉化的新懲罰</li>
            <li>由2️⃣備案轉化而成的懲罰</li>
            <li>由具有轉換條件*的懲罰轉化的額外懲罰</li>
          </ol>

          <VaButton
            preset="plain"
            color="warning"
            @click="showExtraConditionDesc = !showExtraConditionDesc"
          >
            <div class="mt-2 text-sm text-left">
              <VaIcon name="help_outline" />
              無法完成的懲罰與轉換條件
            </div>
            <VaModal
              v-model:model-value="showExtraConditionDesc"
              hide-default-actions
              close-button
            >
              <div class="text-2xl mb-2">無法完成的懲罰與轉換條件說明</div>
              某些懲罰的主文有可能存在會造成該懲罰永遠無法完成的條件，例如時間限制等；雖然這類懲罰通常會在進入轉盤之前被過濾，但難保完全沒有。而這類懲罰在該條件無法被滿足之後會被判定失敗，永遠無法獲得
              <div :class="`inline !text-[${statusColorSet[1]}]`">▲已完成</div>
              狀態，僅可獲得
              <div :class="`inline !text-[${statusColorSet[2]}]`">▲勉強過</div>
              。
              除非該懲罰帶有「否則」、「失敗」等語，即視為擁有轉換條件，當前項條件無法被滿足時，會轉換為後面所說的懲罰。
              <br />
              <br />
              假設今有一個懲罰：「2024年結束以前打完星鐵主線」；如主文所示，在2024年結束之後便永遠無法達成前項條件，此時將會判定為懲罰失敗，並將永遠無法獲得
              <div :class="`inline !text-[${statusColorSet[1]}]`">▲已完成</div>
              ；唯一的完成方式是懲罰原主人或投票給過，即以
              <div :class="`inline !text-[${statusColorSet[2]}]`">▲勉強過</div>
              方式完成。
              <br />
              但是，若懲罰為「2024年結束以前打完星鐵主線，否則懲罰+2」，就會在判定失敗時將原懲罰取代為2個新的未抽出懲罰，並於之後由直播時抽出，原懲罰便轉化為2個新的懲罰了。
            </VaModal>
          </VaButton>
        </div>
      </VaTabs>
    </VaModal>

    <VaModal
      v-model="showStatusSyntax"
      hide-default-actions
      close-button
      size="medium"
    >
      <template #header>
        <div class="text-xl mb-4 font-bold">懲罰圖例：完成狀態類</div>
      </template>

      <VaTabs v-model="statusTabValue" color="info" vertical grow>
        <template #tabs>
          <VaTab
            v-for="tab in statusSyntaxTabs"
            :key="tab.title"
            :name="tab.title"
            @click="currentTab(tab.title, statusSyntaxTabs)"
          >
            <div v-if="tab.title === '四大完成狀態'">
              <div
                v-for="color in statusColorSet.slice(0, 4)"
                :class="`inline !text-[${color}]`"
              >
                ▲
              </div>
              四大完成狀態
            </div>
            <div v-if="tab.title === '未生效'">
              <div :class="`inline !text-[${statusColorSet[4]}]`">▲</div>
              未生效
            </div>
            <div v-if="tab.title != '未生效' && tab.title != '四大完成狀態'">
              {{ tab.title }}
            </div>
          </VaTab>
        </template>

        <div class="text-xl font-bold mt-2">
          <!-- 四大完成狀態的獨立顯示 -->
          <div v-if="statusTabValue === '四大完成狀態'">
            <div
              v-for="color in statusColorSet.slice(0, 4)"
              :class="`inline !text-[${color}]`"
            >
              ▲
            </div>
            四大完成狀態
          </div>
          <!-- 未生效的獨立顯示 -->
          <div v-if="statusTabValue === '未生效'">
            <div :class="`inline !text-[${statusColorSet[4]}]`">▲</div>
            未生效
          </div>
          <div
            v-if="
              statusTabValue != '未生效' && statusTabValue != '四大完成狀態'
            "
          >
            {{ currentTab(statusTabValue, statusSyntaxTabs).value.title }}
          </div>
        </div>
        <VaDivider />
        <div
          v-html="currentTab(statusTabValue, statusSyntaxTabs).value.content"
          class="text-xl mt-2"
        />
        <!-- 四大完成狀態的獨立顯示 -->
        <div class="text-base" v-if="statusTabValue === '四大完成狀態'">
          <VaChip outline readonly :color="`${statusColorSet[0]}`" class="mr-2">
            <div :class="`inline !text-[${statusColorSet[0]}] font-bold`">
              ● 未開始
            </div>
          </VaChip>
          尚未開始嘗試完成該懲罰，沒有進度
          <div class="mt-2" />
          <VaChip outline readonly :color="`${statusColorSet[1]}`" class="mr-2">
            <div :class="`inline !text-[${statusColorSet[1]}] font-bold`">
              ● 已完成
            </div>
          </VaChip>
          已經完成該懲罰主文要求的全部條件
          <div class="mt-2" />
          <VaChip outline readonly :color="`${statusColorSet[2]}`" class="mr-2">
            <div :class="`inline !text-[${statusColorSet[2]}] font-bold`">
              ● 勉強過
            </div>
          </VaChip>
          該懲罰的原主人決定讓惡靈在沒有完成全部條件情況下，<br />以最低及格線通過該懲罰。詳情見<a
            class="underline text-orange-400"
            @click="statusTabValue = '🏁給過'"
            >「🏁給過」</a
          >
          <div class="mt-2" />
          <VaChip outline readonly :color="`${statusColorSet[3]}`" class="mr-2">
            <div :class="`inline !text-[${statusColorSet[3]}] font-bold`">
              ● 進行中
            </div>
          </VaChip>
          正在嘗試完成，已經有進度的懲罰
        </div>
        <!-- 四大完成狀態的獨立顯示 -->
      </VaTabs>
    </VaModal>
  </div>
</template>
