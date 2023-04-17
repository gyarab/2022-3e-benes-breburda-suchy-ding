import { createApp } from 'vue'

// base styles & tailwind css
import "./style.css"

import router from './router.js'
import App from './App.vue'
import { loadUserData } from './store'

const app = createApp(App)

app.use(router)
app.mount('#app')

loadUserData();