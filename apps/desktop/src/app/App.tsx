import { useEffect } from 'react'
import { PaletteShell } from '@/features/palette/PaletteShell'
import { useAppStore } from '@/state/app-store'

export function App() {
  const loadPlatformState = useAppStore((state) => state.loadPlatformState)

  useEffect(() => {
    void loadPlatformState()
  }, [loadPlatformState])

  return <PaletteShell />
}
