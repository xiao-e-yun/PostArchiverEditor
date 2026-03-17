import type {Editor} from "@tiptap/vue-3"
import type {ShallowRef} from "vue"
import {defineAsyncComponent} from "vue"
import {type InjectionKey} from "vue"

export { default as Editor } from "./Editor.vue"
export const AsyncEditor = defineAsyncComponent(() => import("./Editor.vue"))

export const key = Symbol("editor") as InjectionKey<ShallowRef<Editor | undefined>>
