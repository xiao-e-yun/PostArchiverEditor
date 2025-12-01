import { match, P } from "ts-pattern";
import type { Category, WithRelations } from "./api";
import type { Author, Collection, FileMeta, Platform, Post, Tag } from "post-archiver";
import { getFileMetaPath, useRelations, type Relations } from "./utils";

export enum CategoryType {
    Post = "posts",
    Collection = "collections",
    Author = "authors",
    Tag = "tags",
    Platform = "platforms",
    FileMeta = "file_metas",
}

type MatchCategoryCallbacks<T> = {
    post: (post: Post) => T,
    author: (author: Author) => T,
    tag: (tag: Tag) => T,
    collection: (collection: Collection) => T,
    platform: (platform: Platform) => T,
    fileMeta: (fileMeta: FileMeta) => T,
};

export function matchCategory<T>(type: CategoryType.Post, category: Post, callbacks: MatchCategoryCallbacks<T>): T
export function matchCategory<T>(type: CategoryType.Author, category: Author, callbacks: MatchCategoryCallbacks<T>): T
export function matchCategory<T>(type: CategoryType.Collection, category: Collection, callbacks: MatchCategoryCallbacks<T>): T
export function matchCategory<T>(type: CategoryType.Tag, category: Tag, callbacks: MatchCategoryCallbacks<T>): T
export function matchCategory<T>(type: CategoryType.Platform, category: Platform, callbacks: MatchCategoryCallbacks<T>): T
export function matchCategory<T>(type: CategoryType.FileMeta, category: FileMeta, callbacks: MatchCategoryCallbacks<T>): T
export function matchCategory<T>(type: CategoryType, category: Category, callbacks: MatchCategoryCallbacks<T>): T
export function matchCategory<T>(type: CategoryType, category: Category, callbacks: MatchCategoryCallbacks<T>): T {
    return match(type)
        // Post: has title, content, comments, published
        .with(CategoryType.Post, () => callbacks.post(category as Post))
        .with(CategoryType.FileMeta, () => callbacks.fileMeta(category as FileMeta))
        .with(CategoryType.Author, () => callbacks.author(category as Author))
        .with(CategoryType.Collection, () => callbacks.collection(category as Collection))
        .with(CategoryType.Platform, () => callbacks.platform(category as Platform))
        .with(CategoryType.Tag, () => callbacks.tag(category as Tag))
        .exhaustive();
}

export const getCategoryName = (type: CategoryType, category: Category, withSuffix: boolean): string => {
    const suffix = (suffix: string) => withSuffix ? suffix : "";
    return matchCategory(type, category, {
        post: (c) => c.title || `<${c.id}>`,
        author: (c) => suffix("@") + c.name,
        collection: (c) => suffix(".") + c.name,
        platform: (c) => suffix(":") + c.name,
        tag: (c) => suffix("#") + c.name,
        fileMeta: (c) => c.filename,
    })
}

export const getCategoryThumb = (type: CategoryType, category: Category, relations: Relations): string | null => matchCategory<string | null>(type, category, {
    fileMeta: (c) => c.mime.startsWith('image/') ? getFileMetaPath(c) : null,
    // Other types: look up thumbnail from relations
    post: (c) => relations.fileMetaPath(c.thumb) ?? null,
    author: (c) => relations.fileMetaPath(c.thumb) ?? null,
    collection: (c) => relations.fileMetaPath(c.thumb) ?? null,
    // Fallback to null for types without thumbnails
    tag: () => null,
    platform: () => null,
})