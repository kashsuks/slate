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

export async function renameBoard(id: number, name: string) {
  await invoke('rename_board', { id, name })
  boards.update(b => b.map(board => board.id === id ? { ...board, name } : board))
}

export async function deleteBoard(id: number) {
  await invoke('delete_board', { id })
  boards.update(b => b.filter(board => board.id !== id))
  // if we deleted the active board, then select
  // the first remaining one
  const remaining = get(boards)
  if (get(activeBoardId) === id) {
    if (remaining.length > 0) {
      await selectBoard(remaining[0].id)
    } else {
      activeBoardId.set(null)
      columns.set([])
      cardsByColumn.set({})
    }
  }
}

export async function deleteColumn(id: number) {
  await invoke('delete_column', { id })
  columns.update(c => c.filter(col => col.id !== id))
  cardsByColumn.update(m => {
    const next = { ...m }
    delete next[id]
    return next
  })
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

export async function moveCard(
  cardId: number,
  fromColumnId: number,
  toColumnId: number,
  newIndex: number,
) {
  // move the card in the store first
  // so the ui doesnt flicker when db call is happening
  cardsByColumn.update(m => {
    const from = (m[fromColumnId] ?? []).filter(c => c.id !== cardId)
    const card = (m[fromColumnId] ?? []).find(c => c.id === cardId)
      ?? (m[toColumnId] ?? []).find(c => c.id === cardId)
    if (!card) return m
    const to = fromColumnId === toColumnId
      ? from
      : [...(m[toColumnId] ?? [])]
    to.splice(newIndex, 0, {...card, column_id: toColumnId })
    return {
      ...m,
      [fromColumnId]: from,
      [toColumnId]: to,
    }
  })

  // persist to db
  await invoke('move_card', {
    id: cardId,
    columnId: toColumnId,
    position: newIndex,
  })

  // check with db again to get clean pos
  const updatedForm = await invoke<Card[]>('get_cards', { columnId: fromColumnId })
  const updatedTo = fromColumnId === toColumnId
    ? updatedForm
    : await invoke<Card[]>('get_cards', { columnId: toColumnId })
  cardsByColumn.update(m => ({
    ...m,
    [fromColumnId]: updatedForm,
    [toColumnId]: updatedTo,
  }))
}

export async function renameColumn(columnId: number, name: string) {
  await invoke('rename_column', { id: columnId, name })
  columns.update(cols =>
    cols.map(c => c.id === columnId ? {...c, name } : c)
  )
}

export async function updateCard(
  columnId: number,
  cardId: number,
  title: string,
  description: string | null,
  priority: string,
  due_date: string | null,
) {
  await invoke('update_card', { id: cardId, title, description, priority, dueDate: due_date })
  cardsByColumn.update(m => ({
    ...m,
    [columnId]: (m[columnId] ?? []).map(c =>
      c.id === cardId ? { ...c, title, description, priority, due_date } : c
    ),
  }))
}

export async function renameCard(
  columnId: number, 
  cardId: number, 
  title: string
) {

  const existing = get(cardsByColumn)[columnId]?.find(c => c.id === cardId)
  if (!existing) return
  
  await invoke('update_card', {
    id: cardId,
    title,
    description: existing.description,
    priority: existing.priority,
    due_date: existing.due_date,
  })
  cardsByColumn.update(m => ({
    ...m,
    [columnId]: (m[columnId] ?? []).map(c =>
      c.id === cardId ? { ...c, title } : c
    ),
  }))
}
