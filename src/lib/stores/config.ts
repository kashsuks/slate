import { invoke } from '@tauri-apps/api/core'
import { writable } from 'svelte/store'

export type EnabledTools = {
    kanban: boolean
    docs: boolean
    calendar: boolean
    sprint: boolean
}

export const enabledTools = writable<EnabledTools>({
    kanban: true,
    docs: false,
    calendar: false,
    sprint: false,
})

export async function isOnboardingComplete(): Promise<boolean> {
    const val = await invoke<string | null>('get_config', { key: 'onboarding_complete' })
    return val === 'true'
}

export async function completeOnboarding(tools: EnabledTools) {
    await invoke('set_config', { key: 'onboarding_complete', value: 'true' })
    await invoke('set_config', { key: 'enabled_tools', value: JSON.stringify(tools) })
    enabledTools.set(tools)
}

export async function loadTools() {
    const raw = await invoke<string | null>('get_config', { key: 'enabled_tools' })
    if (raw) enabledTools.set(JSON.parse(raw))
}
