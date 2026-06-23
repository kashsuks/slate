import { invoke } from '@tauri-apps/api/core'
import { writable, get } from 'svelte/store'
import type { Board, Column, Card } from '$lib/types'

export const boards = writable<Board[]>([])
export const activeBoardId = writable<number | null>(null)
export const columns = writable<Column[]>([])
export const cardsByColumn = writable<Record<number, Card[]>>({})

export async function loadBoards() {
  const result = await invoke<Board[]>('get_boards')
  boards.set(result)
  if (result.length > 0 && get(activeBoardId) === null) {
    await selectBoard(result[0].id)
  }
}

export async function createBoard(name: string) {
  console.log('[createBoard] invoking with name:', name)
  const board = await invoke<Board>('create_board', { name })
  console.log('[createBoard] result:', board)
  if (!board) throw new Error('create_board returned null — check Rust logs')
  boards.update(b => [...b, board])
  await selectBoard(board.id)
  console.log('[createBoard] done, activeBoardId:', board.id)
}

export async function selectBoard(id: number) {
  activeBoardId.set(id)
  const cols = await invoke<Column[]>('get_columns', { boardId: id })
  columns.set(cols)
  const cardMap: Record<number, Card[]> = {}
  for (const col of cols) {
    cardMap[col.id] = await invoke<Card[]>('get_cards', { columnId: col.id })
  }
  cardsByColumn.set(cardMap) // asign the cards based on column
}

export async function createColumn(boardId: number, name: string) {
  const col = await invoke<Column>('create_column', { boardId, name })
  if (col) {
    columns.update(c => [...c, col])
    cardsByColumn.update(m => ({...m, [col.id]: [] }))
  }
}

export async function createCard(columnId: number, title: string) {
  const card = await invoke<Card>('create_card', { columnId, title })
  if (card) {
    cardsByColumn.update(m => ({
      ...m,
      [columnId]: [...(m[columnId] ?? []), card],
    }))
  }
}

export async function deleteCard(columnId: number, cardId: number) {
  await invoke('delete_card', { id: cardId })
  cardsByColumn.update(m => ({
    ...m,
    [columnId]: (m[columnId] ?? []).filter(c => c.id !== cardId),
  }))
}

export async function renameColumn(columnId: number, name: string) {
  await invoke('rename_column', { id: columnId, name })
  columns.update(cols =>
    cols.map(c => c.id === columnId ? {...c, name } : c)
  )
}

export async function renameCard(columnId: number, cardId: number, title: string) {
  await invoke('update_card', {
    id: cardId,
    title,
    description: null,
    priority: 'none',
    due_date: null,
  })
  cardsByColumn.update(m => ({
    ...m,
    [columnId]: (m[columnId] ?? []).map(c =>
      c.id === cardId ? { ...c, title } : c
    ),
  }))
}
