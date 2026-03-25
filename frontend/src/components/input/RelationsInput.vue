<script setup lang="ts" generic="T extends {'id': number}">
import {CheckIcon, ChevronDown} from 'lucide-vue-next'
import {ListboxContent, ListboxItem, ListboxItemIndicator, ListboxRoot} from 'reka-ui'
import {ref, watch} from 'vue'
import {Button} from '@/components/ui/button'
import {Popover, PopoverAnchor, PopoverContent, PopoverTrigger} from '@/components/ui/popover'
import {TagsInput, TagsInputItem, TagsInputItemDelete} from '@/components/ui/tags-input'

defineProps<{
  options: T[]
  placeholder: string
}>()

const items = defineModel<T[]>({required: true})
const search = defineModel<string>('search', {required: true})

const open = ref(false)
watch(search, (f) => open.value = !!f)
</script>

<template>
  <Popover v-model:open="open">
    <ListboxRoot v-model="items" highlight-on-hover multiple>
      <PopoverAnchor class="inline-flex w-full">
        <TagsInput v-model="items" class="w-full">
          <TagsInputItem v-for="item in items" :key="item.id" :value="item">
            <slot :item="item" />
            <TagsInputItemDelete />
          </TagsInputItem>

          <div class="w-full flex">
            <input v-model="search" :placeholder="placeholder+'...'" @keydown.down="open = true"
              class="border-none text-sm min-h-5 focus:outline-none flex-1 bg-transparent px-1" />

            <PopoverTrigger as-child>
              <Button size="icon-sm" variant="ghost" class="order-last self-start ml-auto">
                <ChevronDown class="size-3.5" />
              </Button>
            </PopoverTrigger>
          </div>
        </TagsInput>
      </PopoverAnchor>

      <PopoverContent class="p-1 z-200 w-[var(--reka-popper-anchor-width)]" @open-auto-focus.prevent>
        <ListboxContent
          class=" scroll-py-1 overflow-x-hidden overflow-y-auto empty:after:content-['No_options'] empty:p-1 empty:after:block"
          tabindex="0">
          <ListboxItem v-for="item in options" :key="item.id" class="data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground
            [&_svg:not([class*=\'text-\'])]:text-muted-foreground relative flex cursor-default
            items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none
            data-[disabled]:pointer-events-none data-[disabled]:opacity-50
            [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*=\'size-\'])]:size-4
            hover:bg-accent/50" :value="item" @select="search = ''">
            <slot :item="item" />
            <ListboxItemIndicator class="ml-auto inline-flex items-center justify-center">
              <CheckIcon />
            </ListboxItemIndicator>
          </ListboxItem>
        </ListboxContent>
      </PopoverContent>
    </ListboxRoot>
  </Popover>
</template>
