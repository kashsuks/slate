export type Board = {
  id: number
  name: string
  position: number
  created_at: string
}

export type Column = {
  id: number
  board_id: number
  name: string
  position: number
  color: string
}

export type Card = {
  id: number
  column_id: number
  title: string
  description: string | null
  priority: string
  due_date: string | null
  position: number
  created_at: string
}
