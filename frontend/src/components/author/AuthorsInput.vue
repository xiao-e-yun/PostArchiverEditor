<script setup lang="ts">
import {computed, ref, toRef} from 'vue'
import {useFetch} from '@vueuse/core'
import type {Totalled, WithRelations} from '@/types'
import type {FileMetaId, FileMeta, Author} from 'post-archiver'
import {parseFileMetaToUrl, useFileMetas} from '@/lib/utils'
import RelationsInput from '../input/RelationsInput.vue'

const props = defineProps<{
  fileMetas: Record<FileMetaId, FileMeta>
}>()

const model = defineModel<Author[]>({required: true})

const search = ref('')
const url = computed(() => `/api/authors` + (search.value ? `?search=${encodeURIComponent(search.value)}` : ''))

const {data} = useFetch(url, {
  refetch: true,
}).json<WithRelations<Totalled<Author>>>()

const options = computed(() => data.value?.items || [])
const parentFileMetas = toRef(props, 'fileMetas')
const fileMetas = useFileMetas(data, parentFileMetas)
</script>

<template>
  <RelationsInput :options="options" v-model="model" v-model:search="search" placeholder="Authors" v-slot="{item}">
    <img v-if="item.thumb && fileMetas[item.thumb]" :src="parseFileMetaToUrl(fileMetas[item.thumb]!) + '?w=40&h=40'"
      class="h-full max-h-6 aspect-square rounded-full object-cover mr-1">
    @{{ item.name }}
  </RelationsInput>
</template>
