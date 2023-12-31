<script setup lang="ts">
import { NDivider, NScrollbar } from "naive-ui";

defineProps<{
    players: string[]; // TODO: prototype much
}>();
</script>

<template>
    <div class="flex flex-col w-1/3 ml-[3vw] mr-[3vw]">
        <h1 class="text-3xl">Players ({{players.length}}):</h1>
        <n-scrollbar class="overflow-y-hidden">
            <n-divider />
            <div v-for="player in players" :key="player" class="font-bold">
                {{player}}
                <n-divider />
            </div>
        </n-scrollbar>
    </div>
</template>

<style scoped lang="scss">

</style>