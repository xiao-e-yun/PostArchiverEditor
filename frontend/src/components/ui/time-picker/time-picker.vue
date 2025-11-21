<template>
  <div class="flex items-center gap-1 bg-input/30 rounded-md border">
    <TimePickerInput picker="hours" v-model="date" class="border-none bg-transparent!
    hover:bg-secondary!" />
    <div>:</div>
    <TimePickerInput picker="minutes" v-model="date" class="border-none bg-transparent! hover:bg-secondary!" />
    <template v-if="withSeconds">
      <div>:</div>
      <TimePickerInput picker="seconds" v-model="date"  class="border-none bg-transparent! hover:bg-secondary!" />
    </template>
  </div>
</template>

<script setup lang="ts">
import {TimePickerInput} from '.';

defineProps<{
  withSeconds?: boolean;
  withLabels?: boolean;
}>();

const date = defineModel<string>();
</script>
