<script setup lang="ts">
import {computed, ref, toRef} from 'vue'
import {useFetch} from '@vueuse/core'
import type {Totalled, WithRelations} from '@/types'
import type {Platform, PlatformId, Tag} from 'post-archiver'
import {parseFileMetaToUrl, usePlatforms} from '@/lib/utils'
import RelationsInput from '../input/RelationsInput.vue'

const props = defineProps<{
  platforms: Record<PlatformId, Platform>
}>()

const model = defineModel<Tag[]>({required: true})

const search = ref('')
const url = computed(() => `/api/tags` + (search.value ? `?search=${encodeURIComponent(search.value)}` : ''))

const {data} = useFetch(url, {
  refetch: true,
}).json<WithRelations<Totalled<Tag>>>()

const options = computed(() => data.value?.items || [])
const parentPlatforms = toRef(props, 'platforms')
const platforms = usePlatforms(data, parentPlatforms)

const showPlatform = (item: Tag) => {
  const platform = platforms.value[item.platform!];
  return platform ? ` - (${platform.name})` : '';
}
</script>

<template>
  <RelationsInput :options="options" v-model="model" v-model:search="search" placeholder="Tags"
  v-slot="{item}">
  <span class="pl-2">#{{ item.name + showPlatform(item) }}</span>
  </RelationsInput>
</template>
