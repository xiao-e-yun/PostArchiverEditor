<script setup lang="ts">
import {Input} from '../ui/input';
import {CategoryType} from '@/category';
import CategoryInput from '../inputs/CategoryInput.vue';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Author} from 'post-archiver';
import {injectData, injectRelations, useCategoryActions} from './utils';
import MiniPostList from '../MiniPostList.vue';

const proxyed = injectData<Author>();
const relations = injectRelations();

const {update, remove} = useCategoryActions({
  type: CategoryType.Author,
  proxyed,
});
</script>

<template>
  <div class="flex gap-4 max-lg:flex-col relative">
    <div class="flex flex-col gap-4 w-full">
      <img v-if="proxyed.thumb" alt="Author Thumb" :src="relations.fileMetaPath(proxyed.thumb)"
        class="w-full h-48 object-cover rounded-md" />
      <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
      <CategoryInput v-model="proxyed.thumb" label="Thumb" :type="CategoryType.FileMeta" />
      <ActionButtons v-model="proxyed" @save="update" @delete="remove" />
    </div>
    <div class="flex flex-col gap-4 lg:w-lg">
      <MiniPostList category="authors" :id="proxyed._raw.id" />
    </div>
  </div>
</template>
