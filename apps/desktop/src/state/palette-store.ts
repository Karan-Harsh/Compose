import { create } from 'zustand'
import {
  completeAi,
  getLastSelection,
  listCommands
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
  error: string | null
  bootstrap: () => Promise<void>
  refreshContext: () => Promise<void>
  setQuery: (query: string) => void
  moveSelection: (delta: number) => void
  selectIndex: (index: number) => void
  executeSelected: () => Promise<void>
  executeCommand: (commandId: string) => Promise<void>
  clearResult: () => void
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

export const usePaletteStore = create<PaletteStore>((set, get) => ({
  query: '',
  commands: [],
  filteredCommands: [],
  selectedIndex: 0,
  context: null,
  result: null,
  isRunning: false,
  error: null,
  async bootstrap() {
    try {
      const [commands, context] = await Promise.all([
        listCommands(),
        getLastSelection()
      ])
      set({
        commands,
        filteredCommands: commands,
        context,
        selectedIndex: 0,
        error: null
      })
    } catch (bootstrapError) {
      set({
        error: toErrorMessage(bootstrapError, 'Failed to load command palette')
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

    set({ isRunning: true, error: null, result: null })

    try {
      const result = await completeAi({
        commandId,
        messages: [{ role: 'user', content: input }]
      })
      set({ result, isRunning: false, query: `/${commandId}` })
    } catch (executionError) {
      set({
        isRunning: false,
        error: toErrorMessage(executionError, 'Command execution failed')
      })
    }
  },
  clearResult() {
    set({ result: null })
  }
}))
