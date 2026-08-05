import { create } from 'zustand'
import {
  getAppInfo,
  getClipboardText,
  getHotkeyStatus,
  getLastSelection,
  getSelectedText,
  loadSettings,
  saveSettings,
  setClipboardText,
  setGlobalHotkey
} from '@/lib/tauri/commands'
import type {
  AppInfo,
  AppSettings,
  HealthCheck,
  HotkeyStatus,
  SelectedText,
  SelectionCapture
} from '@/types/runtime'

type AppStore = {
  appInfo: AppInfo | null
  health: HealthCheck | null
  settings: AppSettings | null
  hotkeyStatus: HotkeyStatus | null
  clipboardPreview: string | null
  selectedText: SelectedText | null
  lastSelection: SelectionCapture | null
  isLoading: boolean
  error: string | null
  loadRuntimeInfo: () => Promise<void>
  loadPlatformState: () => Promise<void>
  persistSettings: (settings: AppSettings) => Promise<void>
  updateHotkey: (shortcut: string) => Promise<void>
  readClipboard: () => Promise<void>
  writeClipboard: (text: string) => Promise<void>
  captureSelectedText: () => Promise<void>
  refreshLastSelection: () => Promise<void>
  setHealth: (health: HealthCheck) => void
  setError: (error: string | null) => void
}

function toErrorMessage(error: unknown, fallback: string): string {
  return error instanceof Error ? error.message : fallback
}

export const useAppStore = create<AppStore>((set) => ({
  appInfo: null,
  health: null,
  settings: null,
  hotkeyStatus: null,
  clipboardPreview: null,
  selectedText: null,
  lastSelection: null,
  isLoading: false,
  error: null,
  async loadRuntimeInfo() {
    set({ isLoading: true, error: null })

    try {
      const appInfo = await getAppInfo()
      set({ appInfo, isLoading: false })
    } catch (runtimeError) {
      set({
        error: toErrorMessage(
          runtimeError,
          'Failed to load application metadata'
        ),
        isLoading: false
      })
    }
  },
  async loadPlatformState() {
    set({ isLoading: true, error: null })

    try {
      const [settings, hotkeyStatus, lastSelection] = await Promise.all([
        loadSettings(),
        getHotkeyStatus(),
        getLastSelection()
      ])
      set({ settings, hotkeyStatus, lastSelection, isLoading: false })
    } catch (platformError) {
      set({
        error: toErrorMessage(
          platformError,
          'Failed to load platform services'
        ),
        isLoading: false
      })
    }
  },
  async persistSettings(settings) {
    try {
      const saved = await saveSettings(settings)
      const hotkeyStatus = await getHotkeyStatus()
      set({ settings: saved, hotkeyStatus, error: null })
    } catch (settingsError) {
      set({
        error: toErrorMessage(settingsError, 'Failed to save settings')
      })
    }
  },
  async updateHotkey(shortcut) {
    try {
      const hotkeyStatus = await setGlobalHotkey(shortcut)
      set((state) => ({
        hotkeyStatus,
        settings: state.settings
          ? { ...state.settings, globalHotkey: hotkeyStatus.shortcut }
          : state.settings,
        error: null
      }))
    } catch (hotkeyError) {
      set({
        error: toErrorMessage(hotkeyError, 'Failed to update hotkey')
      })
    }
  },
  async readClipboard() {
    try {
      const clipboard = await getClipboardText()
      set({ clipboardPreview: clipboard.text, error: null })
    } catch (clipboardError) {
      set({
        error: toErrorMessage(clipboardError, 'Failed to read clipboard')
      })
    }
  },
  async writeClipboard(text) {
    try {
      await setClipboardText(text)
      set({ clipboardPreview: text, error: null })
    } catch (clipboardError) {
      set({
        error: toErrorMessage(clipboardError, 'Failed to write clipboard')
      })
    }
  },
  async captureSelectedText() {
    try {
      const selectedText = await getSelectedText()
      const lastSelection = await getLastSelection()
      set({ selectedText, lastSelection, error: null })
    } catch (selectionError) {
      set({
        error: toErrorMessage(
          selectionError,
          'Failed to capture selected text'
        )
      })
    }
  },
  async refreshLastSelection() {
    try {
      const lastSelection = await getLastSelection()
      set({ lastSelection, error: null })
    } catch (selectionError) {
      set({
        error: toErrorMessage(
          selectionError,
          'Failed to load last selection'
        )
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
