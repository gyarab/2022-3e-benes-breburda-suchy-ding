import { createApp } from 'vue'
import notify from './notify'

// base styles & tailwind css
import "./style.css"

import './assets/fonts.css';

import router from './router.js'
import App from './App.vue'
import { loadUserData } from './store'

const app = createApp(App)

loadUserData().finally(() => {
    app.use(router)
    app.mount('#app')
});

app.config.errorHandler = err => {
    notify.send('error', `${err}`)
}
