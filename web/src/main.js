if (window.__POWERED_BY_WUJIE__) {
    // eslint-disable-next-line
    window.__webpack_public_path__ = window.__WUJIE_PUBLIC_PATH__;
}


import {createApp} from 'vue'
import App from './App.vue'
import store from './store'
import router from './router'
import {R} from './utils/R.js'
import {U} from './utils/util.js'

//Element Plus
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'

//qs
import qs from 'qs'

//svg图标
import 'virtual:svg-icons-register';
import SvgIcon from '@/components/SvgIcon/index.vue'
import PubSub from 'pubsub-js'

//全局样式
import '@/styles/index.scss'


const app = createApp(App);

app
    .use(store)
    .use(router)
    .use(ElementPlus, {locale: zhCn,})
    .use(PubSub)
    .component('svg-icon', SvgIcon)

app.mount('#app')

app.config.globalProperties.R = R;
app.config.globalProperties.U = U;
app.config.globalProperties.$qs = qs;

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}


// 解决报错：You must define a function MonacoEnvironment.getWorkerUrl or MonacoEnvironment.getWorker
if (!window['MonacoEnvironment']) {
    window['MonacoEnvironment'] = {
        getWorkerUrl: function (moduleId, label) {
            if (label === 'json') {
                return './node_modules/monaco-editor/esm/vs/language/json/json.worker.js'
            }
            return './node_modules/monaco-editor/esm/vs/editor/editor.worker.js'
        }
    }
}
