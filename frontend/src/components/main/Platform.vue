<script setup lang="ts">
import {reactiveChanges} from '@/utils';
import type {WithRelations} from '@api/WithRelations';
import {ref, toRef} from 'vue';
import {Input} from '../ui/input';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Platform} from 'post-archiver';
import {CategoryType} from '@/category';
import {useCategoryActions} from './utils';

const props = defineProps<{
  data: WithRelations<Platform>
}>();

const data = toRef(props, 'data');
const proxyed = ref(reactiveChanges(data.value));

const {update, remove, discard} = useCategoryActions({
  type: CategoryType.Platform,
  data,
  proxyed,
});
</script>

<template>
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <ActionButtons
      :changes="proxyed.changes"
      @save="update"
      @discard="discard"
      @delete="remove"
    />
  </div>
</template>
