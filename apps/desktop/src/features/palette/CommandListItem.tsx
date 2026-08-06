type CommandListItemProps = {
  id: string
  description: string
  active: boolean
  onSelect: () => void
  onRun: () => void
}

export function CommandListItem({
  id,
  description,
  active,
  onSelect,
  onRun
}: CommandListItemProps) {
  return (
    <button
      className={`flex w-full items-start gap-3 px-4 py-3 text-left transition ${
        active
          ? 'bg-stone-950 text-stone-50'
          : 'text-stone-800 hover:bg-stone-900/5'
      }`}
      onClick={onRun}
      onMouseEnter={onSelect}
      type="button"
    >
      <span
        className={`font-mono text-sm ${
          active ? 'text-amber-300' : 'text-amber-700'
        }`}
      >
        /{id}
      </span>
      <span
        className={`text-sm leading-5 ${
          active ? 'text-stone-200' : 'text-stone-600'
        }`}
      >
        {description}
      </span>
    </button>
  )
}
