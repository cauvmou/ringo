<script setup lang="ts">
import { RouterView } from "vue-router";
import {
   NLayout,
   NLayoutContent,
   NLayoutHeader,
   NMessageProvider,
   NConfigProvider,
} from "naive-ui";
import Header from "@/components/Header.vue";
import { useThemeStore } from "@/stores/theme";
import { ref } from "vue";

const themeStore = useThemeStore();

const inverted = ref(true);
</script>

<template>
   <div class="app-container">
      <n-config-provider :theme="themeStore.theme">
         <n-message-provider>
            <n-layout>
               <n-layout-header :inverted="inverted">
                  <Header />
               </n-layout-header>
               <div class="view">
                  <n-layout-content>
                     <RouterView />
                  </n-layout-content>
               </div>
            </n-layout>
         </n-message-provider>
      </n-config-provider>
   </div>
</template>

<style lang="scss" scoped>
.app-container {
   height: 100vh;
   overflow-y: hidden;
   overflow-x: hidden;

   .view {
      height: 100vh;
   }
}
</style>
