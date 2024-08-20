import { createRouter, createWebHashHistory } from 'vue-router'
import LoginView from '../views/LoginView.vue'

const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'login',
            component: LoginView
        },
        {
            path: '/lobby',
            name: 'lobby',
            component: () => import('../views/LobbyView.vue')
        },
        {
            path: '/game',
            name: 'game',
            component: () => import('../views/GameView.vue')
        },
        {
            path: '/:pathMatch(.*)*',
            name: 'missing',
            component: () => import('../views/MissingView.vue')
        }
    ]
})

export default router
