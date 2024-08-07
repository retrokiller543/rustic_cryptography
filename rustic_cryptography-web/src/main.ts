import './assets/main.sass'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import Modal from './components/common/Modal.vue';
import IconButton from './components/common/IconButton.vue';
import CloseIcon from './assets/icons/CloseIcon.vue';
import App from './App.vue'
import router from './router'

const app = createApp(App)

app.component("Modal", Modal)
app.component("IconButton", IconButton)
app.component("CloseIcon", CloseIcon)
app.use(createPinia())
app.use(router)

app.mount('#app')
