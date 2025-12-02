<script setup lang="ts">
import type {Category, ListResponse, WithRelations} from '@/api'
import {CategoryType, getCategoryName} from '@/category'
import {getUrlWithParams, type Relations} from '@/utils'
import {refDebounced} from '@vueuse/core'
import {CheckIcon, ChevronDownIcon, LoaderCircleIcon} from 'lucide-vue-next'
import {ListboxContent, ListboxFilter, ListboxItem, ListboxRoot} from 'reka-ui'
import {computed, ref, watch} from 'vue'
import {Button} from '../ui/button'
import {Popover, PopoverAnchor, PopoverContent, PopoverTrigger} from '../ui/popover'
import {ScrollArea} from '../ui/scroll-area'
import {
  TagsInput,
  TagsInputInput,
  TagsInputItem,
  TagsInputItemDelete,
  TagsInputItemText,
} from '../ui/tags-input'
import { isEmpty } from 'lodash-es'

const props = defineProps<{
  type: CategoryType
  relations: Relations
}>()

const model = defineModel<Category[]>({required: true})

// Convert model to string array for TagsInput
const modelIds = computed({
  get: () => model.value.map((item) => item.id.toString()),
  set: (ids: string[]) => {
    // Filter model to only include items with matching ids
    model.value = model.value.filter((item) => ids.includes(item.id.toString()))
  },
})

// Search state
const searchOpen = ref(false)
const searchQuery = ref('')
const searchQueryDebounced = refDebounced(searchQuery, 250)
const searchResults = ref<Category[]>([])
const isSearching = ref(false)

// Search API call
async function search() {
  if (!searchQueryDebounced.value.trim()) {
    searchResults.value = []
    return
  }

  try {
    const response = await fetch(
      getUrlWithParams(`/api/${props.type}`, {
        search: searchQueryDebounced.value,
      }),
    )
    const result: WithRelations<ListResponse<Category>> = await response.json()
    searchResults.value = result.list
    props.relations.merge(result)

    for (const item of result.list) {
      // @ts-expect-error enum index
      props.relations[props.type].set(item.id, item)
    }
  } catch (e) {
    console.error('Search failed:', e)
    searchResults.value = []
  }
  isSearching.value = false
}

// Watch debounced search query
watch(searchQueryDebounced, search)
watch(searchQuery, q => {
  const searching = !isEmpty(q.trim())
  isSearching.value = searching
})

// Add item to model
function addItem(item: Category) {
  if (!model.value.some((m) => m.id === item.id)) {
    model.value.push(item)
  }
  searchQuery.value = ''
  searchResults.value = []
  searchOpen.value = false
}

// Get display label for category item
function getItemLabel(id: number): string {
  // @ts-expect-error enum index
  const item = props.relations[props.type].get(id) as Category | undefined
  if (!item) return id.toString()
  const name = getCategoryName(props.type, item, true)

  const list = [name]
  if ('platform' in item && item.platform) {
    const platformName = props.relations.platforms.get(item.platform)?.name
    if (platformName) list.push("(" + platformName + ")")
  }
  if (props.type === CategoryType.Collection && 'source' in item && item.source) {
    console.log('item source', item.source)
    list.push("(" + item.source + ")")
  }
  return list.join(' ')
}
</script>

<template>
  <div class="flex flex-col gap-1">
    <span class="text-sm ml-2">{{ type }}:</span>
    <Popover v-model:open="searchOpen">
      <ListboxRoot v-model="modelIds" highlight-on-hover multiple>
        <PopoverAnchor class="inline-flex w-full">
          <TagsInput v-slot="{modelValue: tags}" v-model="modelIds" class="w-full gap-1" :displayValue="(id) =>
            getItemLabel(Number(id))">
            <TagsInputItem v-for="id in tags" :key="String(id)" :value="String(id)" class="h-6">
              <TagsInputItemText class="text-xs" />
              <TagsInputItemDelete />
            </TagsInputItem>

            <ListboxFilter v-model="searchQuery" as-child>
              <TagsInputInput :placeholder="`Search ${type}...`" class="min-w-20" @keydown.enter.prevent
                @input="(v: string) => searchOpen = true" />
            </ListboxFilter>

            <PopoverTrigger as-child>
              <Button size="icon-sm" variant="ghost" class="order-last self-start ml-auto">
                <ChevronDownIcon class="size-3.5" />
              </Button>
            </PopoverTrigger>
          </TagsInput>
        </PopoverAnchor>

        <PopoverContent class="p-1" align="start" :style="{ width: 'var(--reka-popover-trigger-width)' }" @open-auto-focus.prevent>
          <div v-if="isSearching" class="flex items-center justify-center py-4">
            <LoaderCircleIcon class="animate-spin" :size="20" />
          </div>
          <div v-else-if="searchQueryDebounced && searchResults.length === 0"
            class="py-4 text-center text-sm text-muted-foreground">
            No results found
          </div>
          <ScrollArea v-else-if="searchResults.length">
            <ListboxContent class="scroll-py-1 max-h-[300px]" tabindex="0">
              <ListboxItem v-for="item in searchResults" :key="item.id" :value="item.id.toString()"
                class="data-highlighted:bg-accent data-highlighted:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none"
                @select="() => {addItem(item); searchQuery = ''}">
                <span class="truncate">{{ getItemLabel(item.id) }}</span>
                <CheckIcon v-if="model.some((m) => m.id === item.id)" class="ml-auto size-4" />
              </ListboxItem>
            </ListboxContent>
          </ScrollArea>
          <div v-else class="py-4 text-center text-sm text-muted-foreground">
            Start typing to search {{ type }}...
          </div>
        </PopoverContent>
      </ListboxRoot>
    </Popover>
  </div>
</template>
