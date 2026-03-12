<script setup lang="ts">
import {Input} from '../ui/input';
import {CategoryType} from '@/category';
import CategoryInput from '../inputs/CategoryInput.vue';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Tag} from 'post-archiver';
import {injectData, useCategoryActions} from './utils';
import MiniPostList from '../MiniPostList.vue';

const proxyed = injectData<Tag>();

const {update, remove} = useCategoryActions({
  type: CategoryType.Tag,
  proxyed,
});
</script>

<template>
  <div class="flex gap-4 max-lg:flex-col relative">
    <div class="flex flex-col gap-4 w-full">
      <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
      <CategoryInput v-model="proxyed.platform" :type="CategoryType.Platform" />
      <ActionButtons v-model="proxyed" @save="update" @delete="remove" />
    </div>
    <div class="flex flex-col gap-4 lg:w-lg">
      <MiniPostList category="tags" :id="proxyed._raw.id" />
    </div>
  </div>
</template>
