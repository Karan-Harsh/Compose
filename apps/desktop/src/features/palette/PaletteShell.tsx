import { useEffect, useRef, type KeyboardEvent } from 'react'
import { CommandListItem } from '@/features/palette/CommandListItem'
import { hidePalette } from '@/lib/tauri/commands'
import { usePaletteStore } from '@/state/palette-store'

export function PaletteShell() {
  const query = usePaletteStore((state) => state.query)
  const filteredCommands = usePaletteStore((state) => state.filteredCommands)
  const selectedIndex = usePaletteStore((state) => state.selectedIndex)
  const context = usePaletteStore((state) => state.context)
  const result = usePaletteStore((state) => state.result)
  const isRunning = usePaletteStore((state) => state.isRunning)
  const isInserting = usePaletteStore((state) => state.isInserting)
  const error = usePaletteStore((state) => state.error)
  const statusMessage = usePaletteStore((state) => state.statusMessage)
  const bootstrap = usePaletteStore((state) => state.bootstrap)
  const refreshContext = usePaletteStore((state) => state.refreshContext)
  const setQuery = usePaletteStore((state) => state.setQuery)
  const moveSelection = usePaletteStore((state) => state.moveSelection)
  const selectIndex = usePaletteStore((state) => state.selectIndex)
  const executeSelected = usePaletteStore((state) => state.executeSelected)
  const executeCommand = usePaletteStore((state) => state.executeCommand)
  const replaceWithResult = usePaletteStore((state) => state.replaceWithResult)
  const copyResult = usePaletteStore((state) => state.copyResult)
  const clearResult = usePaletteStore((state) => state.clearResult)
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    void bootstrap()
    inputRef.current?.focus()
  }, [bootstrap])

  useEffect(() => {
    function onFocus() {
      void refreshContext()
      inputRef.current?.focus()
    }

    window.addEventListener('focus', onFocus)
    return () => window.removeEventListener('focus', onFocus)
  }, [refreshContext])

  async function dismissPalette() {
    clearResult()
    setQuery('')
    try {
      await hidePalette()
    } catch {
      // Window may already be hidden via blur.
    }
  }

  function handleKeyDown(event: KeyboardEvent<HTMLInputElement>) {
    if (event.key === 'ArrowDown') {
      event.preventDefault()
      moveSelection(1)
      return
    }

    if (event.key === 'ArrowUp') {
      event.preventDefault()
      moveSelection(-1)
      return
    }

    if (event.key === 'Enter' && (event.metaKey || event.ctrlKey) && result) {
      event.preventDefault()
      void replaceWithResult()
      return
    }

    if (event.key === 'Enter') {
      event.preventDefault()
      if (result) {
        void replaceWithResult()
      } else {
        void executeSelected()
      }
      return
    }

    if (event.key === 'Escape') {
      event.preventDefault()
      if (result) {
        clearResult()
      } else {
        void dismissPalette()
      }
    }
  }

  const contextPreview = context?.text?.trim()
    ? context.text.trim()
    : 'No selection captured yet. Highlight text, then use the global hotkey.'

  return (
    <div className="flex min-h-screen items-start justify-center bg-[linear-gradient(160deg,#1c1917_0%,#292524_45%,#44403c_100%)] px-4 py-10 text-stone-100">
      <section className="w-full max-w-xl overflow-hidden rounded-2xl border border-white/10 bg-stone-50 text-stone-950 shadow-[0_30px_80px_rgba(0,0,0,0.45)]">
        <header className="border-b border-stone-900/10 px-5 py-4">
          <p className="font-mono text-[11px] uppercase tracking-[0.35em] text-amber-700">
            TypeFlow
          </p>
          <input
            ref={inputRef}
            aria-label="Command palette"
            className="mt-3 w-full bg-transparent text-2xl font-medium tracking-tight text-stone-950 outline-none placeholder:text-stone-400"
            onChange={(event) => setQuery(event.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Search commands or type /rewrite"
            value={query}
          />
        </header>

        <div className="border-b border-stone-900/10 px-5 py-3">
          <p className="font-mono text-[10px] uppercase tracking-[0.3em] text-stone-400">
            Context
          </p>
          <p className="mt-2 line-clamp-3 text-sm leading-6 text-stone-600">
            {contextPreview}
          </p>
        </div>

        <div className="max-h-56 overflow-y-auto">
          {filteredCommands.length === 0 ? (
            <p className="px-5 py-6 text-sm text-stone-500">No commands match.</p>
          ) : (
            filteredCommands.map((command, index) => (
              <CommandListItem
                key={command.id}
                active={index === selectedIndex}
                description={command.description}
                id={command.id}
                onRun={() => void executeCommand(command.id)}
                onSelect={() => selectIndex(index)}
              />
            ))
          )}
        </div>

        <footer className="border-t border-stone-900/10 bg-stone-100/80 px-5 py-4">
          {error ? (
            <p className="text-sm text-red-700">{error}</p>
          ) : null}
          {statusMessage ? (
            <p className="text-sm text-stone-600">{statusMessage}</p>
          ) : null}
          {isRunning ? (
            <p className="text-sm text-stone-600">Running command…</p>
          ) : null}
          {isInserting ? (
            <p className="text-sm text-stone-600">Inserting into previous app…</p>
          ) : null}
          {result ? (
            <div className="space-y-3">
              <p className="font-mono text-[10px] uppercase tracking-[0.3em] text-amber-700">
                Result · {result.providerId} · {result.finishReason}
              </p>
              <p className="whitespace-pre-wrap text-sm leading-6 text-stone-800">
                {result.content}
              </p>
              <div className="flex flex-wrap gap-2">
                <button
                  className="rounded-full bg-stone-950 px-4 py-2 text-sm font-medium text-white transition hover:bg-stone-800"
                  onClick={() => void replaceWithResult()}
                  type="button"
                >
                  Replace
                </button>
                <button
                  className="rounded-full border border-stone-950/15 bg-white px-4 py-2 text-sm font-medium text-stone-900 transition hover:bg-stone-50"
                  onClick={() => void copyResult()}
                  type="button"
                >
                  Copy
                </button>
              </div>
              <p className="text-xs text-stone-500">
                Enter / ⌘Enter replaces selection in the previous app
              </p>
            </div>
          ) : (
            <p className="text-sm text-stone-500">
              ↑↓ navigate · Enter run · Esc hide · blur hides
            </p>
          )}
        </footer>
      </section>
    </div>
  )
}
