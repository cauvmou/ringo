<script setup lang="ts">
import { NButton } from "naive-ui";
import { computed } from "vue";
import type { Type } from "naive-ui/es/button/src/interface";

const props = defineProps<{
   label: string;
   crossed: boolean;
}>();

const emit = defineEmits<{
   (e: "click"): void;
}>();

const type = computed<Type>(() => (props.crossed ? "primary" : "default"));
</script>

<template>
   <n-button :type="type" class="field" :on-click="() => emit('click')">
      {{ label }}
   </n-button>
</template>

<style scoped lang="scss">
.field {
   width: 10em;
   height: 10em;
}
</style>
