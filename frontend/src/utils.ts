import type { FileMeta } from "@api/FileMeta";
import type { WithRelations } from "@api/WithRelations";
import { createGlobalState, extendRef, reactiveComputed, useSessionStorage, useUrlSearchParams } from "@vueuse/core";
import { cloneDeep, isNumber, toString } from "lodash-es";
import { computed, reactive, ref, shallowRef, toValue, triggerRef, watch, type MaybeRefOrGetter, type Ref } from "vue";
import { CategoryType } from "./category";
import type { Author, Collection, Platform, Tag } from "post-archiver";

const urlParams = useUrlSearchParams("history")
export const useActiveTab = () => computed({
  get() {
    const tab = toString(urlParams.tab);
    return [
      CategoryType.Post,
      CategoryType.Collection,
      CategoryType.Author,
      CategoryType.Tag,
      CategoryType.Platform,
      CategoryType.FileMeta,
    ].includes(tab as CategoryType) ? tab as CategoryType : CategoryType.Post;
  },
  set(val) {
    if (val === CategoryType.Post) return urlParams.tab = [];
    urlParams.tab = val;
  }
})
export const useActiveItem = () => computed({
  get() {
    if (!urlParams.item) return null;
    const decoded = toString(urlParams.item).split("-")
    if (!decoded || decoded.length !== 2) return null;
    const [rawType, rawId] = decoded as [string, string];
    const type = ({
      p: CategoryType.Post,
      c: CategoryType.Collection,
      a: CategoryType.Author,
      t: CategoryType.Tag,
      pl: CategoryType.Platform,
      f: CategoryType.FileMeta,
    } as Record<string, CategoryType>)[rawType] ?? null;
    let id = null;
    try { id = parseInt(rawId) } catch { return null; }
    if (!type || isNaN(id)) return null;
    return { type, id };
  },
  set(val) {
    if (!val) return urlParams.item = [];
    const rawType = ({
      posts: "p",
      collections: "c",
      authors: "a",
      tags: "t",
      platforms: "pl",
      file_metas: "f",
    } as Record<string, string>)[val.type];
    if (!rawType) return urlParams.item = [];
    urlParams.item = `${rawType}-${val.id}`;
  }
})
export const useSettingsTab = () => useSessionStorage("settings", false)

export const isImage = (mime?: string) => mime && mime.startsWith("image/");

export const useGlobalRelations = createGlobalState(() => reactive({
  authors: new Map<number, Author>(),
  collections: new Map<number, Collection>(),
  platforms: new Map<number, Platform>(),
  tags: new Map<number, Tag>(),
  file_metas: new Map<number, FileMeta>(),
}))

export function useRelations<T>(
  data: MaybeRefOrGetter<WithRelations<T> | null | undefined>,
) {
  return reactiveComputed(() => {
    const relations = toValue(data) ?? ({} as WithRelations<T>);

    // insert into global relations
    const global = useGlobalRelations();
    for (const [key, globalMap] of Object.entries(global))
      // @ts-expect-error: same structure
      for (const item of relations[key] ?? [])
        globalMap.set(item.id, item);

    return {
      ...global,
      fileMetaPath(id?: number | null): string | undefined {
        const fileMeta = isNumber(id) ? global.file_metas.get(id) : undefined;
        return fileMeta && getFileMetaPath(fileMeta);
      },
      merge(rhs: WithRelations<any>): void {
        for (const [key, globalMap] of Object.entries(global))
          for (const item of rhs[key] ?? []) globalMap.set(item.id, item);
      },
      clear(): void {
        for (const globalMap of Object.values(global)) globalMap.clear();
      }
    };
  });
}
export type Relations = {
  authors: Map<number, Author>;
  collections: Map<number, Collection>;
  platforms: Map<number, Platform>;
  tags: Map<number, Tag>;
  file_metas: Map<number, FileMeta>;
  fileMetaPath(id?: number | null): string | undefined;
  merge(rhs: WithRelations<any>): void;
  /**
   * Clear all relations.
   * (WARNING: This will clear the global relations store)
   */
  clear(): void;
};

export function getFileMetaPath(fileMeta: FileMeta, raw = false): string {
  const isImage = fileMeta.mime.startsWith("image/");
  const base = isImage ? "images" : "resource";
  const suffix = isImage && !raw ? "?ce" : "";
  return `/${base}/${Math.floor(fileMeta.post / 2048)}/${fileMeta.post % 2048}/${fileMeta.filename}${suffix}`;
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

export const reactiveChanges = <T extends Object>(raw: T) => {
  const temp = shallowRef() as Ref<T & { _raw: T, changes: Partial<T> }>;
  temp.value = new Proxy({
    _raw: cloneDeep(raw),
    changes: {} as Partial<T>
  }, {
    get(self, prop: string) {
      if ((['_raw', 'changes']).includes(prop)) return self[prop as keyof typeof self];
      if (prop in self.changes) return self.changes[prop as keyof T];
      return self._raw[prop as keyof T];
    },
    set(self, prop: string, value) {
      if (prop === '_raw') {
        self[prop] = value;
        self.changes = {};
      } else if (prop === "changes") {
        self[prop] = value;
      } else {
        self.changes[prop as keyof T] = value;
      }
      triggerRef(temp);
      return true;
    }
  }) as unknown as T & { _raw: T, changes: Partial<T> };
  return temp
}
