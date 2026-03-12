<script setup lang="ts">
import {useRelations} from '@/utils';
import {useFetch} from '@vueuse/core';
import {computed, ref} from 'vue';
import {Badge} from './ui/badge';
import {Separator} from './ui/separator';
import {ChevronLeft, ChevronRight, Image} from 'lucide-vue-next';
import {ButtonGroup} from './ui/button-group';
import {Button} from './ui/button';

const props = defineProps<{
  category: string,
  id: number
}>()

const page = ref(0)
const url = computed(() => `/api/${props.category}/${props.id}/posts?page=${page.value}`)
const {data: posts, isFetching} = useFetch(url, {refetch: true}).json()

const relations = useRelations(posts)
</script>

<template>
  <div class="flex flex-col gap-2 md:w-sm">
    <div class="flex items-center gap-2 justify-between w-full">
      <p>
        Posts
        <span v-if="posts" class="text-sm text-muted-foreground">({{ posts.total }})</span>
      </p>
      <Separator class="flex-1" />
      <ButtonGroup v-if="posts" size="sm">
        <Button variant="outline" :disabled="page === 0" @click="page--" class="w-8 h-8">
          <ChevronLeft />
        </Button>
        <Button variant="outline" class="w-12 h-8 px-2 border-x select-none"select-none @click="page = 0">
          {{ page + 1 }} / {{ Math.ceil(posts.total / 20) || 1 }}
        </Button>
        <Button variant="outline" :disabled="(page + 1) * 20 >= posts.total" @click="page++" class="w-8 h-8">
          <ChevronRight />
        </Button>
      </ButtonGroup>
    </div>
    <ul v-if="posts" class="flex flex-col gap-2">
      <li v-for="post in posts.items" :key="post.id" class="flex items-center gap-2 h-16">
        <a :href="'/?item=p-' + post.id" target="_blank" class="flex items-center gap-2 rounded-md
          w-full pr-2 rounded-md border hover:bg-secondary">
          <div class="h-full aspect-square rounded-md border-r overflow-hidden w-16 flex-shrink-0">
            <img v-if="post.thumb" :src="relations.fileMetaPath(post.thumb) + '&w=62&h=62'" alt="Post Thumb"
              class="h-full aspect-square object-cover" />
            <Image v-else class="h-full w-full p-1" />
          </div>
          <div class="flex flex-col min-w-0">
            <span class="flex-1 truncate">{{ post.title }}</span>
            <Badge variant="outline" class="shrink-0">{{ relations.platforms.get(post.platform)?.name || post.platform
              }}</Badge>
          </div>
        </a>
      </li>
      <li v-if="isFetching" class="text-sm text-muted-foreground text-center py-4">
        Loading...
      </li>
      <li v-else-if="posts.items.length === 0" class="text-sm text-muted-foreground text-center py-4">
        No posts found.
      </li>
    </ul>
  </div>
</template>
