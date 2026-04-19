<route lang="json5">
{
  name: 'Downloads',
  meta: {
    layout: 'main',
    isTab: true,
    tabsName: '下载',
    tabOrder: 4,
  },
}
</route>

<script setup lang="ts">
import { ref } from 'vue'
import { useMessage } from 'naive-ui'

interface DownloadItem {
  id: string
  name: string
  type: 'mod' | 'resourcepack' | 'shader' | 'world' | 'other'
  status: 'downloading' | 'completed' | 'failed' | 'pending'
  progress: number
  size: string
  source: string
}

const message = useMessage()
const downloads = ref<DownloadItem[]>([
  {
    id: '1',
    name: 'Sodium 0.15.11',
    type: 'mod',
    status: 'completed',
    progress: 100,
    size: '2.3 MB',
    source: 'Modrinth',
  },
  {
    id: '2',
    name: 'Iris Shaders 1.7.5',
    type: 'mod',
    status: 'downloading',
    progress: 67,
    size: '1.8 MB',
    source: 'Modrinth',
  },
  {
    id: '3',
    name: 'Faithful 32x',
    type: 'resourcepack',
    status: 'pending',
    progress: 0,
    size: '15.4 MB',
    source: 'CurseForge',
  },
])

function getTypeIcon(type: string) {
  switch (type) {
    case 'mod': return 'i-mdi-puzzle'
    case 'resourcepack': return 'i-mdi-palette'
    case 'shader': return 'i-mdi-brightness-6'
    case 'world': return 'i-mdi-earth'
    default: return 'i-mdi-file'
  }
}

function getStatusColor(status: string) {
  switch (status) {
    case 'completed': return 'success'
    case 'downloading': return 'info'
    case 'failed': return 'error'
    case 'pending': return 'default'
    default: return 'default'
  }
}

function getStatusText(status: string) {
  switch (status) {
    case 'completed': return '已完成'
    case 'downloading': return '下载中'
    case 'failed': return '失败'
    case 'pending': return '等待中'
    default: return status
  }
}

function pauseDownload(item: DownloadItem) {
  if (item.status === 'downloading') {
    item.status = 'pending'
    message.info(`已暂停: ${item.name}`)
  }
}

function resumeDownload(item: DownloadItem) {
  if (item.status === 'pending') {
    item.status = 'downloading'
    message.info(`已恢复: ${item.name}`)
  }
}

function retryDownload(item: DownloadItem) {
  if (item.status === 'failed') {
    item.status = 'downloading'
    item.progress = 0
    message.info(`正在重试: ${item.name}`)
  }
}

function deleteDownload(item: DownloadItem) {
  const index = downloads.value.findIndex(d => d.id === item.id)
  if (index > -1) {
    downloads.value.splice(index, 1)
    message.success(`已删除: ${item.name}`)
  }
}

function clearCompleted() {
  downloads.value = downloads.value.filter(d => d.status !== 'completed')
  message.success('已清除已完成项')
}
</script>

<template>
  <div class="downloads-page">
    <div class="page-header">
      <h2>下载管理</h2>
      <div class="header-actions">
        <n-button size="small" @click="clearCompleted">
          <template #icon>
            <div class="i-mdi-delete-sweep" />
          </template>
          清除已完成
        </n-button>
        <n-button type="primary" size="small" @click="message.info('添加下载功能开发中')">
          <template #icon>
            <div class="i-mdi-plus" />
          </template>
          添加下载
        </n-button>
      </div>
    </div>

    <div class="downloads-list">
      <div
        v-for="item in downloads"
        :key="item.id"
        class="download-item"
      >
        <div class="item-icon">
          <div :class="getTypeIcon(item.type)" />
        </div>
        <div class="item-info">
          <h4>{{ item.name }}</h4>
          <div class="item-meta">
            <n-tag :type="getStatusColor(item.status)" size="small">
              {{ getStatusText(item.status) }}
            </n-tag>
            <span class="size">{{ item.size }}</span>
            <span class="source">{{ item.source }}</span>
          </div>
          <n-progress
            v-if="item.status === 'downloading'"
            type="line"
            :percentage="item.progress"
            :show-indicator="false"
            :height="4"
            class="mt-2"
          />
        </div>
        <div class="item-actions">
          <n-button
            v-if="item.status === 'downloading'"
            size="small"
            circle
            @click="pauseDownload(item)"
          >
            <template #icon>
              <div class="i-mdi-pause" />
            </template>
          </n-button>
          <n-button
            v-if="item.status === 'pending'"
            size="small"
            circle
            @click="resumeDownload(item)"
          >
            <template #icon>
              <div class="i-mdi-play" />
            </template>
          </n-button>
          <n-button
            v-if="item.status === 'failed'"
            size="small"
            circle
            @click="retryDownload(item)"
          >
            <template #icon>
              <div class="i-mdi-refresh" />
            </template>
          </n-button>
          <n-button
            size="small"
            circle
            @click="deleteDownload(item)"
          >
            <template #icon>
              <div class="i-mdi-close" />
            </template>
          </n-button>
        </div>
      </div>
    </div>

    <div v-if="downloads.length === 0" class="empty-state">
      <div class="i-mdi-download-outline text-6xl text-stone-500" />
      <p>暂无下载任务</p>
      <p class="text-stone-400 text-sm mt-2">点击添加下载来获取 Mods、资源包等内容</p>
    </div>
  </div>
</template>

<style scoped>
.downloads-page {
  --uno: flex flex-col h-full p-5;
}

.page-header {
  --uno: flex justify-between items-center mb-4;
}

.page-header h2 {
  --uno: text-white text-xl font-medium;
}

.header-actions {
  --uno: flex gap-2;
}

.downloads-list {
  --uno: flex flex-col gap-3 overflow-y-auto flex-1;
}

.download-item {
  --uno: flex items-center gap-4 p-4 bg-white/6 rounded-xl hover:bg-white/10 transition-colors;
}

.item-icon {
  --uno: w-12 h-12 bg-white/10 rounded-xl flex items-center justify-center text-2xl text-stone-300;
}

.item-info {
  --uno: flex-1;
}

.item-info h4 {
  --uno: text-white font-medium mb-2;
}

.item-meta {
  --uno: flex items-center gap-2;
}

.item-meta .size,
.item-meta .source {
  --uno: text-stone-400 text-sm;
}

.item-actions {
  --uno: flex gap-2;
}

.empty-state {
  --uno: flex flex-col items-center justify-center flex-1 text-stone-400;
}
</style>
