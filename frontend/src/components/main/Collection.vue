<script setup lang="ts">
import { computed } from 'vue'
import { Input } from '../ui/input'
import { isEmpty } from 'lodash-es'
import { CategoryType } from '@/category'
import ActionButtons from '../inputs/ActionButtons.vue'
import type { Collection } from 'post-archiver'
import { injectData, injectRelations, useCategoryActions } from './utils'
import CategoryInput from '../inputs/CategoryInput.vue'

const proxyed = injectData<Collection>()
const relations = injectRelations();

const proxyedSource = computed({
  get: () => proxyed.value.source || '',
  set: (val: string) => (proxyed.value.source = isEmpty(val) ? null : val),
})
const { update, remove } = useCategoryActions({
  type: CategoryType.Collection,
  proxyed,
})
</script>

<template>
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <img  v-if="proxyed.thumb" alt="Author Thumb" :src="relations.fileMetaPath(proxyed.thumb)" class="w-full h-48 object-cover rounded-md" />
    <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <Input v-model="proxyedSource" class="w-full" placeholder="Source URL" />
    <CategoryInput v-model="proxyed.thumb" label="Thumb" :type="CategoryType.FileMeta" />
    <ActionButtons :value="proxyed" @save="update" @delete="remove" />
  </div>
</template>
