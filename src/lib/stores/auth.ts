import { writable, get } from 'svelte/store'
import { getServerUrl } from '$lib/api'

export type AuthState = {
  token: string | null
  userId: number | null
  username: string | null
}

export const auth = writable<AuthState>({
  token: null,
  userId: null,
  username: null,
})

export function loadAuthFromStorage() {
  const token = localStorage.getItem('slate_token')
  const userId = localStorage.getItem('slate_user_id')
  const username = localStorage.getItem('slate_username')
  if (token && userId && username) {
    auth.set({ token, userId: Number(userId), username })
  }
}

export function saveAuth(token: string, userId: number, username: string) {
  localStorage.setItem('slate_token', token)
  localStorage.setItem('slate_user_id', String(userId))
  localStorage.setItem('slate_username', username)
  auth.set({ token, userId, username })
}

export function clearAuth() {
  localStorage.removeItem('slate_token')
  localStorage.removeItem('slate_user_id')
  localStorage.removeItem('slate_username')
  auth.set({ token: null, userId: null, username: null })
}

export function getToken(): string | null {
  return get(auth).token
}

export async function logout() {
  const url = getServerUrl()
  const token = getToken()
  if (url && token) {
    await fetch(`${url}/api/auth/logout`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${token}` },
    }).catch(() => {})
  }
  clearAuth()
}
