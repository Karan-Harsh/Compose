import { create } from 'zustand'
import {
  completeAi,
  getLastSelection,
  insertText,
  listCommands,
  setClipboardText,
  setPaletteSession
} from '@/lib/tauri/commands'
import type {
  AiCompletionResponse,
  CommandDefinition,
  SelectionCapture
} from '@/types/runtime'

type PaletteStore = {
  query: string
  commands: CommandDefinition[]
  filteredCommands: CommandDefinition[]
  selectedIndex: number
  context: SelectionCapture | null
  result: AiCompletionResponse | null
  isRunning: boolean
  isInserting: boolean
  error: string | null
  statusMessage: string | null
  bootstrap: () => Promise<void>
  onPaletteOpened: (resumed: boolean) => Promise<void>
  refreshContext: () => Promise<void>
  setQuery: (query: string) => void
  moveSelection: (delta: number) => void
  selectIndex: (index: number) => void
  executeSelected: () => Promise<void>
  executeCommand: (commandId: string) => Promise<void>
  replaceWithResult: () => Promise<void>
  copyResult: () => Promise<void>
  clearResult: () => void
  dismissSession: () => Promise<void>
}

function toErrorMessage(error: unknown, fallback: string): string {
  return error instanceof Error ? error.message : fallback
}

function filterCommands(
  commands: CommandDefinition[],
  query: string
): CommandDefinition[] {
  const normalized = query.trim().replace(/^\//, '').toLowerCase()

  if (!normalized) {
    return commands
  }

  return commands.filter((command) => {
    return (
      command.id.toLowerCase().includes(normalized) ||
      command.description.toLowerCase().includes(normalized)
    )
  })
}

async function syncSessionActive(active: boolean) {
  try {
    await setPaletteSession(active)
  } catch (error) {
    console.warn('[TypeFlow] failed to sync palette session', error)
  }
}

export const usePaletteStore = create<PaletteStore>((set, get) => ({
  query: '',
  commands: [],
  filteredCommands: [],
  selectedIndex: 0,
  context: null,
  result: null,
  isRunning: false,
  isInserting: false,
  error: null,
  statusMessage: null,
  async bootstrap() {
    try {
      const [commands, context] = await Promise.all([
        listCommands(),
        getLastSelection()
      ])
      set({
        commands,
        filteredCommands: filterCommands(commands, get().query),
        context,
        error: null
      })
    } catch (bootstrapError) {
      set({
        error: toErrorMessage(bootstrapError, 'Failed to load command palette')
      })
    }
  },
  async onPaletteOpened(resumed) {
    if (resumed) {
      // Keep query/result; only refresh context snapshot if needed.
      try {
        const context = await getLastSelection()
        set({ context, error: null, statusMessage: null })
      } catch {
        // Keep existing context on resume failures.
      }
      return
    }

    try {
      const context = await getLastSelection()
      const commands = get().commands.length
        ? get().commands
        : await listCommands()
      set({
        commands,
        filteredCommands: commands,
        context,
        query: '',
        selectedIndex: 0,
        result: null,
        isRunning: false,
        error: null,
        statusMessage: null
      })
      await syncSessionActive(false)
    } catch (openError) {
      set({
        error: toErrorMessage(openError, 'Failed to open command palette')
      })
    }
  },
  async refreshContext() {
    try {
      const context = await getLastSelection()
      set({ context, error: null })
    } catch (contextError) {
      set({
        error: toErrorMessage(contextError, 'Failed to refresh selection')
      })
    }
  },
  setQuery(query) {
    const filteredCommands = filterCommands(get().commands, query)
    set({
      query,
      filteredCommands,
      selectedIndex: 0,
      error: null
    })
  },
  moveSelection(delta) {
    const total = get().filteredCommands.length
    if (total === 0) {
      return
    }

    const next = (get().selectedIndex + delta + total) % total
    set({ selectedIndex: next })
  },
  selectIndex(index) {
    const total = get().filteredCommands.length
    if (index < 0 || index >= total) {
      return
    }
    set({ selectedIndex: index })
  },
  async executeSelected() {
    const command = get().filteredCommands[get().selectedIndex]
    if (!command) {
      return
    }
    await get().executeCommand(command.id)
  },
  async executeCommand(commandId) {
    const input =
      get().context?.text?.trim() ||
      'Select text in another app, then open TypeFlow with the hotkey.'

    set({ isRunning: true, error: null, result: null, statusMessage: null })
    await syncSessionActive(true)

    try {
      console.info('[TypeFlow] running command', commandId, {
        inputChars: input.length
      })
      const result = await completeAi({
        commandId,
        messages: [{ role: 'user', content: input }]
      })
      console.info('[TypeFlow] command ok', commandId, {
        provider: result.providerId,
        model: result.model,
        chars: result.content.length
      })
      set({ result, isRunning: false, query: `/${commandId}` })
      await syncSessionActive(true)
    } catch (executionError) {
      const message = toErrorMessage(executionError, 'Command execution failed')
      console.error('[TypeFlow] command failed', commandId, message, executionError)
      set({
        isRunning: false,
        error: message
      })
      await syncSessionActive(false)
    }
  },
  async replaceWithResult() {
    const content = get().result?.content
    if (!content?.trim()) {
      set({ error: 'No result available to insert' })
      return
    }

    set({ isInserting: true, error: null, statusMessage: null })
    await syncSessionActive(true)

    try {
      console.info('[TypeFlow] replace/insert start', { chars: content.length })
      const insertion = await insertText(content)
      console.info('[TypeFlow] replace/insert ok', insertion)
      set({
        isInserting: false,
        result: null,
        query: '',
        statusMessage: null
      })
      await syncSessionActive(false)
    } catch (insertionError) {
      const message = toErrorMessage(insertionError, 'Failed to insert text')
      console.error('[TypeFlow] replace/insert failed', message, insertionError)
      set({
        isInserting: false,
        error: message
      })
      await syncSessionActive(Boolean(get().result))
    }
  },
  async copyResult() {
    const content = get().result?.content
    if (!content?.trim()) {
      set({ error: 'No result available to copy' })
      return
    }

    try {
      await setClipboardText(content)
      set({ statusMessage: 'Copied', error: null })
    } catch (copyError) {
      set({
        error: toErrorMessage(copyError, 'Failed to copy result')
      })
    }
  },
  clearResult() {
    set({ result: null, statusMessage: null })
    void syncSessionActive(false)
  },
  async dismissSession() {
    set({
      result: null,
      query: '',
      statusMessage: null,
      error: null,
      isRunning: false
    })
    await syncSessionActive(false)
  }
}))
