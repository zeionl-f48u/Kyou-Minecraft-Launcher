<script setup lang="ts">
import type { RouteRecordRaw } from 'vue-router'
import { AnimatePresence, motion, MotionConfig, useReducedMotion } from 'motion-v'
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { routes } from 'vue-router/auto-routes'

import Tabs from '@/layouts/components/Tabs.vue'

const route = useRoute()
const prefersReducedMotion = useReducedMotion()

function getTabRoutes(items: readonly RouteRecordRaw[]): RouteRecordRaw[] {
  return items.flatMap((item) => {
    const children = item.children ? getTabRoutes(item.children) : []
    return item.meta?.isTab ? [item, ...children] : children
  })
}

const routeOrderMap = new Map(
  getTabRoutes(routes).flatMap((item) => {
    if (typeof item.name !== 'string')
      return []

    const order = typeof item.meta?.tabOrder === 'number'
      ? item.meta.tabOrder
      : Number.MAX_SAFE_INTEGER

    return [[item.name, order] as const]
  }),
)

function getRouteKey() {
  return String(route.name)
}

function getRouteOrder() {
  const routeName = typeof route.name === 'string' ? route.name : ''
  return routeOrderMap.get(routeName) ?? Number.MAX_SAFE_INTEGER
}

const navigationDirection = ref(0)
const previousRouteOrder = ref(getRouteOrder())

watch(
  () => route.fullPath,
  () => {
    const nextRouteOrder = getRouteOrder()
    navigationDirection.value = nextRouteOrder === previousRouteOrder.value
      ? 0
      : nextRouteOrder > previousRouteOrder.value
        ? 1
        : -1
    previousRouteOrder.value = nextRouteOrder
  },
)

const currentMotionState = computed(() => {
  if (prefersReducedMotion.value) {
    return {
      initial: { opacity: 0.01 },
      animate: { opacity: 1 },
      exit: { opacity: 0.01 },
      transition: { duration: 0.12, ease: 'linear' as const },
    }
  }

  const direction = navigationDirection.value
  const enterY = direction === 0 ? 18 : direction > 0 ? 28 : -28
  const exitY = direction === 0 ? -14 : direction > 0 ? -24 : 24

  return {
    initial: {
      opacity: 0,
      y: enterY,
      filter: 'blur(10px)',
    },
    animate: {
      opacity: 1,
      y: 0,
      filter: 'blur(0px)',
    },
    exit: {
      opacity: 0,
      y: exitY,
      filter: 'blur(8px)',
    },
    transition: {
      duration: 0.28,
      ease: [0.22, 1, 0.36, 1] as const,
    },
  }
})
</script>

<template>
  <div class="layout-shell">
    <header class="tabs-header">
      <Tabs />
    </header>

    <MotionConfig reducedMotion="user">
      <AnimatePresence mode="wait">
        <motion.div
          :key="getRouteKey()"
          class="page-content"
          :initial="currentMotionState.initial"
          :animate="currentMotionState.animate"
          :exit="currentMotionState.exit"
          :transition="currentMotionState.transition"
        >
          <slot />
        </motion.div>
      </AnimatePresence>
    </MotionConfig>
  </div>
</template>

<style scoped>
.layout-shell {
  --uno: flex flex-col h-screen;
}

.tabs-header {
  --uno: flex-none;
}

.page-content {
  --uno: flex-1 min-h-0;
}
</style>
