import {createRouter, createWebHistory, createWebHashHistory} from 'vue-router'
import Layout from '@/layout/index.vue'

const routes = [
    // 登录页
    {
        path: '/login',
        name: 'Login',
        component: () => import('@/views/login/index.vue'),
        meta: {
            title: '登录'
        }
    },
    // 仪表盘
    {
        path: '/',
        component: Layout,
        redirect: '/dashboard',
        children: [
            {
                path: 'dashboard',
                name: 'Dashboard',
                component: () => import('@/views/dashboard/index.vue'),
                meta: {
                    title: '仪表盘'
                }
            },
            {
                path: 'route',
                name: 'Route',
                component: () => import('@/views/route/index.vue'),
                meta: {
                    title: '路由配置'
                }
            },
            {
                path: 'service',
                name: 'Service',
                component: () => import('@/views/service/index.vue'),
                meta: {
                    title: '服务管理'
                }
            },
            {
                path: 'plugin',
                name: 'Plugin',
                component: () => import('@/views/plugin/index.vue'),
                meta: {
                    title: '插件管理'
                }
            },
            {
                path: 'log',
                name: 'Log',
                component: () => import('@/views/log/index.vue'),
                meta: {
                    title: '网关日志'
                }
            },
            {
                path: 'apikey',
                name: 'ApiKey',
                component: () => import('@/views/apikey/index.vue'),
                meta: {
                    title: '密钥管理'
                }
            },
            {
                path: 'firewall',
                name: 'Firewall',
                component: () => import('@/views/firewall/index.vue'),
                meta: {
                    title: '防火墙'
                }
            },
            {
                path: 'model',
                name: 'Model',
                component: () => import('@/views/model/index.vue'),
                meta: {
                    title: '模型管理'
                }
            },
            {
                path: 'users',
                name: 'Users',
                component: () => import('@/views/user/index.vue'),
                meta: {
                    title: '用户管理'
                }
            },
            {
                path: 'setting',
                name: 'Setting',
                component: () => import('@/views/system/index.vue'),
                meta: {
                    title: '系统设置'
                }
            }
        ]
    }
]


const router = createRouter({
    //history: createWebHashHistory(),  // hash 模式
    history: createWebHistory(),  // history 模式
    routes: routes
})

router.beforeEach((to, from, next) => {
    // console.log(to, from)
    if (to.meta.title) {
        document.title = `${to.meta.title}`;
    }
    next()
})

export default router