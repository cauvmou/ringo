<script setup lang="ts">
import { ref } from "vue";
import { NSpace } from "naive-ui";
import BingoField from "@/components/BingoField.vue";

// temp
const fields = ref<Array<string>>(
   Array(25)
      .fill(1)
      .map((n, i) => n + i),
);
const crossedFields = ref<Array<string>>([]);
</script>

<template>
   <div class="container">
      <n-space>
         <BingoField
            v-for="field in fields"
            :key="field"
            :label="field"
            :crossed="crossedFields[field]"
            @click="crossedFields[field] = !crossedFields[field]"
         />
      </n-space>
   </div>
</template>

<style scoped>
.container {
   width: 53.5em;
   margin: auto auto;
}
</style>
