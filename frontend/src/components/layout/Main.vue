<script setup lang="ts">
import { useActiveItem } from '@/utils'
import { useFetch } from '@vueuse/core'
import { computed, type Ref } from 'vue'
import { Alert, AlertDescription, AlertTitle } from '../ui/alert'

import MainPost from '../main/Post.vue'
import { ScrollArea } from '../ui/scroll-area'

const activeItem = useActiveItem()

const url = computed(
  () => activeItem.value && `/api/${activeItem.value.type}/${activeItem.value.id}`,
)
const { data, isFetching, error } = useFetch(url as Ref<string>, { refetch: true }).json()

const match = (t: string) => activeItem.value?.type === t
</script>

<template>
  <main class="w-full">
    <h1 class="text-lg m-4" v-if="!activeItem">Select an item from aside.</h1>
    <h1 class="text-lg m-4" v-else-if="isFetching">Loading...</h1>
    <Alert v-else-if="error" variant="destructive" class="my-4! m-auto max-w-lg">
      <AlertTitle>Error</AlertTitle>
      <AlertDescription>
        {{ error }}
      </AlertDescription>
    </Alert>
    <ScrollArea
      v-else-if="data"
      class="h-full *:data-reka-scroll-area-viewport:p-4 *:data-reka-scroll-area-viewport:mx-auto"
    >
      <MainPost v-if="match('posts')" :data="data" />
      <template v-else>{{ data }}</template>
    </ScrollArea>
  </main>
</template>
