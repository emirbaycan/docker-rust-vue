import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

import AuthLayout from '../layouts/AuthLayout.vue'
import AppLayout from '../layouts/AppLayout.vue'

import RouteViewComponent from '../layouts/RouterBypass.vue'

import stores from '../stores'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/:pathMatch(.*)*',
    redirect: { name: 'dashboard' },
  },
  
  {
    name: 'admin',
    path: '/',
    component: AppLayout,
    redirect: { name: 'dashboard' },
    meta:{
      requiresAuth:true
    },
    children: [
      {
        name: 'dashboard',
        path: 'dashboard',
        component: () => import('../pages/admin/dashboard/Dashboard.vue'),
      },
      {
        name: 'agenda',
        path: '/agenda',
        component: () => import('../pages/agenda/AgendaPage.vue'),
      },
      {
        name: 'settings',
        path: 'settings',
        component: () => import('../pages/settings/Settings.vue'),
      },
      {
        name: 'preferences',
        path: 'preferences',
        component: () => import('../pages/preferences/Preferences.vue'),
      },
      {
        name: 'projects',
        path: 'projects',
        component: () => import('../pages/projects/ProjectsPage.vue'),
      },
      {
        name: 'testimonials',
        path: 'testimonials',
        component: () => import('../pages/testimonials/TestimonialsPage.vue'),
      },
      {
        name: 'jobs',
        path: 'jobs',
        component: () => import('../pages/jobs/JobsPage.vue'),
      },
      {
        name: 'images',
        path: 'images',
        component: () => import('../pages/images/ImagesPage.vue'),
      },
      {
        name: 'details',
        path: 'details',
        component: () => import('../pages/projects/ProjectsPage.vue'),
      },
      {
        name: 'users',
        path: '/users',
        component: RouteViewComponent,
        children: [
          {
            name: 'registered-users',
            path: 'registered-users',
            component: () => import('../pages/users/UsersPage.vue'),
          },
          {
            name: 'new-users',
            path: 'new-users',
            component: () => import('../pages/users/NewUsersPage.vue'),
          },
        ],
      },
      {
        name: 'faq',
        path: '/faq',
        component: () => import('../pages/faq/FaqPage.vue'),
      },
    ],
  },
  {
    path: '/auth',
    component: AuthLayout,
    children: [
      {
        name: 'login',
        path: 'login',
        component: () => import('../pages/auth/Login.vue'),
      },
      {
        name: 'signup',
        path: 'signup',
        component: () => import('../pages/auth/Signup.vue'),
      },
      {
        name: 'recover-password',
        path: 'recover-password',
        component: () => import('../pages/auth/RecoverPassword.vue'),
      },
      {
        name: 'recover-password-email',
        path: 'recover-password-email',
        component: () => import('../pages/auth/CheckTheEmail.vue'),
      },
      {
        path: '',
        redirect: { name: 'login' },
      },
    ],
  },
  {
    name: '404',
    path: '/404',
    component: () => import('../pages/404.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    }
    // For some reason using documentation example doesn't scroll on page navigation.
    if (to.hash) {
      return { el: to.hash, behavior: 'smooth' }
    } else {
      window.scrollTo(0, 0)
    }
  },
  routes,
})

router.beforeEach((to,from,next)=>{
  if(to.matched.some(route=>route.meta.requiresAuth)){
    if(!localStorage.getItem('id')){
      next({ name: 'login' })
    }else{
      var last_login_time = localStorage.getItem('last_login');
      if(last_login_time){
        var loginValidFor = 86400 * 1000;
        if (new Date().getTime()-parseInt(last_login_time)>=loginValidFor){
          localStorage.clear();
          next({ name: 'login' })    
        }      
      }
    }
  }
  next();
})

export default router
