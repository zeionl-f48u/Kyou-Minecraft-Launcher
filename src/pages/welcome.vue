<route lang="json5">
{
  name: 'Welcome',
  meta: {
    layout: 'main',
    isTab: true,
    tabsName: '欢迎',
    tabOrder: 0,
  },
}
</route>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()
</script>

<template>
  <div class="flex flex-col h-full items-center justify-center">
    <div class="flex gap-1 items-center justify-center">
      <div class="i-minecraft-dirt-block text-5xl" />
      <div class="text-4xl font-500">
        Kyou Minecraft Launcher
      </div>
    </div>
    <div class="text-stone-300 mt-10">
      Vue3 + Tauri 2 + Naive UI
    </div>
    <div class="text-stone-300 mt-3">
      现代 Minecraft 启动器
    </div>
    <n-button round class="mt-10" icon-placement="right" type="primary" @click="router.push('/home')">
      <template #icon>
        <div class="i-mdi-menu-right" />
      </template>
      开始游戏
    </n-button>
  </div>
</template>
