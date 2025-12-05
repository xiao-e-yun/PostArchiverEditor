<script setup lang="ts">
import {reactiveChanges, useRelations} from '@/utils';
import type {WithRelations} from '@api/WithRelations';
import {computed, ref, toRef} from 'vue';
import {Input} from '../ui/input';
import {CategoryType} from '@/category';
import CategoryInput from '../inputs/CategoryInput.vue';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Tag} from 'post-archiver';
import {useCategoryActions} from './utils';

const props = defineProps<{
  data: WithRelations<Tag>
}>();

const data = toRef(props, 'data');
const relations = computed(() => useRelations(props.data));
const proxyed = ref(reactiveChanges(data.value));

const {update, remove} = useCategoryActions({
  type: CategoryType.Tag,
  data,
  proxyed,
});
</script>

<template>
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <CategoryInput v-model="proxyed.platform" :type="CategoryType.Platform" :relations="relations" />
    <ActionButtons
      v-model="proxyed.changes"
      @save="update"
      @delete="remove"
    />
  </div>
</template>
