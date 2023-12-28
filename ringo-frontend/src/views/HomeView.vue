<script setup lang="ts">
import BingoBoard from "@/components/BingoBoard.vue";
</script>

<template>
   <main>
      <div class="fd">dsd</div>
      <div class="board">
         <BingoBoard />
      </div>
      <div class="fd last" id="last">dsd</div>
   </main>
</template>

<style scoped lang="scss">
main {
   display: flex;
   flex-direction: row;
   width: 100vw;

   .board {
      width: 60vw;
      margin-left: auto;
      margin-right: auto;
   }

   .fd {
      width: 20vw;
      background-color: green;
   }

   .last {
      background-color: red;
   }
}
</style>
