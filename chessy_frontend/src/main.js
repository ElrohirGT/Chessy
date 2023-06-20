import '@/assets/reset.css'
import '@/assets/base.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

// API REQUEST URL
export const REGISTER_URL = "http://127.0.0.1:8080/user/register"
export const GAME_URL = "http://127.0.0.1:8080/user/register"

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
