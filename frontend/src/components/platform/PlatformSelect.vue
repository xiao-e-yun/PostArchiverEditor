<script setup lang="ts">
import {CheckIcon, ChevronsUpDownIcon} from 'lucide-vue-next'
import {computed, ref} from 'vue'
import {Button} from '@/components/ui/button'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import {useFetch} from '@vueuse/core'
import type {Totalled} from '@/types'
import type  {PlatformId, Platform} from 'post-archiver'
import {watch} from 'vue'
import {capitalize} from 'lodash-es'

const props = defineProps<{
  platforms: Record<PlatformId, Platform>
  nullable?: boolean
}>()

const model = defineModel<PlatformId | null>()

const search = ref('')
const url = computed(() => `/api/platforms` + (search.value ? `?search=${search.value}` : ''))

const {data} = useFetch(url, {
  refetch: true,
}).json<Totalled<Platform>>()

const items = computed(() => data.value?.items || [])

const open = ref(false)

const selected = ref<Platform | null>(null)

watch(model, id => {
  if (id === selected.value?.id) return
  if (id === null) selected.value = null
  selected.value = props.platforms[id!] ?? null
}, {immediate: true})

watch(selected, platform => {
  if (platform?.id === model.value) return
  model.value = platform?.id ?? null
})

function select(selectedValue: Platform) {
  const isSame = selectedValue.id === selected.value?.id
  selected.value = isSame && props.nullable ? null : selectedValue
  open.value = false
}
</script>

<template>
  <Popover v-model:open="open">
    <PopoverTrigger as-child>
      <Button variant="outline" role="combobox" :aria-expanded="open" class="w-full justify-between
        dark:bg-transparent" v-bind="$attrs">
        <span v-if="selected">{{ capitalize(selected.name) }}</span>
        <span v-else class="text-muted-foreground font-normal">{{ nullable ? 'Empty' : 'Platform' }}</span>
        <ChevronsUpDownIcon class="opacity-50" />
      </Button>
    </PopoverTrigger>
    <PopoverContent class="p-0 z-200 w-[var(--reka-popper-anchor-width)]">
      <Command>
        <CommandInput class="h-9" placeholder="Search platforms..." v-model="search" />
        <CommandList>
          <CommandEmpty>No Platform
            found.</CommandEmpty>
          <CommandGroup>
            <CommandItem v-for="item in items" :key="item.id" :value="item" @select="() =>
            select(item)" class="hover:bg-accent/50">
              {{ capitalize(item.name) }}
              <CheckIcon v-show="selected?.id === item.id" class="ml-auto" />
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>
    </PopoverContent>
  </Popover>
</template>
