<route lang="json5">
{
  name: 'Settings',
  meta: {
    layout: 'settings',
  },
}
</route>

<script setup lang="ts">
import type { CloseBehavior } from '@/utils/desktop-preferences'
import { disable as disableAutostart, enable as enableAutostart, isEnabled as isAutostartEnabled } from '@tauri-apps/plugin-autostart'
import { openUrl } from '@tauri-apps/plugin-opener'

import { computed, onMounted, ref } from 'vue'
import { appConfig } from '@/config/app'
import {
  getCloseBehavior,
  setCloseBehavior,
  syncCloseBehavior,
} from '@/utils/desktop-preferences'

const isAutoStart = ref(false)
const isAutoStartLoading = ref(false)
const closeBehavior = ref<CloseBehavior>(getCloseBehavior())
const closeBehaviorOptions = computed(() => [
  {
    label: '退出程序',
    value: 'exit' satisfies CloseBehavior,
  },
  {
    label: '最小化到托盘',
    value: 'tray' satisfies CloseBehavior,
  },
])

async function openAuthorSite(url: string) {
  await openUrl(url)
}

async function syncAutostartState() {
  try {
    isAutoStart.value = await isAutostartEnabled()
  }
  catch (error) {
    console.error('Failed to load autostart setting', error)
    isAutoStart.value = false
  }
}

async function toggleAutoStart() {
  isAutoStartLoading.value = true
  try {
    if (isAutoStart.value) {
      await disableAutostart()
    }
    else {
      await enableAutostart()
    }
    isAutoStart.value = !isAutoStart.value
  }
  catch (error) {
    console.error('Failed to toggle autostart', error)
  }
  finally {
    isAutoStartLoading.value = false
  }
}

function handleCloseBehaviorChange(value: CloseBehavior) {
  closeBehavior.value = value
  setCloseBehavior(value)
  void syncCloseBehavior()
}

onMounted(() => {
  void syncAutostartState()
})
</script>

<template>
  <n-tabs type="segment" animated>
    <n-tab-pane name="general" tab="常规">
      <div class="settings-section">
        <h3 class="section-title">启动选项</h3>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">开机自启</span>
            <span class="setting-desc">系统启动时自动运行启动器</span>
          </div>
          <n-switch
            :value="isAutoStart"
            :loading="isAutoStartLoading"
            @update:value="toggleAutoStart"
          />
        </div>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">关闭行为</span>
            <span class="setting-desc">关闭启动器时的行为</span>
          </div>
          <n-select
            v-model:value="closeBehavior"
            :options="closeBehaviorOptions"
            style="width: 150px"
            @update:value="handleCloseBehaviorChange"
          />
        </div>
      </div>
    </n-tab-pane>

    <n-tab-pane name="game" tab="游戏">
      <div class="settings-section">
        <h3 class="section-title">游戏设置</h3>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">游戏目录</span>
            <span class="setting-desc">Minecraft 游戏文件存储位置</span>
          </div>
          <n-button size="small" @click="openAuthorSite('https://minecraft.net')">
            浏览
          </n-button>
        </div>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Java 路径</span>
            <span class="setting-desc">指定 Java 运行环境</span>
          </div>
          <n-button size="small" @click="openAuthorSite('https://java.com')">
            自动检测
          </n-button>
        </div>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">最大内存</span>
            <span class="setting-desc">游戏最大使用内存 (MB)</span>
          </div>
          <n-input-number v-model:value="4096" :min="512" :max="16384" size="small" style="width: 120px" />
        </div>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">窗口分辨率</span>
            <span class="setting-desc">游戏窗口大小</span>
          </div>
          <div class="flex gap-2">
            <n-input-number v-model:value="854" :min="400" :max="3840" size="small" style="width: 100px" />
            <span class="text-stone-400">×</span>
            <n-input-number v-model:value="480" :min="300" :max="2160" size="small" style="width: 100px" />
          </div>
        </div>
      </div>
    </n-tab-pane>

    <n-tab-pane name="about" tab="关于">
      <div class="settings-section">
        <h3 class="section-title">关于</h3>
        <div class="about-info">
          <div class="about-logo">
            <div class="i-mdi-minecraft text-6xl" />
          </div>
          <h2>{{ appConfig.projectName }}</h2>
          <p class="text-stone-400">{{ appConfig.version }}</p>
          <p class="text-stone-500 mt-4 text-sm">
            基于 Vue 3 + Tauri 2 + Naive UI 构建的现代 Minecraft 启动器
          </p>
        </div>
        <div class="setting-item mt-6">
          <div class="setting-info">
            <span class="setting-label">作者</span>
          </div>
          <n-button size="small" text @click="openAuthorSite(appConfig.authorUrl)">
            {{ appConfig.authorName }}
            <template #icon>
              <div class="i-mdi-open-in-new" />
            </template>
          </n-button>
        </div>
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">源代码</span>
          </div>
          <n-button size="small" text @click="openAuthorSite('https://github.com')">
            GitHub
            <template #icon>
              <div class="i-mdi-open-in-new" />
            </template>
          </n-button>
        </div>
      </div>
    </n-tab-pane>
  </n-tabs>
</template>

<style scoped>
.settings-section {
  --uno: py-4;
}

.section-title {
  --uno: text-lg font-medium text-white mb-4;
}

.setting-item {
  --uno: flex items-center justify-between py-3 border-b border-white/8;
}

.setting-item:last-child {
  --uno: border-b-0;
}

.setting-info {
  --uno: flex flex-col;
}

.setting-label {
  --uno: text-white;
}

.setting-desc {
  --uno: text-stone-400 text-sm mt-1;
}

.about-info {
  --uno: flex flex-col items-center text-center py-8;
}

.about-logo {
  --uno: mb-4;
}

.about-info h2 {
  --uno: text-2xl font-medium text-white mt-4;
}
</style>
