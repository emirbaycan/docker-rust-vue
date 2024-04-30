export type UserRole = 1 | 2 | 3 | 4;

export type User = {
  id: number
  fullname: string
  email: string
  username: string
  role: UserRole
  avatar: string
  notes: string
  active: boolean
}
