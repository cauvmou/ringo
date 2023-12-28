<script setup lang="ts">
import { NMenu, NSwitch } from "naive-ui";
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
   <div class="header">
      <div class="title" @click="redirect">
         <h1>Ringo</h1>
      </div>
      <n-menu mode="horizontal">
         <!--TODO-->
      </n-menu>
      <div class="align-right">
         <n-switch :value="themeStore.darkThemeActive" @update:value="themeStore.flipTheme" />
         <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24"><g :fill="themeStore.darkThemeActive ? 'none' : 'white'" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="9"></circle><circle cx="12" cy="10" r="3"></circle><path d="M6.168 18.849A4 4 0 0 1 10 16h4a4 4 0 0 1 3.834 2.855"></path></g></svg>
      </div>
   </div>
</template>

<style lang="scss" scoped>
.header {
  display: flex;
  flex-direction: row;
  padding: 5px 10px 5px 10px;

  .align-right {
    display: flex;
    flex-direction: row-reverse;
    margin-left: auto;

    .icon {
      height: 100%;
    }
  }

  .title {
    cursor: pointer;
  }
}
</style>