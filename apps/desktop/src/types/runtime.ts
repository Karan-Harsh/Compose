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

export type ProviderCapabilities = {
  supportsStreaming: boolean
  supportsSystemPrompt: boolean
}

export type AiProviderInfo = {
  id: string
  kind: string
  displayName: string
  configured: boolean
  capabilities: ProviderCapabilities
}

export type CommandDefinition = {
  id: string
  description: string
  requiresInput: boolean
  requiresContext: boolean
}

export type AiMessage = {
  role: string
  content: string
}

export type AiCompletionRequest = {
  providerId?: string
  model?: string
  commandId?: string
  messages: AiMessage[]
}

export type AiCompletionResponse = {
  providerId: string
  model: string
  content: string
  finishReason: string
}
