<script setup lang="ts">
import {Input} from '../ui/input';
import type {FileMeta} from 'post-archiver';
import CategoryInput from '../inputs/CategoryInput.vue';
import {CategoryType} from '@/category';
import {Textarea} from '../ui/textarea';
import ActionButtons from '../inputs/ActionButtons.vue';
import {injectData, useCategoryActions} from './utils';
import {computed, ref} from 'vue';

const proxyed = injectData<FileMeta>();

const mime = computed({
  get: () => (proxyed.value.mime.split('/') || ['', '']) as [string, string],
  set: (val: [string, string]) => proxyed.value.mime = val.join('/')
})

const extra = computed({
  get: () => JSON.stringify(proxyed.value.extra, null, 2),
  set: (val: string) => {
    extraIsValid.value = true
    try {
      proxyed.value.extra = JSON.parse(val);
    } catch {
      console.warn('Invalid JSON for extra data');
      extraIsValid.value = false
    }
  }
})
const extraIsValid = ref(true);

const {update, remove} = useCategoryActions({
  type: CategoryType.FileMeta,
  proxyed,
});
</script>

<template>
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Input v-model="proxyed.filename" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <div>
      <span class="ml-2">Mime:</span>
      <div class="rounded-md bg-input px-3 py-1 text-sm shadow-sm cursor-pointer mt-1 flex
        items-center gap-3 border border-transparent">
        <Input v-model="mime[0]" class="w-1/2 bg-transparent! p-0 m-0 border-0 focus:ring-0 text-center
           focus:outline-0" placeholder="type" />
        /
        <Input v-model="mime[1]" class="w-1/2 bg-transparent! p-0 m-0 border-0 focus:ring-0 text-center
           focus:outline-0" placeholder="subtype" />
      </div>
    </div>

    <CategoryInput v-model="proxyed.post" :type="CategoryType.Post" />

    <div class="relative w-full">
      <span class="ml-2">Extra Data:</span>
      <Textarea v-model.lazy="extra"
        class="w-full h-32 p-2 border rounded-md bg-input text-sm shadow-sm mt-1 resize-none focus:ring-0 focus:outline-0"
        placeholder="Extra metadata in JSON format" />
      <span v-if="!extraIsValid" class="absolute top-2 right-2 text-sm text-red-500">
        Invalid JSON
      </span>
    </div>

    <ActionButtons
      v-model="proxyed"
      @save="update"
      @delete="remove"
    />
  </div>
</template>
