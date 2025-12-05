<script setup lang="ts">
import {Input} from '../ui/input';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Platform} from 'post-archiver';
import {CategoryType} from '@/category';
import {injectData, useCategoryActions} from './utils';

const proxyed = injectData<Platform>();

const {update, remove} = useCategoryActions({
  type: CategoryType.Platform,
  proxyed,
});
</script>

<template>
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <ActionButtons
      v-model="proxyed"
      @save="update"
      @delete="remove"
    />
  </div>
</template>
