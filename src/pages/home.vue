<route lang="json5">
{
  name: 'Home',
  meta: {
    layout: 'main',
    isTab: true,
    tabsName: '主页',
    tabOrder: 1,
  },
}
</route>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useMessage, useLoadingBar } from 'naive-ui'

interface GameVersion {
  id: string
  name: string
  version: string
  loader: string | null
  loader_version: string | null
  path: string
  is_modded: boolean
}

interface Account {
  id: string
  username: string
  avatar_url: string
  account_type: string
}

const message = useMessage()
const loadingBar = useLoadingBar()

const gameVersions = ref<GameVersion[]>([])
const accounts = ref<Account[]>([])
const selectedVersion = ref<GameVersion | null>(null)
const selectedAccount = ref<Account | null>(null)
const isLaunching = ref(false)
const isVersionMenuOpen = ref(false)
const isAccountMenuOpen = ref(false)
const memoryInfo = ref('')

const storedAccounts = localStorage.getItem('kyou-launcher-accounts')
if (storedAccounts) {
  try {
    accounts.value = JSON.parse(storedAccounts)
  }
  catch {
    accounts.value = []
  }
}

if (accounts.value.length === 0) {
  accounts.value = [{
    id: 'offline-default',
    username: 'Steve',
    avatar_url: 'https://minotar.net/helm/Steve/32.png',
    account_type: '离线模式',
  }]
  localStorage.setItem('kyou-launcher-accounts', JSON.stringify(accounts.value))
}

selectedAccount.value = accounts.value[0]

async function loadVersions() {
  try {
    const versions = await invoke<GameVersion[]>('get_game_versions')
    gameVersions.value = versions
    if (versions.length > 0 && !selectedVersion.value) {
      selectedVersion.value = versions[0]
    }
  }
  catch (error) {
    console.error('Failed to load versions:', error)
  }
}

onMounted(async () => {
  await loadVersions()

  if (selectedAccount.value) {
    const avatarUrl = `https://minotar.net/helm/${selectedAccount.value.username}/32.png`
    selectedAccount.value = {
      ...selectedAccount.value,
      avatar_url: avatarUrl,
    }
  }

  if (window.performance && window.performance.memory) {
    const memory = window.performance.memory
    const usedMB = Math.round(memory.usedJSHeapSize / 1024 / 1024)
    const totalMB = Math.round(memory.jsHeapSizeLimit / 1024 / 1024)
    memoryInfo.value = `内存: ${usedMB}MB / ${totalMB}MB`
  }
})

function selectVersion(version: GameVersion) {
  selectedVersion.value = version
  isVersionMenuOpen.value = false
}

function selectAccount(account: Account) {
  const avatarUrl = `https://minotar.net/helm/${account.username}/32.png`
  selectedAccount.value = { ...account, avatar_url: avatarUrl }
  isAccountMenuOpen.value = false
}

function toggleVersionMenu() {
  isAccountMenuOpen.value = false
  isVersionMenuOpen.value = !isVersionMenuOpen.value
}

function toggleAccountMenu() {
  isVersionMenuOpen.value = false
  isAccountMenuOpen.value = !isAccountMenuOpen.value
}

function addOfflineAccount() {
  const name = window.prompt('输入离线用户名')
  if (name && name.trim()) {
    const newAccount: Account = {
      id: `offline-${Date.now()}`,
      username: name.trim(),
      avatar_url: `https://minotar.net/helm/${name.trim()}/32.png`,
      account_type: '离线模式',
    }
    accounts.value.push(newAccount)
    localStorage.setItem('kyou-launcher-accounts', JSON.stringify(accounts.value))
    selectAccount(newAccount)
  }
  isAccountMenuOpen.value = false
}

async function launchGame() {
  if (!selectedVersion.value || !selectedAccount.value) {
    message.warning('请选择游戏版本和账户')
    return
  }

  isLaunching.value = true
  loadingBar.start()

  try {
    await invoke('launch_game', {
      config: {
        version_id: selectedVersion.value.id,
        account_id: selectedAccount.value.id,
        max_memory: 4096,
        min_memory: 512,
        java_path: null,
        java_args: [],
        window_width: 854,
        window_height: 480,
        fullscreen: false,
      },
    })
    loadingBar.finish()
    message.success('游戏启动中...')
  }
  catch (error) {
    loadingBar.error()
    message.error(`启动失败: ${error}`)
  }
  finally {
    setTimeout(() => {
      isLaunching.value = false
    }, 1000)
  }
}
</script>

<template>
  <div class="home-page">
    <div class="launch-panel">
      <div class="account-selector" @click="toggleAccountMenu">
        <img
          v-if="selectedAccount"
          :src="selectedAccount.avatar_url"
          :alt="selectedAccount.username"
          class="account-avatar"
        />
        <div v-if="selectedAccount" class="account-info">
          <span class="account-name">{{ selectedAccount.username }}</span>
          <span class="account-type">{{ selectedAccount.account_type }}</span>
        </div>
        <div class="i-iconamoon-arrow-down-2" :class="{ rotated: isAccountMenuOpen }" />
      </div>

      <div v-if="isAccountMenuOpen" class="dropdown-menu">
        <div
          v-for="account in accounts"
          :key="account.id"
          class="dropdown-item"
          :class="{ active: account.id === selectedAccount?.id }"
          @click.stop="selectAccount(account)"
        >
          <img :src="account.avatar_url" :alt="account.username" class="dropdown-avatar" />
          <span>{{ account.username }}</span>
          <span class="badge">{{ account.account_type }}</span>
        </div>
        <div class="menu-divider" />
        <div class="dropdown-item" @click.stop="addOfflineAccount">
          <div class="i-mdi-plus" />
          <span>添加离线账户</span>
        </div>
      </div>

      <div class="version-selector" @click="toggleVersionMenu">
        <div class="version-icon">
          <div class="i-mdi-cube-outline text-2xl" />
        </div>
        <div v-if="selectedVersion" class="version-info">
          <span class="version-name">{{ selectedVersion.name }}</span>
          <span class="version-loader">{{ selectedVersion.loader || 'Vanilla' }}</span>
        </div>
        <div v-else class="version-info">
          <span class="version-name text-stone-400">选择版本</span>
        </div>
        <div class="i-iconamoon-arrow-down-2" :class="{ rotated: isVersionMenuOpen }" />
      </div>

      <div v-if="isVersionMenuOpen" class="dropdown-menu">
        <div
          v-for="version in gameVersions"
          :key="version.id"
          class="dropdown-item"
          :class="{ active: version.id === selectedVersion?.id }"
          @click.stop="selectVersion(version)"
        >
          <div class="i-mdi-cube-outline" />
          <span>{{ version.name }}</span>
          <span class="badge">{{ version.loader || 'Vanilla' }}</span>
        </div>
        <div v-if="gameVersions.length === 0" class="dropdown-item text-stone-400">
          暂无可用版本
        </div>
      </div>

      <div class="launch-footer">
        <span v-if="memoryInfo" class="memory-info">{{ memoryInfo }}</span>
        <n-button
          type="primary"
          size="large"
          :loading="isLaunching"
          :disabled="!selectedVersion || !selectedAccount"
          class="launch-btn"
          @click="launchGame"
        >
          <template #icon>
            <div class="i-mdi-play text-xl" />
          </template>
          启动游戏
        </n-button>
      </div>
    </div>

    <div class="side-panel">
      <n-card title="Minecraft 快讯" class="news-card">
        <template #header-extra>
          <div class="i-mdi-refresh cursor-pointer hover:text-white" />
        </template>
        <div class="news-content">
          <div class="news-item">
            <div class="news-icon">
              <div class="i-mdi-update" />
            </div>
            <div class="news-text">
              <h4>Minecraft 1.21.5 发布</h4>
              <p>春季更新带来全新装饰方块和性能优化</p>
            </div>
          </div>
          <div class="news-item">
            <div class="news-icon">
              <div class="i-mdi-calendar" />
            </div>
            <div class="news-text">
              <h4>Minecraft LIVE 即将到来</h4>
              <p>下个月公布未来大型更新方向</p>
            </div>
          </div>
        </div>
      </n-card>
    </div>
  </div>
</template>

<style scoped>
.home-page {
  --uno: flex gap-5 h-full p-5;
}

.launch-panel {
  --uno: flex-1 flex flex-col gap-4;
}

.side-panel {
  --uno: w-80 flex-shrink-0;
}

.account-selector,
.version-selector {
  --uno: flex items-center gap-3 p-3 bg-white/6 rounded-xl cursor-pointer hover:bg-white/10 transition-colors;
}

.account-selector {
  --uno: relative;
}

.version-selector {
  --uno: relative;
}

.account-avatar {
  --uno: w-10 h-10 rounded-full;
}

.account-info {
  --uno: flex flex-col flex-1;
}

.account-name {
  --uno: text-white font-medium;
}

.account-type {
  --uno: text-stone-400 text-sm;
}

.version-icon {
  --uno: w-10 h-10 bg-white/10 rounded-lg flex items-center justify-center;
}

.version-info {
  --uno: flex flex-col flex-1;
}

.version-name {
  --uno: text-white font-medium;
}

.version-loader {
  --uno: text-stone-400 text-sm;
}

.i-iconamoon-arrow-down-2 {
  --uno: text-stone-400 transition-transform;
}

.i-iconamoon-arrow-down-2.rotated {
  --uno: rotate-180;
}

.dropdown-menu {
  --uno: absolute left-0 right-0 top-full mt-2 bg-[rgba(24,28,34,0.96)] rounded-xl border border-white/10 overflow-hidden z-50;
}

.dropdown-item {
  --uno: flex items-center gap-3 px-4 py-3 cursor-pointer hover:bg-white/10 transition-colors;
}

.dropdown-item.active {
  --uno: bg-white/10;
}

.dropdown-item .badge {
  --uno: ml-auto text-xs px-2 py-0.5 rounded bg-white/10 text-stone-300;
}

.dropdown-avatar {
  --uno: w-8 h-8 rounded-full;
}

.menu-divider {
  --uno: h-px bg-white/10 mx-2;
}

.launch-footer {
  --uno: mt-auto flex flex-col items-center gap-2;
}

.memory-info {
  --uno: text-stone-400 text-sm;
}

.launch-btn {
  --uno: w-full text-lg;
}

.news-card {
  --uno: h-full;
}

.news-card :deep(.n-card-header) {
  --uno: pb-2;
}

.news-content {
  --uno: flex flex-col gap-4;
}

.news-item {
  --uno: flex gap-3;
}

.news-icon {
  --uno: w-10 h-10 bg-white/6 rounded-lg flex items-center justify-center flex-shrink-0 text-stone-300;
}

.news-text h4 {
  --uno: text-white font-medium mb-1;
}

.news-text p {
  --uno: text-stone-400 text-sm;
}
</style>
