<script setup lang="ts">
import { getUrlWithParams, useActiveItem, useActiveTab, useRelations } from '@/utils'
import { Input } from '../ui/input'
import { refDebounced, useInfiniteScroll, useSessionStorage, useVirtualList } from '@vueuse/core'
import { ref, useTemplateRef, watch } from 'vue'
import { Alert, AlertDescription, AlertTitle } from '../ui/alert'
import type { Category, ListResponse, WithRelations } from '@/api'
import { ScrollArea } from '../ui/scroll-area'
import { LoaderCircleIcon } from 'lucide-vue-next'
import { getCategoryName, getCategoryThumb, matchCategory } from '@/category'

const activeTab = useActiveTab()
const activeItem = useActiveItem()

const search = useSessionStorage('editor-search', '')
const searchDebounced = refDebounced(search, 250)

let page = ref<number | null>(0)
const error = ref<[string, string] | null>(null)
const list = ref<
  {
    id: number
    name: string
    thumb: string | null
    tags: string | null
  }[]
>([])
const listEl = useTemplateRef<HTMLDivElement>('lastEl')
const { reset, isLoading } = useInfiniteScroll(
  listEl,
  async () => {
    if (page === null) return
    const type = activeTab.value

    error.value = null
    let response: Response
    try {
      response = await fetch(
        getUrlWithParams(`/api/${type}`, {
          search: searchDebounced.value ?? undefined,
          page: page.value || 0,
        }),
      )
    } catch (e) {
      error.value = ['Network Error', (e as Error).message]
      page.value = null
      return
    }

    let result: WithRelations<ListResponse<Category>>
    try {
      result = await response.json()
      page.value = result.list.length === 20 ? (page.value || 0) + 1 : null
    } catch (e) {
      error.value = ['Error parsing response', (e as Error).message]
      page.value = null
      return
    }

    const relations = useRelations(result)
    list.value.push(
      ...result.list.map((item) => {
        const thumb = getCategoryThumb(type, item, relations)
        const name = getCategoryName(type, item, false)

        const tags = matchCategory(type, item, {
          post: ({ platform }) => platform ? (":" + relations.platforms.get(platform)!.name) : null,
          tag: ({ platform }) => platform ? (":" + relations.platforms.get(platform)!.name) : null,
          fileMeta: ({ post }) => post.toString(),
          collection: () => null,
          platform: () => null,
          author: () => null,
        })

        return {
          id: item.id,
          name,
          thumb,
          tags,
        }
      }),
    )
  },
  {
    distance: 20,

    canLoadMore: () => page.value !== null,
  },
)

watch([activeTab, searchDebounced], resetList)
function resetList() {
  list.value.length = 0
  page.value = 0
  reset()
}

const {
  list: vList,
  containerProps,
  wrapperProps,
} = useVirtualList(list, { itemHeight: 61, overscan: 5 })
</script>

<template>
  <aside class="w-32 md:w-60 border-r bg-secondary h-full overflow-hidden">
    <Input class="rounded-none border-none border-b p-1" placeholder="Search..." v-model="search" />
    <ScrollArea class="h-[calc(100%-36px)]">
      <Alert v-if="error" variant="destructive" class="mx-auto my-2 w-fit">
        <AlertTitle>{{ error[0] }}</AlertTitle>
        <AlertDescription>{{ error[1] }}</AlertDescription>
      </Alert>
      <div v-else-if="list.length === 0" class="p-4">No items found.</div>
      <div v-bind="containerProps">
        <ul v-bind="wrapperProps" class="m-0 p-0 list-none">
          <li
            v-for="{ data: item, index } in vList"
            :key="index"
            @click="activeItem = { id: item.id, type: activeTab }"
            class="flex hover:bg-input/30 cursor-pointer border-b relative p-2 flex-col items-start"
            :class="{
              'bg-input/30': activeItem?.id === item.id && activeItem?.type === activeTab,
            }"
          >
            <div class="z-10 w-full">
              <h2 class="truncate">{{ item.name }}</h2>
              <div class="flex items-start gap-1">
                <span class="opacity-50 text-sm">#{{ item.id }}</span>
                <span v-if="item.tags" class="opacity-50 text-sm bg-input px-2 rounded-md">{{ item.tags }}</span>
              </div>
            </div>
            <img
              v-if="item.thumb"
              :src="item.thumb + '&w=128&h=128'"
              class="absolute h-full aspect-square object-cover right-0 top-0 mask-l-from-black opacity-50"
            />
          </li>
        </ul>
        <div ref="lastEl" />
      </div>
      <div v-if="isLoading && page !== null && !error" class="p-4">
        <LoaderCircleIcon class="animate-spin mx-auto" :size="24" />
        Loading...
      </div>
    </ScrollArea>
  </aside>
</template>
