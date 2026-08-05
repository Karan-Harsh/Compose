import { useEffect } from 'react'
import { getHealthCheck, showPalette } from '@/lib/tauri/commands'
import { useAppStore } from '@/state/app-store'

export function App() {
  const appInfo = useAppStore((state) => state.appInfo)
  const health = useAppStore((state) => state.health)
  const settings = useAppStore((state) => state.settings)
  const hotkeyStatus = useAppStore((state) => state.hotkeyStatus)
  const clipboardPreview = useAppStore((state) => state.clipboardPreview)
  const selectedText = useAppStore((state) => state.selectedText)
  const lastSelection = useAppStore((state) => state.lastSelection)
  const error = useAppStore((state) => state.error)
  const isLoading = useAppStore((state) => state.isLoading)
  const loadRuntimeInfo = useAppStore((state) => state.loadRuntimeInfo)
  const loadPlatformState = useAppStore((state) => state.loadPlatformState)
  const persistSettings = useAppStore((state) => state.persistSettings)
  const updateHotkey = useAppStore((state) => state.updateHotkey)
  const readClipboard = useAppStore((state) => state.readClipboard)
  const writeClipboard = useAppStore((state) => state.writeClipboard)
  const captureSelectedText = useAppStore((state) => state.captureSelectedText)
  const refreshLastSelection = useAppStore((state) => state.refreshLastSelection)
  const setHealth = useAppStore((state) => state.setHealth)

  useEffect(() => {
    void loadRuntimeInfo()
    void loadPlatformState()
  }, [loadPlatformState, loadRuntimeInfo])

  async function handleHealthCheck() {
    try {
      const result = await getHealthCheck()
      setHealth(result)
    } catch (healthError) {
      useAppStore
        .getState()
        .setError(
          healthError instanceof Error
            ? healthError.message
            : 'Health check failed'
        )
    }
  }

  async function handleToggleTheme() {
    if (!settings) {
      return
    }

    const nextTheme =
      settings.theme === 'system'
        ? 'light'
        : settings.theme === 'light'
          ? 'dark'
          : 'system'

    await persistSettings({ ...settings, theme: nextTheme })
  }

  async function handleToggleLaunch() {
    if (!settings) {
      return
    }

    await persistSettings({
      ...settings,
      launchAtLogin: !settings.launchAtLogin
    })
  }

  return (
    <main className="min-h-screen bg-[radial-gradient(circle_at_top_left,_rgba(245,158,11,0.24),_transparent_24%),linear-gradient(160deg,_#f7f3e8_0%,_#efe5d0_55%,_#e8dcc4_100%)] text-stone-900">
      <div className="mx-auto flex min-h-screen max-w-6xl flex-col gap-8 px-6 py-10 lg:px-10">
        <header className="flex flex-col gap-4 rounded-[2rem] border border-stone-900/10 bg-white/70 p-8 shadow-[0_24px_80px_rgba(68,46,24,0.12)] backdrop-blur">
          <p className="font-mono text-xs uppercase tracking-[0.4em] text-amber-700">
            Phase 2 Platform Services
          </p>
          <div className="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
            <div className="max-w-3xl space-y-3">
              <h1 className="text-4xl font-semibold tracking-tight text-stone-950 lg:text-6xl">
                Hotkey captures selection before the window steals focus.
              </h1>
              <p className="max-w-2xl text-base leading-7 text-stone-700 lg:text-lg">
                Phase 2 now includes settings, clipboard, OS hotkeys, and
                accessibility text capture via a guarded clipboard fallback.
              </p>
            </div>
            <div className="rounded-[1.5rem] border border-stone-900/10 bg-stone-950 px-5 py-4 text-stone-50 shadow-lg">
              <p className="font-mono text-xs uppercase tracking-[0.3em] text-amber-300">
                Native Status
              </p>
              <p className="mt-2 text-2xl font-semibold">
                {health?.status ?? 'Not checked'}
              </p>
            </div>
          </div>
        </header>

        <section className="grid gap-6 lg:grid-cols-[1.3fr_0.9fr]">
          <article className="rounded-[2rem] border border-stone-900/10 bg-white/75 p-8 shadow-[0_20px_60px_rgba(68,46,24,0.1)] backdrop-blur">
            <div className="flex flex-wrap gap-3">
              <button
                className="rounded-full bg-stone-950 px-5 py-3 text-sm font-medium text-white transition hover:bg-stone-800"
                onClick={() => void loadRuntimeInfo()}
                type="button"
              >
                Refresh App Info
              </button>
              <button
                className="rounded-full bg-amber-500 px-5 py-3 text-sm font-medium text-stone-950 transition hover:bg-amber-400"
                onClick={() => void handleHealthCheck()}
                type="button"
              >
                Run Health Check
              </button>
              <button
                className="rounded-full border border-stone-950/15 bg-white px-5 py-3 text-sm font-medium text-stone-900 transition hover:border-stone-950/30 hover:bg-stone-50"
                onClick={() => void showPalette()}
                type="button"
              >
                Focus Palette Window
              </button>
              <button
                className="rounded-full border border-stone-950/15 bg-white px-5 py-3 text-sm font-medium text-stone-900 transition hover:border-stone-950/30 hover:bg-stone-50"
                onClick={() => void loadPlatformState()}
                type="button"
              >
                Reload Platform State
              </button>
            </div>

            <div className="mt-8 grid gap-4 md:grid-cols-3">
              <ArchitectureCard
                title="Settings"
                detail="SQLite-backed AppSettings via SettingsService."
              />
              <ArchitectureCard
                title="Hotkey"
                detail="OS-registered shortcut captures text, then focuses."
              />
              <ArchitectureCard
                title="Selection"
                detail="AccessibilityService uses clipboard-fallback capture."
              />
            </div>

            <div className="mt-8 grid gap-4 md:grid-cols-2">
              <div className="rounded-[1.5rem] border border-stone-900/10 bg-stone-50 p-5">
                <p className="font-mono text-xs uppercase tracking-[0.25em] text-amber-700">
                  Settings Actions
                </p>
                <div className="mt-4 flex flex-wrap gap-3">
                  <button
                    className="rounded-full bg-stone-950 px-4 py-2 text-sm font-medium text-white transition hover:bg-stone-800"
                    onClick={() => void handleToggleTheme()}
                    type="button"
                  >
                    Cycle Theme
                  </button>
                  <button
                    className="rounded-full border border-stone-950/15 bg-white px-4 py-2 text-sm font-medium text-stone-900 transition hover:bg-stone-50"
                    onClick={() => void handleToggleLaunch()}
                    type="button"
                  >
                    Toggle Launch At Login
                  </button>
                  <button
                    className="rounded-full border border-stone-950/15 bg-white px-4 py-2 text-sm font-medium text-stone-900 transition hover:bg-stone-50"
                    onClick={() =>
                      void updateHotkey('CommandOrControl+Shift+T')
                    }
                    type="button"
                  >
                    Set Hotkey To Ctrl+Shift+T
                  </button>
                </div>
              </div>

              <div className="rounded-[1.5rem] border border-stone-900/10 bg-stone-50 p-5">
                <p className="font-mono text-xs uppercase tracking-[0.25em] text-amber-700">
                  Clipboard Actions
                </p>
                <div className="mt-4 flex flex-wrap gap-3">
                  <button
                    className="rounded-full bg-stone-950 px-4 py-2 text-sm font-medium text-white transition hover:bg-stone-800"
                    onClick={() => void readClipboard()}
                    type="button"
                  >
                    Read Clipboard
                  </button>
                  <button
                    className="rounded-full border border-stone-950/15 bg-white px-4 py-2 text-sm font-medium text-stone-900 transition hover:bg-stone-50"
                    onClick={() => void writeClipboard('TypeFlow Phase 2')}
                    type="button"
                  >
                    Write Test Text
                  </button>
                </div>
                <p className="mt-4 text-sm leading-6 text-stone-600">
                  Preview:{' '}
                  {clipboardPreview?.trim()
                    ? clipboardPreview.slice(0, 120)
                    : 'empty'}
                </p>
              </div>
            </div>

            <div className="mt-4 rounded-[1.5rem] border border-stone-900/10 bg-stone-50 p-5">
              <p className="font-mono text-xs uppercase tracking-[0.25em] text-amber-700">
                Selection Capture
              </p>
              <div className="mt-4 flex flex-wrap gap-3">
                <button
                  className="rounded-full bg-stone-950 px-4 py-2 text-sm font-medium text-white transition hover:bg-stone-800"
                  onClick={() => void captureSelectedText()}
                  type="button"
                >
                  Capture Selected Text
                </button>
                <button
                  className="rounded-full border border-stone-950/15 bg-white px-4 py-2 text-sm font-medium text-stone-900 transition hover:bg-stone-50"
                  onClick={() => void refreshLastSelection()}
                  type="button"
                >
                  Refresh Last Hotkey Capture
                </button>
              </div>
              <p className="mt-4 text-sm leading-6 text-stone-600">
                On-demand:{' '}
                {selectedText
                  ? selectedText.empty
                    ? '(empty)'
                    : selectedText.text.slice(0, 120)
                  : 'none'}
              </p>
              <p className="mt-2 text-sm leading-6 text-stone-600">
                Last hotkey capture:{' '}
                {lastSelection
                  ? lastSelection.empty
                    ? '(empty)'
                    : lastSelection.text.slice(0, 120)
                  : 'none'}
              </p>
            </div>
          </article>

          <aside className="rounded-[2rem] border border-stone-900/10 bg-stone-950 p-8 text-stone-50 shadow-[0_20px_60px_rgba(25,20,14,0.28)]">
            <p className="font-mono text-xs uppercase tracking-[0.35em] text-amber-300">
              Runtime
            </p>
            <dl className="mt-6 space-y-4">
              <DetailRow label="Product" value={appInfo?.name ?? 'Loading'} />
              <DetailRow
                label="Version"
                value={appInfo?.version ?? 'Loading'}
              />
              <DetailRow label="OS" value={appInfo?.os ?? 'Loading'} />
              <DetailRow label="Arch" value={appInfo?.arch ?? 'Loading'} />
              <DetailRow
                label="Theme"
                value={settings?.theme ?? 'Loading'}
              />
              <DetailRow
                label="Launch"
                value={
                  settings
                    ? settings.launchAtLogin
                      ? 'enabled'
                      : 'disabled'
                    : 'Loading'
                }
              />
              <DetailRow
                label="Hotkey"
                value={hotkeyStatus?.shortcut ?? 'Loading'}
              />
              <DetailRow
                label="Hotkey State"
                value={hotkeyStatus?.activation ?? 'Loading'}
              />
            </dl>
            <p className="mt-4 text-xs leading-5 text-stone-400">
              Press the registered shortcut after selecting text in another app.
              On macOS, grant Accessibility and Input Monitoring if prompted.
            </p>
            {error ? (
              <p className="mt-6 rounded-2xl border border-red-400/20 bg-red-500/10 px-4 py-3 text-sm text-red-100">
                {error}
              </p>
            ) : null}
            <p className="mt-6 text-sm leading-6 text-stone-300">
              {isLoading
                ? 'Loading native metadata through the Rust command boundary.'
                : 'Platform services are ready for deeper Phase 2 work.'}
            </p>
          </aside>
        </section>
      </div>
    </main>
  )
}

type ArchitectureCardProps = {
  title: string
  detail: string
}

function ArchitectureCard({ title, detail }: ArchitectureCardProps) {
  return (
    <div className="rounded-[1.5rem] border border-stone-900/10 bg-stone-50 p-5">
      <p className="font-mono text-xs uppercase tracking-[0.25em] text-amber-700">
        {title}
      </p>
      <p className="mt-3 text-sm leading-6 text-stone-700">{detail}</p>
    </div>
  )
}

type DetailRowProps = {
  label: string
  value: string
}

function DetailRow({ label, value }: DetailRowProps) {
  return (
    <div className="flex items-center justify-between gap-4 border-b border-stone-50/10 pb-3">
      <dt className="font-mono text-xs uppercase tracking-[0.25em] text-stone-400">
        {label}
      </dt>
      <dd className="text-right text-sm text-stone-100">{value}</dd>
    </div>
  )
}
