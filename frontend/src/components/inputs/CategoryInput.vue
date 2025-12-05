<script setup lang="ts">
import type {Category, ListResponse, WithRelations} from '@/api'
import {getCategoryName, type CategoryType} from '@/category'
import {getUrlWithParams, type Relations} from '@/utils'
import {refDebounced} from '@vueuse/core'
import {CheckIcon, ChevronDownIcon, LoaderCircleIcon, XIcon} from 'lucide-vue-next'
import {ListboxContent, ListboxItem, ListboxRoot} from 'reka-ui'
import {computed, ref, watch} from 'vue'
import {Button} from '../ui/button'
import {Popover, PopoverContent, PopoverTrigger} from '../ui/popover'
import {ScrollArea} from '../ui/scroll-area'
import {Input} from '../ui/input'

const props = defineProps<{
  type: CategoryType
  relations: Relations
}>()

const model = defineModel<number | null>({required: true})

// Search state
const searchOpen = ref(false)
const searchQuery = ref('')
const searchQueryDebounced = refDebounced(searchQuery, 250)
const searchResults = ref<Category[]>([])
const isSearching = ref(false)

// Search API call
async function search() {
  try {
    const params: Record<string, string> = {}
    if (searchQueryDebounced.value.trim()) {
      params.search = searchQueryDebounced.value
    }
    const response = await fetch(
      getUrlWithParams(`/api/${props.type}`, params),
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
watch(searchQuery, () => isSearching.value = true)

// Trigger initial search when popover opens
watch(searchOpen, (open) => {
  if (open && searchResults.value.length === 0) {
    search()
  }
})

// Select item
function selectItem(item: Category) {
  model.value = item.id
  searchQuery.value = ''
  searchResults.value = []
  searchOpen.value = false
}

// Clear selection
function clearSelection() {
  model.value = null
}

// Get display label for category item
function getItemLabel(id: number): string {
  // @ts-expect-error enum index
  const item = props.relations[props.type]?.get(id) as Category | undefined
  if (!item) return id.toString()
  const name = getCategoryName(props.type, item, true)
  if ('platform' in item && item.platform) {
    const platformName = props.relations.platforms.get(item.platform)?.name
    return platformName ? `${name} (${platformName})` : name
  }
  return name
}

// Computed display value
const displayValue = computed(() => {
  if (model.value === null) return ''
  return getItemLabel(model.value)
})
</script>

<template>
  <div class="flex flex-col gap-1">
    <span class="text-sm ml-2 capitalize">{{ type.slice(0, type.length - 1) }}:</span>
    <Popover v-model:open="searchOpen">
      <ListboxRoot highlight-on-hover>
        <PopoverTrigger as-child>
          <div
            class="flex h-9 w-full items-center justify-between rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm cursor-pointer hover:bg-accent hover:text-accent-foreground"
          >
            <span v-if="model !== null" class="truncate">{{ displayValue }}</span>
            <span v-else class="text-muted-foreground">Select {{ type }}...</span>
            <div class="flex items-center gap-1">
              <Button
                v-if="model !== null"
                size="icon-sm"
                variant="outline"
                class="size-5"
                @click.stop="clearSelection"
              >
                <XIcon class="size-3" />
              </Button>
              <ChevronDownIcon class="size-4 opacity-50" />
            </div>
          </div>
        </PopoverTrigger>

        <PopoverContent
          class="p-1"
          align="start"
          :style="{width: 'var(--reka-popover-trigger-width)'}"
          @open-auto-focus.prevent
        >
          <Input
            v-model="searchQuery"
            :placeholder="`Search ${type}...`"
            class="mb-1"
            @keydown.enter.prevent
          />

          <div v-if="isSearching" class="flex items-center justify-center py-4">
            <LoaderCircleIcon class="animate-spin" :size="20" />
          </div>
          <div
            v-else-if="searchQueryDebounced && searchResults.length === 0"
            class="py-4 text-center text-sm text-muted-foreground"
          >
            No results found
          </div>
          <ScrollArea v-else-if="searchResults.length">
            <ListboxContent class="scroll-py-1 max-h-[300px]" tabindex="0">
              <ListboxItem
                v-for="item in searchResults"
                :key="item.id"
                :value="item.id.toString()"
                class="hover:bg-accent hover:text-accent-foreground data-highlighted:bg-accent data-highlighted:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none"
                @select="() => selectItem(item)"
              >
                <span class="truncate">{{ getItemLabel(item.id) }}</span>
                <CheckIcon v-if="model === item.id" class="ml-auto size-4" />
              </ListboxItem>
            </ListboxContent>
          </ScrollArea>
          <div v-else class="py-4 text-center text-sm text-muted-foreground">
            No items available
          </div>
        </PopoverContent>
      </ListboxRoot>
    </Popover>
  </div>
</template>
