<route lang="json5">
{
  name: 'Versions',
  meta: {
    layout: 'main',
    isTab: true,
    tabsName: '版本',
    tabOrder: 2,
  },
}
</route>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'

interface GameVersion {
  id: string
  name: string
  version: string
  loader: string | null
  loader_version: string | null
  path: string
  is_modded: boolean
}

const message = useMessage()
const versions = ref<GameVersion[]>([])
const isLoading = ref(false)
const searchQuery = ref('')

async function loadVersions() {
  isLoading.value = true
  try {
    const result = await invoke<GameVersion[]>('get_game_versions')
    versions.value = result
  }
  catch (error) {
    message.error(`加载版本失败: ${error}`)
  }
  finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadVersions()
})

function getVersionIcon(version: GameVersion) {
  if (version.loader) {
    return 'i-mdi-puzzle'
  }
  return 'i-mdi-cube-outline'
}

function getVersionType(version: GameVersion) {
  if (version.is_modded) return 'modded'
  if (version.loader) return version.loader.toLowerCase().includes('forge') ? 'forge' : 'fabric'
  return 'vanilla'
}

const filteredVersions = ref<GameVersion[]>([])

function filterVersions() {
  const query = searchQuery.value.toLowerCase()
  if (!query) {
    filteredVersions.value = versions.value
  }
  else {
    filteredVersions.value = versions.value.filter(v =>
      v.name.toLowerCase().includes(query) ||
      v.version.toLowerCase().includes(query),
    )
  }
}
</script>

<template>
  <div class="versions-page">
    <div class="page-header">
      <n-input
        v-model:value="searchQuery"
        placeholder="搜索版本..."
        class="search-input"
        @update:value="filterVersions"
      >
        <template #prefix>
          <div class="i-mdi-magnify text-stone-400" />
        </template>
      </n-input>
      <n-button type="primary" @click="loadVersions">
        <template #icon>
          <div class="i-mdi-refresh" />
        </template>
        刷新
      </n-button>
    </div>

    <n-spin :show="isLoading">
      <div class="versions-grid">
        <div
          v-for="version in versions"
          :key="version.id"
          class="version-card"
        >
          <div class="version-icon">
            <div :class="getVersionIcon(version)" />
          </div>
          <div class="version-details">
            <h3>{{ version.name }}</h3>
            <p>{{ version.version }}</p>
            <n-tag v-if="version.loader" :type="getVersionType(version) === 'forge' ? 'warning' : 'info'" size="small">
              {{ version.loader }}
            </n-tag>
            <n-tag v-else type="success" size="small">
              Vanilla
            </n-tag>
          </div>
          <div class="version-actions">
            <n-button size="small" @click="message.info('版本详情功能开发中')">
              <template #icon>
                <div class="i-mdi-information-outline" />
              </template>
            </n-button>
          </div>
        </div>
      </div>
    </n-spin>

    <div v-if="versions.length === 0 && !isLoading" class="empty-state">
      <div class="i-mdi-folder-open-outline text-6xl text-stone-500" />
      <p>暂无版本</p>
      <p class="text-stone-400 text-sm mt-2">请确保 Minecraft 已安装并运行过</p>
    </div>
  </div>
</template>

<style scoped>
.versions-page {
  --uno: flex flex-col h-full p-5;
}

.page-header {
  --uno: flex gap-3 mb-4;
}

.search-input {
  --uno: max-w-md;
}

.versions-grid {
  --uno: grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-y-auto flex-1;
}

.version-card {
  --uno: flex items-center gap-3 p-4 bg-white/6 rounded-xl hover:bg-white/10 transition-colors;
}

.version-icon {
  --uno: w-12 h-12 bg-white/10 rounded-xl flex items-center justify-center text-2xl text-stone-300;
}

.version-details {
  --uno: flex-1;
}

.version-details h3 {
  --uno: text-white font-medium mb-1;
}

.version-details p {
  --uno: text-stone-400 text-sm mb-2;
}

.version-actions {
  --uno: flex gap-2;
}

.empty-state {
  --uno: flex flex-col items-center justify-center flex-1 text-stone-400;
}
</style>
