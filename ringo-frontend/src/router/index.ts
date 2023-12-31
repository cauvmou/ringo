import { createRouter, createWebHistory } from "vue-router";
import StartView from "../views/StartView.vue";
import RoomView from "../views/RoomView.vue";

const router = createRouter({
   history: createWebHistory(import.meta.env.BASE_URL),
   routes: [
      {
         path: "/",
         name: "start",
         component: StartView,
      },
      {
         path: "/room",
         name: "room",
         component: RoomView,
      },
   ],
});

export default router;
