
export * from "@api/Comment"
export * from "@api/Content"
export * from "@api/ListResponse"
export * from "@api/PostResponse"
export * from "@api/WithRelations"

import type { Post, Author, Collection, Platform, Tag, FileMeta } from "post-archiver"

export type Category = Post | Author | Collection | Platform | Tag | FileMeta