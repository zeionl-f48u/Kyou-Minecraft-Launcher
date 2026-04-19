<route lang="json5">
{
  name: 'Accounts',
  meta: {
    layout: 'main',
    isTab: true,
    tabsName: '账户',
    tabOrder: 3,
  },
}
</route>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMessage } from 'naive-ui'

interface Account {
  id: string
  username: string
  avatar_url: string
  account_type: string
}

const message = useMessage()
const accounts = ref<Account[]>([])
const showAddDialog = ref(false)
const newUsername = ref('')

function loadAccounts() {
  const stored = localStorage.getItem('kyou-launcher-accounts')
  if (stored) {
    try {
      accounts.value = JSON.parse(stored)
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
    saveAccounts()
  }
}

function saveAccounts() {
  localStorage.setItem('kyou-launcher-accounts', JSON.stringify(accounts.value))
}

onMounted(() => {
  loadAccounts()
})

function getAvatarUrl(username: string) {
  return `https://minotar.net/helm/${username}/32.png`
}

function addOfflineAccount() {
  if (!newUsername.value.trim()) {
    message.warning('请输入用户名')
    return
  }

  const newAccount: Account = {
    id: `offline-${Date.now()}`,
    username: newUsername.value.trim(),
    avatar_url: getAvatarUrl(newUsername.value.trim()),
    account_type: '离线模式',
  }

  accounts.value.push(newAccount)
  saveAccounts()
  showAddDialog.value = false
  newUsername.value = ''
  message.success('账户添加成功')
}

function deleteAccount(account: Account) {
  if (account.id === 'offline-default') {
    message.warning('默认账户不能删除')
    return
  }

  const index = accounts.value.findIndex(a => a.id === account.id)
  if (index > -1) {
    accounts.value.splice(index, 1)
    saveAccounts()
    message.success('账户已删除')
  }
}
</script>

<template>
  <div class="accounts-page">
    <div class="page-header">
      <h2>账户管理</h2>
      <n-button type="primary" @click="showAddDialog = true">
        <template #icon>
          <div class="i-mdi-plus" />
        </template>
        添加账户
      </n-button>
    </div>

    <div class="accounts-grid">
      <div
        v-for="account in accounts"
        :key="account.id"
        class="account-card"
      >
        <img
          :src="account.avatar_url"
          :alt="account.username"
          class="account-avatar"
        />
        <div class="account-info">
          <h3>{{ account.username }}</h3>
          <n-tag :type="account.account_type === '微软正版' ? 'success' : 'default'" size="small">
            {{ account.account_type }}
          </n-tag>
        </div>
        <div class="account-actions">
          <n-button
            size="small"
            type="error"
            :disabled="account.id === 'offline-default'"
            @click="deleteAccount(account)"
          >
            <template #icon>
              <div class="i-mdi-delete-outline" />
            </template>
          </n-button>
        </div>
      </div>
    </div>

    <n-modal v-model:show="showAddDialog" preset="card" title="添加离线账户" style="max-width: 400px;">
      <div class="add-dialog-content">
        <n-input
          v-model:value="newUsername"
          placeholder="输入用户名"
          @keyup.enter="addOfflineAccount"
        />
        <div class="dialog-actions">
          <n-button @click="showAddDialog = false">取消</n-button>
          <n-button type="primary" @click="addOfflineAccount">添加</n-button>
        </div>
      </div>
    </n-modal>
  </div>
</template>

<style scoped>
.accounts-page {
  --uno: flex flex-col h-full p-5;
}

.page-header {
  --uno: flex justify-between items-center mb-4;
}

.page-header h2 {
  --uno: text-white text-xl font-medium;
}

.accounts-grid {
  --uno: grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-y-auto flex-1;
}

.account-card {
  --uno: flex items-center gap-4 p-4 bg-white/6 rounded-xl hover:bg-white/10 transition-colors;
}

.account-avatar {
  --uno: w-14 h-14 rounded-full;
}

.account-info {
  --uno: flex-1;
}

.account-info h3 {
  --uno: text-white font-medium mb-2;
}

.account-actions {
  --uno: flex gap-2;
}

.add-dialog-content {
  --uno: flex flex-col gap-4;
}

.dialog-actions {
  --uno: flex justify-end gap-2 mt-4;
}
</style>
