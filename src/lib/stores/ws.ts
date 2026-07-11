import { getServerUrl, isConnected } from "$lib/api"
import { getToken } from "$lib/stores/auth"
import {
  columns,
  cardsByColumn,
  boards,
} from '$lib/stores/board'
import { get } from 'svelte/store'
import type { Card, Column } from '$lib/types'

let socket: WebSocket | null = null
let currentBoardId: number | null = null
let reconnectTimer: ReturnType<typeof setTimeout> | null = null
let intentionalClose = false

export function connectToBoard(boardId: number) {
  // already connected to the right board
  if (socket && socket.readyState === WebSocket.OPEN && currentBoardId === boardId) return

  disconnect()
  intentionalClose = false
  currentBoardId = boardId

  const url = getServerUrl()
  const token = getToken()
  if (!url || !token || !isConnected()) return

  const wsUrl = url.replace(/^http/, 'ws') + `/api/ws/${boardId}?token=${token}`
  open(wsUrl)
}

export function disconnect() {
  intentionalClose = true
  if (reconnectTimer) {
    clearTimeout(reconnectTimer)
    reconnectTimer = null
  }
  if (socket) {
    socket.close()
    socket = null
  }
  currentBoardId = null
}

function open(wsUrl: string) {
  socket = new WebSocket(wsUrl)

  socket.onopen = () => {
    console.debug('[slate ws] connected')
  }

  socket.onmessage = (e) => {
    try {
      const event = JSON.parse(e.data)
      handleEvent(event)
    } catch {
      console.warn('[slate ws] bad message]', e.data)
    }
  }

  socket.onclose = () => {
    socket = null
    if (!intentionalClose && currentBoardId !== null) {
      // reconnect afte 3 seconds
      reconnectTimer = setTimeout(() => {
        const url = getServerUrl()
	const token = getToken()
	if (url && token && currentBoardId !== null) {
	  const wsUrl = url.replace(/^http/, 'ws') + `/api/ws/${currentBoardId}?token=${token}`
	  open(wsUrl)
	}
      }, 3000)
    }
  }

  socket.onerror = () => {
    socket?.close()
  }
}

// event handlers

function handleEvent(event: any) {
  switch (event.type) {
    case 'card_created': return onCardCreated(event)
    case 'card_updated': return onCardUpdated(event)
    case 'card_deleted': return onCardDeleted(event)
    case 'card_moved': return onCardMoved(event)
    case 'column_created': return onColumnCreated(event)
    case 'column_renamed': return onColumnRenamed(event)
    case 'column_color': return onColumnColor(event)
    case 'column_deleted': return onColumnDeleted(event)
    case 'board_renamed': return onBoardRenamed(event)
  }
}

function onCardCreated(e: { card: Card}) {
  const card: Card = e.card
  cardsByColumn.update(m => ({
    ...m,
    [card.column_id]: [...(m[card.column_id] ?? []), card],
  }))
}

function onCardUpdated(e: { card: Partial<Card> & { id: number } }) {
  cardsByColumn.update(m => {
    const next = { ...m }
    for (const [colId, cards] of Object.entries(next)) {
      const idx = cards.findIndex(c => c.id === e.card.id)
      if (idx !== -1) {
        next[Number(colId)] = cards.map(c =>
          c.id === e.card.id ? { ...c, ...e.card } : c
	)
	break
      }
    }
    return next
  })
}

function onCardDeleted(e: { card_id: number, column_id: number }) {
  cardsByColumn.update(m => ({
    ...m,
    [e.column_id]: (m[e.column_id] ?? []).filter(c => c.id !== e.card_id),
  }))
}

function onCardMoved(e: {
  card_id: number
  from_column_id: number
  to_column_id: number
  position: number
}) {
  cardsByColumn.update(m => {
    const card = (m[e.from_column_id] ?? []).find(c => c.id === e.card_id)
    if (!card) return m

    const from = (m[e.from_column_id] ?? []).filter(c => c.id !== e.card_id)
    const to = e.from_column_id === e.to_column_id
      ? from
      : [...(m[e.to_column_id] ??[])]

    to.splice(e.position, 0, { ...card, column_id: e.to_column_id })

    return {
      ...m,
      [e.from_column_id]: from,
      [e.to_column_id]: to,
    }
  })
}

function onColumnCreated(e: { column: Column }) {
  columns.update(cols => {
    if (cols.some( c => c.id === e.column.id)) return cols
    return [...cols, e.column]
  })
  cardsByColumn.update(m => ({
    ...m,
    [e.column.id]: m[e.column.id] ?? [],
  }))
}

function onColumnRenamed(e: { column_id: number, name: string }) {
  columns.update(cols =>
    cols.map(c => c.id === e.column_id ? { ...c, name: e.name } : c)
  )
}

function onColumnColor(e: { column_id: number, color: string }) {
  columns.update(cols =>
    cols.map(c => c.id === e.column_id ? { ...c, color: e.color } : c)
  )
}

function onColumnDeleted(e: { column_id: number }) {
  columns.update(cols => cols.filter(c => c.id !== e.column_id))
  cardsByColumn.update(m => {
    const next = { ...m }
    delete next[e.column_id]
    return next
  })
}

function onBoardRenamed(e: { board_id: number, name: string }) {
  boards.update(bs =>
    bs.map(b => b.id === e.board_id ? { ...b, name: e.name } : b)
  )
}
