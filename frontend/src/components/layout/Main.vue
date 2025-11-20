<script setup lang="ts">
import {useActiveItem} from '@/utils';
import {useFetch} from '@vueuse/core';
import {computed, ref, type Ref} from 'vue';
import {Alert, AlertDescription, AlertTitle} from '../ui/alert';

const activeItem = useActiveItem();

const url = computed(() => activeItem.value && `/api/${activeItem.value.type}/${activeItem.value.id}`);
const error = ref<string | null>(null);
const { data, isFetching } = useFetch(url as Ref<string>,  {refetch: true, onFetchError: (err) => error.value = err.error}).json<any>();
</script>

<template>
  <main class="p-4 w-full">
    <h1 class="text-lg" v-if="!activeItem"> Select an item from aside. </h1>
    <h1 class="text-lg" v-else-if="isFetching"> Loading... </h1>
    <Alert v-else-if="error" variant="destructive" class="m-auto max-w-lg">
      <AlertTitle>Error</AlertTitle>
      <AlertDescription>
        {{ error }}
      </AlertDescription>
    </Alert>
    <template v-else>
      {{ data }}
    </template>
  </main>
</template>
