import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/pages/index.vue'

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    {
      path: '/posts/:id?',
      name: 'posts',
      components: {
        default: () => import("@/pages/posts.vue"),
        aside: () => import("@/components/aside/AsideList.vue")
      },
      meta: { requireId: true }
    },
    {
      path: '/authors/:id?',
      name: 'authors',
      components: {
        default: () => import("@/pages/authors.vue"),
        aside: () => import("@/components/aside/AsideList.vue")
      },
      meta: { requireId: true }
    },
    {
      path: '/tags/:id?',
      name: 'tags',
      components: {
        default: () => import("@/pages/tags.vue"),
        aside: () => import("@/components/aside/AsideList.vue")
      },
      meta: { requireId: true }
    },
    {
      path: '/collections/:id?',
      name: 'collections',
      components: {
        default: () => import("@/pages/collections.vue"),
        aside: () => import("@/components/aside/AsideList.vue")
      },
      meta: { requireId: true }
    },
    {
      path: '/platforms/:id?',
      name: 'platforms',
      components: {
        default: () => import("@/pages/platforms.vue"),
        aside: () => import("@/components/aside/AsideList.vue")
      },
      meta: { requireId: true }
    },
  ],
  strict: true,
})
