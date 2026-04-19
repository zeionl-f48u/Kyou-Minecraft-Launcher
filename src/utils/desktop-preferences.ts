import { ref } from 'vue'

export type CloseBehavior = 'exit' | 'tray'

const CLOSE_BEHAVIOR_KEY = 'kyou-launcher-close-behavior'

function getDefaultCloseBehavior(): CloseBehavior {
  return 'exit'
}

export function getCloseBehavior(): CloseBehavior {
  try {
    const stored = localStorage.getItem(CLOSE_BEHAVIOR_KEY)
    if (stored === 'exit' || stored === 'tray') {
      return stored
    }
  }
  catch {}
  return getDefaultCloseBehavior()
}

export function setCloseBehavior(behavior: CloseBehavior) {
  try {
    localStorage.setItem(CLOSE_BEHAVIOR_KEY, behavior)
  }
  catch {}
}

export async function syncCloseBehavior() {
  const behavior = getCloseBehavior()
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('set_close_to_tray_enabled', { enabled: behavior === 'tray' })
  }
  catch (error) {
    console.error('Failed to sync close behavior', error)
  }
}

export function setupCloseBehaviorSync() {
  window.addEventListener('focus', syncCloseBehavior)
}
