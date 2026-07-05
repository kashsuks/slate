import { writable, get } from 'svelte/store'
import type { Board, Column, Card } from '$lib/types'
import {
  apiGetBoards, apiCreateBoard, apiRenameBoard, apiDeleteBoard,
  apiGetColumns, apiCreateColumn, apiRenameColumn, apiUpdateColumnColor, apiDeleteColumn,
  apiGetCards, apiCreateCard, apiUpdateCard, apiDeleteCard, apiMoveCard,
} from '$lib/api'

export const boards = writable<Board[]>([])
export const activeBoardId = writable<number | null>(null)
export const columns = writable<Column[]>([])
export const cardsByColumn = writable<Record<number, Card[]>>({})

export async function loadBoards() {
  const result = await apiGetBoards()
  boards.set(result)
  if (result.length > 0 && get(activeBoardId) === null) {
    await selectBoard(result[0].id)
  }
}

export async function createBoard(name: string) {
  const board = await apiCreateBoard(name)
  if (!board) throw new Error('createBoard returned null')
  boards.update(b => [...b, board])
  await selectBoard(board.id)
}

export async function renameBoard(id: number, name: string) {
  await apiRenameBoard(id, name)
  boards.update(b => b.map(board => board.id === id ? { ...board, name } : board))
}

export async function deleteBoard(id: number) {
  await apiDeleteBoard(id)
  boards.update(b => b.filter(board => board.id !== id))
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

export async function selectBoard(id: number) {
  activeBoardId.set(id)
  const cols = await apiGetColumns(id)
  columns.set(cols)
  const cardMap: Record<number, Card[]> = {}
  for (const col of cols) {
    cardMap[col.id] = await apiGetCards(col.id)
  }
  cardsByColumn.set(cardMap)
}

export async function createColumn(boardId: number, name: string) {
  const col = await apiCreateColumn(boardId, name)
  if (col) {
    columns.update(c => [...c, col])
    cardsByColumn.update(m => ({ ...m, [col.id]: [] }))
  }
}

export async function deleteColumn(id: number) {
  await apiDeleteColumn(id)
  columns.update(c => c.filter(col => col.id !== id))
  cardsByColumn.update(m => {
    const next = { ...m }
    delete next[id]
    return next
  })
}

export async function createCard(columnId: number, title: string) {
  const card = await apiCreateCard(columnId, title)
  if (card) {
    cardsByColumn.update(m => ({
      ...m,
      [columnId]: [...(m[columnId] ?? []), card],
    }))
  }
}

export async function deleteCard(columnId: number, cardId: number) {
  await apiDeleteCard(cardId)
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
  // Optimistic update first so UI doesn't flicker
  cardsByColumn.update(m => {
    const from = (m[fromColumnId] ?? []).filter(c => c.id !== cardId)
    const card = (m[fromColumnId] ?? []).find(c => c.id === cardId)
      ?? (m[toColumnId] ?? []).find(c => c.id === cardId)
    if (!card) return m
    const to = fromColumnId === toColumnId
      ? from
      : [...(m[toColumnId] ?? [])]
    to.splice(newIndex, 0, { ...card, column_id: toColumnId })
    return { ...m, [fromColumnId]: from, [toColumnId]: to }
  })

  await apiMoveCard(cardId, toColumnId, newIndex)

  // Reconcile with server/db
  const updatedFrom = await apiGetCards(fromColumnId)
  const updatedTo = fromColumnId === toColumnId
    ? updatedFrom
    : await apiGetCards(toColumnId)
  cardsByColumn.update(m => ({
    ...m,
    [fromColumnId]: updatedFrom,
    [toColumnId]: updatedTo,
  }))
}

export async function renameColumn(columnId: number, name: string) {
  await apiRenameColumn(columnId, name)
  columns.update(cols =>
    cols.map(c => c.id === columnId ? { ...c, name } : c)
  )
}

export async function updateColumnColor(columnId: number, color: string) {
  await apiUpdateColumnColor(columnId, color)
  columns.update(cols =>
    cols.map(c => c.id === columnId ? { ...c, color } : c)
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
  await apiUpdateCard(cardId, title, description, priority, due_date)
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
  title: string,
) {
  const existing = get(cardsByColumn)[columnId]?.find(c => c.id === cardId)
  if (!existing) return
  await apiUpdateCard(cardId, title, existing.description, existing.priority, existing.due_date)
  cardsByColumn.update(m => ({
    ...m,
    [columnId]: (m[columnId] ?? []).map(c =>
      c.id === cardId ? { ...c, title } : c
    ),
  }))
}
