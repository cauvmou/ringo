import { defineStore } from "pinia";
import { ref } from "vue";
import axios from "axios";

type Board = {
   username: string,
   room_code: string,
   seed_position: number,
   seed_fields: number,
}

export const useGameStore = defineStore("game", () => {
   const loading = ref(false);

   const board = ref<Board>({} as Board);

   async function loadGame() {
      loading.value = true;

      const { data } = await axios.get(
         "http://localhost:5000/" // TODO: insert API here
      );

      // stuff

      loading.value = false;
   }

   return { board, loading, loadGame };
});