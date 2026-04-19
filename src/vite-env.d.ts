/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module 'vue-router/auto-routes' {
  import type { RouteRecordRaw } from 'vue-router'
  export const routes: readonly RouteRecordRaw[]
  export function handleHotUpdate(router: any): void
}
