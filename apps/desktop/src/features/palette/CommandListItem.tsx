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
      className={`flex w-full items-center gap-3 px-4 py-2 text-left transition ${
        active
          ? 'bg-stone-950 text-stone-50'
          : 'text-stone-800 hover:bg-stone-900/5'
      }`}
      onClick={onRun}
      onMouseEnter={onSelect}
      type="button"
    >
      <span
        className={`w-20 shrink-0 font-mono text-[12px] ${
          active ? 'text-amber-300' : 'text-amber-700'
        }`}
      >
        /{id}
      </span>
      <span
        className={`truncate text-[13px] leading-5 ${
          active ? 'text-stone-200' : 'text-stone-600'
        }`}
      >
        {description}
      </span>
    </button>
  )
}
