import { useActiveItem, type Relations } from '@/utils'
import { type CategoryType } from '@/category'
import { isEmpty, startCase } from 'lodash-es'
import { inject, type Ref } from 'vue'
import type { Category, WithRelations } from '@/api'
import { toast } from 'vue-sonner'

export const dataSymbol = Symbol('category-data')
export const relationsSymbol = Symbol('category-relations')
export const injectData = <T extends Category>(): Ref<WithRelations<T> & { _raw: T, changes: Partial<T> }> => {
    const data = inject<Ref<WithRelations<T> & { _raw: T, changes: Partial<T> }>>(dataSymbol)
    if (!data) throw new Error('No category data provided')
    return data
}
export const injectRelations = () => {
    const relations = inject<Relations>(relationsSymbol)
    if (!relations) throw new Error('No category relations provided')
    return relations
}

export interface CategoryData {
    id: number
    [key: string]: unknown
}

export interface UseCategoryActionsOptions<T extends CategoryData> {
    type: CategoryType
    proxyed: Ref<T & { _raw: T; changes: Partial<T> }>
    /** Transform changes before sending to API */
    transformChanges?: (changes: Partial<T>) => Record<string, unknown>
}

export function useCategoryActions<T extends CategoryData>(
    options: UseCategoryActionsOptions<T>,
) {
    const { type, proxyed, transformChanges } = options

    // Get singular name for display (remove trailing 's' or handle special cases)
    const displayName = startCase(type.slice(0, -1))

    const update = async () => {
        if (isEmpty(proxyed.value.changes)) return

        const id = proxyed.value._raw.id
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
                proxyed.value._raw = { ...proxyed.value._raw, ...proxyed.value.changes }
                toast.success(`${displayName} updated successfully`)
            } else {
                const error = await res.text()
                toast.error(`Error updating ${displayName.toLowerCase()}`, {
                    description: `${error} ${res.statusText}`,
                })
            }
        } catch (err) {
            toast.error(`Error updating ${displayName.toLowerCase()}`, {
                description: (err as Error).message,
            })
        }
    }

    const remove = async () => {
        if (!confirm(`Are you sure you want to delete this ${displayName.toLowerCase()}?`)) return

        const id = proxyed.value._raw.id

        try {
            const res = await fetch(`/api/${type}/${id}`, {
                method: 'DELETE',
            })

            if (res.ok) {
                toast.success(`${displayName} deleted successfully`)
                useActiveItem().value = null
            } else {
                const error = await res.text()
                toast.error(`Error deleting ${displayName.toLowerCase()}`, {
                    description: error,
                })
            }
        } catch (err) {
            toast.error(`Error deleting ${displayName.toLowerCase()}`, {
                description: (err as Error).message,
            })
        }
    }

    return {
        update,
        remove,
    }
}
