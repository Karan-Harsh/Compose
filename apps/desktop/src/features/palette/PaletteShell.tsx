import { useEffect, useRef, type KeyboardEvent } from 'react'
import { listen } from '@tauri-apps/api/event'
import { CommandListItem } from '@/features/palette/CommandListItem'
import { hidePalette } from '@/lib/tauri/commands'
import { usePaletteStore } from '@/state/palette-store'

type PaletteOpenedPayload = {
  resumed: boolean
}

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
  const onPaletteOpened = usePaletteStore((state) => state.onPaletteOpened)
  const setQuery = usePaletteStore((state) => state.setQuery)
  const moveSelection = usePaletteStore((state) => state.moveSelection)
  const selectIndex = usePaletteStore((state) => state.selectIndex)
  const executeSelected = usePaletteStore((state) => state.executeSelected)
  const executeCommand = usePaletteStore((state) => state.executeCommand)
  const replaceWithResult = usePaletteStore((state) => state.replaceWithResult)
  const copyResult = usePaletteStore((state) => state.copyResult)
  const clearResult = usePaletteStore((state) => state.clearResult)
  const dismissSession = usePaletteStore((state) => state.dismissSession)
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    void bootstrap()
  }, [bootstrap])

  useEffect(() => {
    let disposed = false
    let unlisten: (() => void) | undefined

    void listen<PaletteOpenedPayload>('palette-opened', (event) => {
      void onPaletteOpened(event.payload.resumed).then(() => {
        if (!disposed) {
          inputRef.current?.focus()
        }
      })
    }).then((fn) => {
      unlisten = fn
    })

    return () => {
      disposed = true
      unlisten?.()
    }
  }, [onPaletteOpened])

  useEffect(() => {
    function onWindowFocus() {
      inputRef.current?.focus()
    }

    window.addEventListener('focus', onWindowFocus)
    return () => window.removeEventListener('focus', onWindowFocus)
  }, [])

  async function dismissPalette() {
    await dismissSession()
    try {
      await hidePalette()
    } catch {
      // Window may already be hidden.
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
    : 'Select text, then open with the hotkey.'

  const showCommands = !result && !isRunning

  return (
    <div className="flex h-screen w-screen items-start justify-center bg-transparent px-3 pt-10 text-stone-100">
      <section className="flex w-full max-w-[480px] flex-col overflow-hidden rounded-xl border border-stone-900/10 bg-stone-50/95 text-stone-950 shadow-[0_18px_50px_rgba(0,0,0,0.35)] backdrop-blur-md">
        <header className="flex items-center gap-3 border-b border-stone-900/8 px-4 py-3">
          <span className="shrink-0 font-mono text-[10px] uppercase tracking-[0.28em] text-amber-700">
            TypeFlow
          </span>
          <input
            ref={inputRef}
            aria-label="Command palette"
            className="min-w-0 flex-1 bg-transparent text-[17px] font-medium tracking-tight text-stone-950 outline-none placeholder:text-stone-400"
            onChange={(event) => setQuery(event.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Rewrite, reply, fix…"
            value={query}
          />
        </header>

        <div className="border-b border-stone-900/8 px-4 py-2">
          <p className="line-clamp-2 text-[12px] leading-5 text-stone-500">
            {contextPreview}
          </p>
        </div>

        {showCommands ? (
          <div className="max-h-52 overflow-y-auto py-1">
            {filteredCommands.length === 0 ? (
              <p className="px-4 py-4 text-sm text-stone-500">No commands match.</p>
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
        ) : null}

        <footer className="border-t border-stone-900/8 px-4 py-3">
          {error ? <p className="text-sm text-red-700">{error}</p> : null}
          {statusMessage ? (
            <p className="text-sm text-stone-600">{statusMessage}</p>
          ) : null}
          {isRunning ? (
            <p className="text-sm text-stone-600">Working…</p>
          ) : null}
          {isInserting ? (
            <p className="text-sm text-stone-600">Replacing…</p>
          ) : null}
          {result ? (
            <div className="space-y-3">
              <p className="max-h-40 overflow-y-auto whitespace-pre-wrap text-sm leading-6 text-stone-800">
                {result.content}
              </p>
              <div className="flex flex-wrap items-center gap-2">
                <button
                  className="rounded-md bg-stone-950 px-3 py-1.5 text-sm font-medium text-white transition hover:bg-stone-800"
                  onClick={() => void replaceWithResult()}
                  type="button"
                >
                  Replace
                </button>
                <button
                  className="rounded-md border border-stone-950/12 bg-white px-3 py-1.5 text-sm font-medium text-stone-900 transition hover:bg-stone-50"
                  onClick={() => void copyResult()}
                  type="button"
                >
                  Copy
                </button>
                <span className="text-[11px] text-stone-400">
                  Enter replace · Esc back
                </span>
              </div>
            </div>
          ) : null}
          {!result && !isRunning && !error && !statusMessage ? (
            <p className="text-[11px] text-stone-400">
              ↑↓ · Enter run · Esc hide
            </p>
          ) : null}
        </footer>
      </section>
    </div>
  )
}
