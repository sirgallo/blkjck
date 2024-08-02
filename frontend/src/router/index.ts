import { createRouter, createWebHistory } from 'vue-router';

import lobby from '@views/lobby/lobby.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'overview',
      component: lobby
    }
  ]
});

export default router;