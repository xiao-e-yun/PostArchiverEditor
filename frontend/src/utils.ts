import type {FileMeta} from "@api/FileMeta";
import type {WithRelations} from "@api/WithRelations";
import {extendRef, reactiveComputed, useLocalStorage, useSessionStorage} from "@vueuse/core";
import {cloneDeep} from "lodash-es";
import {ref, toValue, watch, type MaybeRefOrGetter, type Ref} from "vue";

export const useActiveTab = () => useLocalStorage<string>('activeTab', 'posts');
export const useActiveItem = () => useSessionStorage<{
  type: string
  id: number
} | null>('editor-active-item', null, {
  serializer: {
    read: (v) => v ? JSON.parse(v) : null,
    write: (v) => v ? JSON.stringify(v) : 'null'
  }
})

export function useRelations<T>(
  data: MaybeRefOrGetter<WithRelations<T> | null | undefined>,
) {
  return reactiveComputed(() => {
    const relations = toValue(data) ?? ({} as WithRelations<T>);

    const toMap = <T extends {id: number}>(values?: T[]) =>
      new Map(values ? values.map((v) => [v.id, v]) : []);
    const maps = {
      authors: toMap(relations.authors),
      collections: toMap(relations.collections),
      platforms: toMap(relations.platforms),
      tags: toMap(relations.tags),
      fileMetas: toMap(relations.file_metas),
    };

    return {
      ...maps,
      fileMetaPath(id: number): string | undefined {
        const fileMeta = maps.fileMetas.get(id);
        return fileMeta && getFileMetaPath(fileMeta);
      },
    };
  });
}

export function getFileMetaPath(fileMeta: FileMeta) {
  const base = fileMeta.mime.startsWith("image/") ? "images" : "resource";
  return `/${base}/${Math.floor(fileMeta.post / 2048)}/${fileMeta.post % 2048}/${fileMeta.filename}`
}

export function getUrl(url: string | URL): URL {
  return new URL(url, location.origin);
}

type UrlBaseParam = string | number | undefined;
export type UrlParams = Record<
  string,
  MaybeRefOrGetter<UrlBaseParam | UrlBaseParam[]>
>;
export function getUrlWithParams(url: string | URL, params: UrlParams): string {
  const urlObj = getUrl(url);
  Object.entries(params).forEach(([key, rawValue]) => {
    const value = toValue(rawValue);
    if (value === undefined) return;
    const values = Array.isArray(value) ? value : [value];
    for (const value of values) {
      if (value === "") continue;
      urlObj.searchParams.append(key, String(value));
    }
  });
  return urlObj.toString();
}

export function urlParamIntoString(param: string[] | string): string;
export function urlParamIntoString(param: undefined): undefined;
export function urlParamIntoString(param: string[] | string | undefined) {
  return Array.isArray(param) ? param[0] : param;
}

export function commitRef<T>(staged: Ref<T>) {
  const untracked = ref(cloneDeep(staged.value));
  watch(staged, () => (untracked.value = cloneDeep(staged.value)));
  return extendRef(untracked, {
    commit() {
      staged.value = cloneDeep(untracked.value);
    },
  });
}
