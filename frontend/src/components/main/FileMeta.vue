<script setup lang="ts">
import type { FileMeta } from 'post-archiver'
import CategoryInput from '../inputs/CategoryInput.vue'
import { CategoryType } from '@/category'
import ActionButtons from '../inputs/ActionButtons.vue'
import { injectData, useCategoryActions } from './utils'
import { computed, ref } from 'vue'
import JsonEditor from '../inputs/JsonEditor.vue'
import { isPlainObject } from 'lodash-es'
import { FileIcon } from 'lucide-vue-next'
import { Button } from '../ui/button'
import { getFileMetaPath } from '@/utils'
import { Card } from '../ui/card'
import MimeInput from '../inputs/MimeInput.vue'

const proxyed = injectData<FileMeta>()

const link = computed(() => getFileMetaPath(proxyed.value))

const resetKey = ref(0)
const extra = computed({
  get: () => proxyed.value.extra,
  set: (val) => {
    if (!isPlainObject(val)) {
      console.warn('Extra data must be a plain object')
      resetKey.value++
      return
    }
    proxyed.value.extra = val
  },
})
const extraIsValid = ref(true)

const { update, remove } = useCategoryActions({
  type: CategoryType.FileMeta,
  proxyed,
})
</script>

<template>
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Card v-if="proxyed.mime.startsWith('image/')" class="p-0 gap-1 pb-1">
      <img
        :src="link"
        alt="File Meta Preview"
        class="w-full object-cover rounded-md"
      />
      <a class="mx-auto text-sm text-muted-foreground" :href="link">{{ proxyed.filename }}</a>
    </Card>
    <Card v-else>
      <FileIcon class="mx-auto w-1/3 h-auto aspect-square" />
      <a class="mx-auto text-sm text-muted-foreground">{{ proxyed.filename }}</a>
      <Button size="sm" class="mx-auto w-32" as="a" :href="link" download>
        Download
      </Button>
    </Card>

    <MimeInput v-model="proxyed.mime" />

    <CategoryInput v-model="proxyed.post" :type="CategoryType.Post" />

    <div class="relative w-full">
      <span class="ml-2">Extra Data:</span>
      <JsonEditor v-model="extra" :key="resetKey" />
      <span v-if="!extraIsValid" class="absolute top-2 right-2 text-sm text-red-500">
        Invalid JSON
      </span>
    </div>

    <ActionButtons @save="update" @delete="remove" />
  </div>
</template>
