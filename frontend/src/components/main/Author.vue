
<script setup lang="ts">
import {reactiveChanges, useActiveItem, useRelations} from '@/utils';
import type {WithRelations} from '@api/WithRelations';
import {computed, ref, toRef} from 'vue';
import {Input} from '../ui/input';
import {isEmpty} from 'lodash-es';
import {CategoryType} from '@/category';
import CategoryInput from '../inputs/CategoryInput.vue';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Tag} from 'post-archiver';

const props = defineProps<{
  data: WithRelations<Tag>
}>();

const data = toRef(props, 'data');
const relations = computed(() => useRelations(props.data));
const proxyed = ref(reactiveChanges(data.value));

const updateTag = () => {
  if (isEmpty(proxyed.value.changes)) return;
  const id = data.value.id;
  fetch(`/api/tags/${id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(proxyed.value.changes)
  }).then(async (res) => {
    if (res.ok) {
      const updatedPost = await res.json();
      Object.assign(data.value, updatedPost);
      proxyed.value = reactiveChanges(data.value);
      alert('Tag updated successfully');
    } else {
      const error = await res.text();
      alert(`Error updating tag: ${error} ${res.statusText}`);
    }
  }).catch((err) => {
    alert(`Error updating tag: ${err.message}`);
  });
}

const deletePost = () => {
  if (!confirm('Are you sure you want to delete this tag?')) return
  const id = data.value.id;
  fetch(`/api/tags/${id}`, {
    method: 'DELETE',
  }).then(async (res) => {
    if (res.ok) {
      alert('Tag deleted successfully');
      useActiveItem().value = null;
    } else {
      const error = await res.text();
      alert(`Error deleting post: ${error}`);
    }
  }).catch((err) => {
    alert(`Error deleting post: ${err.message}`);
  });
}
</script>

<template>
  <div class="flex flex-col gap-4 w-full lg:flex-row lg:items-start lg:gap-8">
    <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <CategoryInput v-model="proxyed.platform" :type="CategoryType.Platform" :relations="relations" />
    <ActionButtons
      :changes="proxyed.changes"
      @save="updateTag"
      @discard="proxyed.changes = {}"
      @delete="deletePost"
    />
  </div>
</template>
