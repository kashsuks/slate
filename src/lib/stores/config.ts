import { writable } from 'svelte/store'
import { apiGetConfig, apiSetConfig } from '$lib/api'

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
    const val = await apiGetConfig('onboarding_complete')
    return val === 'true'
}

export async function completeOnboarding(tools: EnabledTools) {
    await apiSetConfig('onboarding_compelete', 'true')
    await apiSetConfig('enabled_tools', JSON.stringify(tools))
    enabledTools.set(tools)
}

export async function loadTools() {
    const raw = await apiGetConfig('enabled_tools')
    if (raw) enabledTools.set(JSON.parse(raw))
}
