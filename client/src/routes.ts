import { lazy } from 'solid-js';
import type { RouteDefinition } from '@solidjs/router';

import Home from './pages/home';
export const routes: RouteDefinition[] = [
  {
    path: '/',
    component: Home,
  },
  {
    path: '/forms',
    component: lazy(() => import('./pages/forms')),
  },
  {
    path: '**',
    component: lazy(() => import('./errors/404')),
  },
  {
    path: '/login',
    component: lazy(() => import('./pages/login'))
  }
];
