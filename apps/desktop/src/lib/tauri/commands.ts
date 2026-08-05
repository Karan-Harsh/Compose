import { invoke } from '@tauri-apps/api/core'
import type {
  AppInfo,
  AppSettings,
  ClipboardText,
  HealthCheck,
  HotkeyStatus,
  SelectedText,
  SelectionCapture
} from '@/types/runtime'

export async function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>('get_app_info')
}

export async function getHealthCheck(): Promise<HealthCheck> {
  return invoke<HealthCheck>('health_check')
}

export async function showPalette(): Promise<void> {
  return invoke<void>('show_palette')
}

export async function loadSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('load_settings')
}

export async function saveSettings(
  settings: AppSettings
): Promise<AppSettings> {
  return invoke<AppSettings>('save_settings', { request: settings })
}

export async function getClipboardText(): Promise<ClipboardText> {
  return invoke<ClipboardText>('get_clipboard_text')
}

export async function setClipboardText(text: string): Promise<void> {
  return invoke<void>('set_clipboard_text', { request: { text } })
}

export async function getHotkeyStatus(): Promise<HotkeyStatus> {
  return invoke<HotkeyStatus>('get_hotkey_status')
}

export async function setGlobalHotkey(shortcut: string): Promise<HotkeyStatus> {
  return invoke<HotkeyStatus>('set_global_hotkey', { request: { shortcut } })
}

export async function getSelectedText(): Promise<SelectedText> {
  return invoke<SelectedText>('get_selected_text')
}

export async function getLastSelection(): Promise<SelectionCapture | null> {
  return invoke<SelectionCapture | null>('get_last_selection')
}
