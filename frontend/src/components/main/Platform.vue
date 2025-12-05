<script setup lang="ts">
import {reactiveChanges, useActiveItem} from '@/utils';
import type {WithRelations} from '@api/WithRelations';
import {ref, toRef} from 'vue';
import {Input} from '../ui/input';
import {isEmpty} from 'lodash-es';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Platform} from 'post-archiver';

const props = defineProps<{
  data: WithRelations<Platform>
}>();

const data = toRef(props, 'data');
const proxyed = ref(reactiveChanges(data.value));

const updatePlatform = () => {
  if (isEmpty(proxyed.value.changes)) return;
  const id = data.value.id;
  fetch(`/api/platforms/${id}`, {
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
      alert('Platform updated successfully');
    } else {
      const error = await res.text();
      alert(`Error updating platform: ${error} ${res.statusText}`);
    }
  }).catch((err) => {
    alert(`Error updating platform: ${err.message}`);
  });
}

const deletePlatform = () => {
  if (!confirm('Are you sure you want to delete this platform?')) return
  const id = data.value.id;
  fetch(`/api/platforms/${id}`, {
    method: 'DELETE',
  }).then(async (res) => {
    if (res.ok) {
      alert('Platform deleted successfully');
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
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <ActionButtons
      :changes="proxyed.changes"
      @save="updatePlatform"
      @discard="proxyed.changes = {}"
      @delete="deletePlatform"
    />
  </div>
</template>
