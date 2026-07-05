import { invoke } from '@tauri-apps/api/core'

// return the server url from localStorage, or null if standalone mode
export function getServerUrl(): string | null {
  const url = localStorage.getItem('serverUrl')
  return url && url.trim() !== '' ? url.trim() : null
}

export function isTauri(): boolean {
  return '__TAURI__' in window
}

let _connected = false
export function isConnected(): boolean {
  return _connected
}

// ping the serverto see if its reachable
// call this on app startup and when the user saves a new server URL
export async function checkConnection(): Promise<boolean> {
  const url = getServerUrl()
  if (!url) {
    _connected = false
    return false
  }
  try {
    const res = await fetch(`${url}/api/boards`, { signal: AbortSignal.timeout(3000) })
    _connected = res.ok
  } catch {
    _connected = false
  }
  return _connected
}

// code dispatcher - use fetch() if connected, invoke() if standalone
async function call<T>(
  tauriCmd: string,
  tauriArgs: Record<string, unknown>,
  httpMethod: string,
  path: string,
  body?: unknown,
): Promise<T> {
  const url = getServerUrl()

  if (url && _connected) {
    // connected mode where we can talk to the server
    const res = await fetch(`${url}/api${path}`, {
      method: httpMethod,
      headers: body ? { 'Content-Type': 'application/json' } : {},
      body: body ? JSON.stringify(body) : undefined,
    })
    if (!res.ok) throw new Error(`Server error: ${res.status}`)
    // 204 No Content responses have no body
    if (res.status === 204) return undefined as T
    return res.json()
  }

  // standalone mode
  return invoke<T>(tauriCmd, tauriArgs)
}

// Boards related stuff

export function apiGetBoards() {
  return call<import('./types').Board[]>(
    'get_boards', {},
    'GET', '/boards'
  )
}

export function apiCreateBoard(name: string) {
  return call<import('./types').Board>(
    'create_board', { name },
    'POST', '/boards', { name }
  )
}

export function apiRenameBoard(id: number, name: string) {
  return call<void>(
    'rename_board', { id, name },
    'PUT', `/boards/${id}/rename`, { name }
  )
}

export function apiDeleteBoard(id: number) {
  return call<void>(
    'delete_board', { id },
    'DELETE', `/boards/${id}`
  )
}

// columns related stuff

export function apiGetColumns(boardId: number) {
  return call<import('./types').Column[]>(
    'get_columns', { boardId },
    'GET', `/columns/${boardId}`,
  )
}

export function apiCreateColumn(boardId: number, name: string) {
  return call<import('./types').Column>(
    'create_column', { boardId, name },
    'POST', '/columns', { board_id: boardId, name }
  )
}

export function apiRenameColumn(id: number, name: string) {
  return call<void>(
    'rename_column', { id, name },
    'PUT', `/columns/${id}/rename`, { name }
  )
}

export function apiUpdateColumnColor(id: number, color: string) {
  return call<void>(
    'update_column_color', { id, color },
    'PUT', `/columns/${id}/color`, { color }
  )
}

export function apiDeleteColumn(id: number) {
  return call<void>(
    'delete_column', { id },
    'DELETE', `/columns/${id}`
  )
}

// cards related stuff

export function apiGetCards(columnId: number) {
  return call<import('./types').Card[]>(
    'get_cards', { columnId },
    'GET', `/cards/${columnId}`
  )
}

export function apiCreateCard(columnId: number, title: string) {
  return call<import('./types').Card>(
    'create_card', { columnId, title },
    'POST', '/cards', { column_id: columnId, title }
  )
}

export function apiUpdateCard(
  id: number,
  title: string,
  description: string | null,
  priority: string,
  due_date: string | null,
) {
  return call<void>(
    'update_card', { id, title, description, priority, dueDate: due_date },
    'PUT', `/cards/${id}`, { title, description, priority, due_date }
  )
}

export function apiDeleteCard(id: number) {
  return call<void>(
    'delete_card', { id },
    'DELETE', `/cards/${id}`
  )
}

export function apiMoveCard(id: number, columnId: number, position: number) {
  return call<void>(
    'move_card', { id, columnId, position },
    'PUT', `/cards/${id}/move`, { column_id: columnId, position }
  )
}

// config related stuff

export function apiGetConfig(key: string) {
  return call<string | null>(
    'get_config', { key },
    'GET', `/config/${key}`
  )
}

export function apiSetConfig(key: string, value: string) {
  return call<void>(
    'set_config', { key, value },
    'POST', '/config', { key, value },
  )
}
