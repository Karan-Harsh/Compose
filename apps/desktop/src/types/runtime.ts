export type AppInfo = {
  name: string
  version: string
  os: string
  arch: string
}

export type HealthCheck = {
  status: string
}

export type AppSettings = {
  launchAtLogin: boolean
  globalHotkey: string
  theme: 'system' | 'light' | 'dark' | string
}

export type ClipboardText = {
  text: string
}

export type HotkeyStatus = {
  shortcut: string
  registered: boolean
  activation: string
}

export type SelectedText = {
  text: string
  method: string
  empty: boolean
}

export type SelectionCapture = {
  text: string
  method: string
  empty: boolean
  capturedAtMs: number
}
