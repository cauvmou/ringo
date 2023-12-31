<script setup lang="ts">
import BingoBoard from "@/components/BingoBoard.vue";
import { NH1, NText } from "naive-ui";
import PlayerList from "@/components/PlayerList.vue";
import {useGameStore} from "@/stores/game";

const testPlayers = ["Max Mustermann", "Davideko", "Cauvmou"];
</script>

<template>
   <div class="flex flex-col gap-4 mt-12">
      <div class="upper flex flex-row ml-[5vw] relative">
         <n-h1 class="vertc m-0 absolute top-1/2 translate" prefix="bar" align-text>
            <n-text> Lorem Ipsum's room </n-text>
         </n-h1>
      </div>
      <div class="flex flex-row w-screen h-[42rem]">
         <PlayerList :players="testPlayers" />
         <div class="w-1/2 ml-auto mr-auto">
            <BingoBoard />
         </div>
         <div class="w-1/3 ml-[3vw] mr-[3vw] bg-red-800">dsd</div>
      </div>
   </div>
</template>

<style scoped lang="scss">
.upper {
   .vertc {
      -ms-transform: translateY(-50%);
      transform: translateY(-50%);
   }
}
</style>
