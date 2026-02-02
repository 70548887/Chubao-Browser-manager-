/**
 * @description 路由配置
 * @author DeepAgent
 */
import { createRouter, createWebHistory } from 'vue-router'

// 认证相关页面
const LoginView = () => import('@/features/auth/LoginView.vue')
const RegisterView = () => import('@/features/auth/RegisterView.vue')

// 主应用页面
const MainApp = () => import('@/views/MainApp.vue')

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: LoginView,
      meta: { requiresAuth: false }
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterView,
      meta: { requiresAuth: false }
    },
    {
      path: '/',
      name: 'main',
      component: MainApp,
      meta: { requiresAuth: true }
    },
    {
      path: '/:pathMatch(.*)*',
      redirect: '/'
    }
  ]
})

// 路由守卫
router.beforeEach((to, _from, next) => {
  // TODO: 后台对接后启用认证检查
  // const isAuthenticated = localStorage.getItem('auth_token')
  // 
  // if (to.meta.requiresAuth && !isAuthenticated) {
  //   next({ name: 'login' })
  // } else if (!to.meta.requiresAuth && isAuthenticated && (to.name === 'login' || to.name === 'register')) {
  //   next({ name: 'main' })
  // } else {
  //   next()
  // }
  
  // 暂时关闭登录验证，直接放行
  next()
})

export default router
