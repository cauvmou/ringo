<script setup lang="ts">
import { NMenu, NSwitch, NTooltip } from "naive-ui";
import { useRoute, useRouter } from "vue-router";
import { useThemeStore } from "@/stores/theme";

const route = useRoute();
const router = useRouter();

const themeStore = useThemeStore();

function redirect() {
   if (route.path === "/") return;

   router.push("/");
}
</script>

<template>
   <div class="header flex flex-row px-3 py-2">
      <div class="title cursor-pointer" @click="redirect">
         <h1>Ringo</h1>
      </div>
      <n-menu mode="horizontal">
         <!--TODO-->
      </n-menu>
      <div class="align-right flex flex-row-reverse ml-auto">
         <n-tooltip trigger="hover">
            <template #trigger>
               <n-switch
                   :value="themeStore.darkThemeActive"
                   @update:value="themeStore.flipTheme"
               />
            </template>
            Darkmode Switch
         </n-tooltip>
      </div>
   </div>
</template>

<style lang="scss" scoped></style>
