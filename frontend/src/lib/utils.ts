import type {ClassValue} from "clsx"
import {clsx} from "clsx"
import {twMerge} from "tailwind-merge"
import type {FileMeta, PlatformId, Platform, FileMetaId} from "post-archiver"
import type {WithRelations} from "@/types"
import {computed, ref, toRaw, toValue, watch, type MaybeRefOrGetter, type Ref} from "vue"
import {assign, capitalize, cloneDeep, isEqual} from "lodash-es"
import {toast} from "vue-sonner"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function parseFileMetaToUrl(fileMeta: FileMeta) {
  const [main, secondary] = [Math.floor(fileMeta.post / 2048), fileMeta.post % 2048]
  return getFileMetaApi(fileMeta) + `/${main}/${secondary}/${fileMeta.filename}`
}

export function getFileMetaApi(fileMeta: FileMeta) {
  return fileMeta.mime.startsWith("image/") ? "/images" : "/resource"
}

export function useFileMetas<T>(
  response: MaybeRefOrGetter<WithRelations<T> | null>,
  defaults: MaybeRefOrGetter<Record<FileMetaId, FileMeta>> = {}
) {
  return computed(() => {
    const $response = toValue(response)
    if (!$response) return {}

    const fileMetas = $response.file_metas

    return assign(fileMetas.reduce((acc, fileMeta) => {
      acc[fileMeta.id] = fileMeta
      return acc
    }, {} as Record<number, FileMeta>
    ), toValue(defaults))
  })
}

export function usePlatforms<T>(
  response: MaybeRefOrGetter<WithRelations<T> | null>,
  defaults: MaybeRefOrGetter<Record<PlatformId, Platform>> = {}
) {
  return computed(() => {
    const $response = toValue(response)
    if (!$response) return {}

    const platforms = $response.platforms
    return assign(
      platforms.reduce((acc, platfrom) => {
        acc[platfrom.id] = platfrom
        return acc
      }, {} as Record<number, Platform>)
      , toValue(defaults))
  })
}

export const useChangeable = <T>(obj: Ref<T | null>) => {
  const changeable = ref<T | null>(null)
  watch(obj, obj => {if (obj) changeable.value = cloneDeep(obj)})
  return changeable
}

export const differenceObject = <T extends object>(source: T, changed: T): Partial<T> => {
  const difference: Partial<T> = {}
  for (const key in source) {
    const sourceValue = toRaw(source[key])
    const changedValue = toRaw(changed[key])
    if (isEqual(sourceValue, changedValue)) continue
    difference[key] = changed[key]
  }
  return difference
}

export const useDifference = <T>(source: MaybeRefOrGetter<T>, changed: MaybeRefOrGetter<T>) => computed(() => {
  if (!toValue(source) || !toValue(changed)) return null;
  const difference = differenceObject(toValue(source) as object, toValue(changed) as object)
  return Object.keys(difference).length === 0 ? null : difference as Partial<T>
})

export const toastResponse = (result: Promise<Response>, successMessage: string, errorMessage: string) => {
  return result.then(res => {
    if (res.ok) {
      toast.success(successMessage)
      return true
    }
    else {
      console.error(res)
      toast.error(errorMessage)
      return false
    }
  }).catch(err => {
    console.error(err);
    toast.error("Failed to update post")
    return false
  })
}

export const useCommonSaveAPI = <T>(name: string, url: MaybeRefOrGetter<string>, raw: Ref<T>, changed: Ref<T>, difference: Ref<Partial<T>>) => {
  const successMessage = `${capitalize(name)} update successfully`
  const errorMessage = `Failed to update ${name}`

  return async () => {
    const diff = difference.value;
    if (!diff) return

    console.log(`Save ${name}:`, diff)
    if (
      await toastResponse(fetch(toValue(url), {
        method: "PATCH",
        body: JSON.stringify(diff),
        headers: {"Content-Type": "application/json"},
      }),
        successMessage,
        errorMessage
      )
    ) raw.value = changed.value;
  }
}

export const useRemoveAPI = (name: string, url: MaybeRefOrGetter<string>) => {
  const successMessage = `${capitalize(name)} deleted successfully`
  const errorMessage = `Failed to delete ${name}`
  return async () => {
    if (
      await toastResponse(fetch(toValue(url), {
        method: "DELETE",
      }),
        successMessage,
        errorMessage
      )
    ) window.history.back()
  }
}
