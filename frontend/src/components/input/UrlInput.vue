<script setup lang="ts">
import {Input} from '../ui/input';
const value = defineModel<string | null>();
</script>

<template>
  <Input type="url" :model-value="value!" @update:model-value="payload => value = (payload === '' ? null : payload) as string" class="text-sm rounded h-8" />
</template>
