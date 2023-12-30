<script setup lang="ts">
import BingoBoard from "@/components/BingoBoard.vue";
import { NH1, NText } from "naive-ui";
</script>

<template>
   <main>
      <div class="upper">
         <n-h1 class="vertc" prefix="bar" align-text>
            <n-text>
               Lorem Ipsum's room
            </n-text>
         </n-h1>
      </div>
      <div class="lower">
         <div class="fd">dsd</div>
         <div class="board">
            <BingoBoard />
         </div>
         <div class="fd last">dsd</div>
      </div>
   </main>
</template>

<style scoped lang="scss">
main {
   display: flex;
   flex-direction: column;
   gap: 4em;
   margin-top: 3em;
}

.upper {
   display: flex;
   flex-direction: row;
   margin-left: 5vw;
   position: relative;

   .vertc {
      margin: 0;
      position: absolute;
      top: 50%;
      -ms-transform: translateY(-50%);
      transform: translateY(-50%);
   }
}

.lower {
   display: flex;
   flex-direction: row;
   width: 100vw;

   .board {
      width: 50vw;
      margin-left: auto;
      margin-right: auto;
   }

   .fd {
      width: 30vw;
      margin-left: 3vw;
      margin-right: 3vw;
      background-color: green;
   }

   .last {
      background-color: red;
   }
}
</style>
