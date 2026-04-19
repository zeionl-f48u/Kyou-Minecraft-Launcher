<script setup lang="ts">
import BlankLayout from '@layouts/BlankLayout.vue'
import MainLayout from '@layouts/MainLayout.vue'
import SettingsLayout from '@layouts/SettingsLayout.vue'

import { darkTheme } from 'naive-ui'
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { appThemeOverrides } from '@/config/theme'

const route = useRoute()

const layoutComponent = computed(() => {
  switch (route.meta.layout) {
    case 'main':
      return MainLayout
    case 'settings':
      return SettingsLayout
    default:
      return BlankLayout
  }
})
</script>

<template>
  <n-config-provider :theme="darkTheme" :theme-overrides="appThemeOverrides">
    <RouterView v-slot="{ Component, route: currentRoute }">
      <component :is="layoutComponent">
        <component :is="Component" :key="currentRoute.fullPath" />
      </component>
    </RouterView>
  </n-config-provider>
</template>
