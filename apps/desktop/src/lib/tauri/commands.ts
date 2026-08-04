import { invoke } from '@tauri-apps/api/core'
import type { AppInfo, HealthCheck } from '@/types/runtime'

export async function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>('get_app_info')
}

export async function getHealthCheck(): Promise<HealthCheck> {
  return invoke<HealthCheck>('health_check')
}

export async function showPalette(): Promise<void> {
  return invoke<void>('show_palette')
}
