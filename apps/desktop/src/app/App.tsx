import { useEffect } from 'react'
import { getHealthCheck, showPalette } from '@/lib/tauri/commands'
import { useAppStore } from '@/state/app-store'

export function App() {
  const appInfo = useAppStore((state) => state.appInfo)
  const health = useAppStore((state) => state.health)
  const error = useAppStore((state) => state.error)
  const isLoading = useAppStore((state) => state.isLoading)
  const loadRuntimeInfo = useAppStore((state) => state.loadRuntimeInfo)
  const setHealth = useAppStore((state) => state.setHealth)

  useEffect(() => {
    void loadRuntimeInfo()
  }, [loadRuntimeInfo])

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

  return (
    <main className="min-h-screen bg-[radial-gradient(circle_at_top_left,_rgba(245,158,11,0.24),_transparent_24%),linear-gradient(160deg,_#f7f3e8_0%,_#efe5d0_55%,_#e8dcc4_100%)] text-stone-900">
      <div className="mx-auto flex min-h-screen max-w-6xl flex-col gap-8 px-6 py-10 lg:px-10">
        <header className="flex flex-col gap-4 rounded-[2rem] border border-stone-900/10 bg-white/70 p-8 shadow-[0_24px_80px_rgba(68,46,24,0.12)] backdrop-blur">
          <p className="font-mono text-xs uppercase tracking-[0.4em] text-amber-700">
            Phase 1 Scaffold
          </p>
          <div className="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
            <div className="max-w-3xl space-y-3">
              <h1 className="text-4xl font-semibold tracking-tight text-stone-950 lg:text-6xl">
                TypeFlow validates the architecture before it chases features.
              </h1>
              <p className="max-w-2xl text-base leading-7 text-stone-700 lg:text-lg">
                This shell proves the React-to-Rust boundary, the command
                client, and the service-oriented native structure without
                pulling AI workflows into the scaffold too early.
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
            </div>

            <div className="mt-8 grid gap-4 md:grid-cols-3">
              <ArchitectureCard
                title="React"
                detail="UI only. No platform APIs. Uses a typed command client."
              />
              <ArchitectureCard
                title="Tauri Commands"
                detail="Thin bridge that delegates to native services."
              />
              <ArchitectureCard
                title="Rust Services"
                detail="Own native behavior and keep OS concerns off the frontend."
              />
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
            </dl>
            {error ? (
              <p className="mt-6 rounded-2xl border border-red-400/20 bg-red-500/10 px-4 py-3 text-sm text-red-100">
                {error}
              </p>
            ) : null}
            <p className="mt-6 text-sm leading-6 text-stone-300">
              {isLoading
                ? 'Loading native metadata through the Rust command boundary.'
                : 'The scaffold is ready for Phase 2 service expansion.'}
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
