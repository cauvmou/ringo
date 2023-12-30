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
      /*{
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue')
    }*/
   ],
});

export default router;
