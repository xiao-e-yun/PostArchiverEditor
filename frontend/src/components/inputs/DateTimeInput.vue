<script setup lang="ts">
import {DatePicker} from '../ui/date-picker';
import {TimePicker} from '../ui/time-picker';

const date = defineModel<string>();
</script>

<template>
  <div class="flex items-center gap-2 max-sm:flex-col max-sm:items-stretch">
    <DatePicker v-model="date" class="flex-1 max-sm:w-full" />
    <TimePicker v-model="date" withSeconds />
  </div>
</template>
