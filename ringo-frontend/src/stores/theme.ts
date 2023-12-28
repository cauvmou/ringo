import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { darkTheme, type GlobalTheme } from "naive-ui";

export const useThemeStore = defineStore("theme", () => {
   const darkThemeActive = ref(true);

   const theme = computed<GlobalTheme | null>(() => (darkThemeActive.value ? darkTheme : null));

   function flipTheme() {
      darkThemeActive.value = !darkThemeActive.value;
   }

   return { darkThemeActive, theme, flipTheme };
});
