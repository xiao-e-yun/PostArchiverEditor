<script setup lang="ts">
import {reactiveChanges} from '@/utils';
import type {WithRelations} from '@api/WithRelations';
import {computed, ref, toRef} from 'vue';
import {Input} from '../ui/input';
import {isEmpty} from 'lodash-es';
import {CategoryType} from '@/category';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Collection} from 'post-archiver';
import {useCategoryActions} from './utils';

const props = defineProps<{
  data: WithRelations<Collection>
}>();

const data = toRef(props, 'data');
const proxyed = ref(reactiveChanges(data.value));

const proxyedSource = computed({
  get: () => proxyed.value.source || '',
  set: (val: string) => proxyed.value.source = isEmpty(val) ? null : val
})

const {update, remove} = useCategoryActions({
  type: CategoryType.Collection,
  data,
  proxyed,
});
</script>

<template>
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <Input v-model="proxyedSource" class="w-full" placeholder="Source URL" />
    <ActionButtons
      v-model="proxyed.changes"
      @save="update"
      @delete="remove"
    />
  </div>
</template>
