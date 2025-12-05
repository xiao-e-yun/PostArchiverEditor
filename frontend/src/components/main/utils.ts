import {reactiveChanges, useActiveItem} from '@/utils'
import type {CategoryType} from '@/category'
import {isEmpty} from 'lodash-es'
import type {Ref} from 'vue'

export interface CategoryData {
  id: number
  [key: string]: unknown
}

export interface UseCategoryActionsOptions<T extends CategoryData> {
  type: CategoryType
  data: Ref<T>
  proxyed: Ref<T & {_raw: T; changes: Partial<T>}>
  /** Transform changes before sending to API */
  transformChanges?: (changes: Partial<T>) => Record<string, unknown>
}

export function useCategoryActions<T extends CategoryData>(
  options: UseCategoryActionsOptions<T>,
) {
  const {type, data, proxyed, transformChanges} = options

  // Get singular name for display (remove trailing 's' or handle special cases)
  const displayName = type.endsWith('_metas')
    ? 'FileMeta'
    : type.charAt(0).toUpperCase() + type.slice(1, -1)

  const update = async () => {
    if (isEmpty(proxyed.value.changes)) return

    const id = data.value.id
    const body = transformChanges
      ? transformChanges(proxyed.value.changes)
      : proxyed.value.changes

    try {
      const res = await fetch(`/api/${type}/${id}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(body),
      })

      if (res.ok) {
        const updated = await res.json()
        Object.assign(data.value, updated)
        proxyed.value = reactiveChanges(data.value) as typeof proxyed.value
        alert(`${displayName} updated successfully`)
      } else {
        const error = await res.text()
        alert(`Error updating ${displayName.toLowerCase()}: ${error} ${res.statusText}`)
      }
    } catch (err) {
      alert(`Error updating ${displayName.toLowerCase()}: ${(err as Error).message}`)
    }
  }

  const remove = async () => {
    if (!confirm(`Are you sure you want to delete this ${displayName.toLowerCase()}?`)) return

    const id = data.value.id

    try {
      const res = await fetch(`/api/${type}/${id}`, {
        method: 'DELETE',
      })

      if (res.ok) {
        alert(`${displayName} deleted successfully`)
        useActiveItem().value = null
      } else {
        const error = await res.text()
        alert(`Error deleting ${displayName.toLowerCase()}: ${error}`)
      }
    } catch (err) {
      alert(`Error deleting ${displayName.toLowerCase()}: ${(err as Error).message}`)
    }
  }

  const discard = () => {
    proxyed.value.changes = {}
  }

  return {
    update,
    remove,
    discard,
  }
}
