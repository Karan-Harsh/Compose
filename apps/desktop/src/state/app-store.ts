import { create } from 'zustand'
import { getAppInfo } from '@/lib/tauri/commands'
import type { AppInfo, HealthCheck } from '@/types/runtime'

type AppStore = {
  appInfo: AppInfo | null
  health: HealthCheck | null
  isLoading: boolean
  error: string | null
  loadRuntimeInfo: () => Promise<void>
  setHealth: (health: HealthCheck) => void
  setError: (error: string | null) => void
}

export const useAppStore = create<AppStore>((set) => ({
  appInfo: null,
  health: null,
  isLoading: false,
  error: null,
  async loadRuntimeInfo() {
    set({ isLoading: true, error: null })

    try {
      const appInfo = await getAppInfo()
      set({ appInfo, isLoading: false })
    } catch (runtimeError) {
      set({
        error:
          runtimeError instanceof Error
            ? runtimeError.message
            : 'Failed to load application metadata',
        isLoading: false
      })
    }
  },
  setHealth(health) {
    set({ health })
  },
  setError(error) {
    set({ error })
  }
}))
