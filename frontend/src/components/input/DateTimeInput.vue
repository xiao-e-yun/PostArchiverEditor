<script setup lang="ts">
import {computed, ref, watch} from 'vue';
import {now, parseAbsolute, ZonedDateTime, type DateValue} from '@internationalized/date'
import {Input} from '../ui/input';
import DatePicker from './DatePicker.vue';

const raw = defineModel<string | null>();

const date = ref<ZonedDateTime>(now("UTC"));
watch(raw, value => {
  if (!value) return date.value = now("UTC");
  date.value = parseAbsolute(value, "UTC");
}, {immediate: true})

watch(date, value => {
  const isoString = value.toDate().toISOString();
  raw.value = isoString.substring(0, isoString.length - 5) + "Z";
}, {deep: true})

const dateWithoutTime = computed({
  get() {
    return date.value
  },
  set(value: DateValue) {
    date.value = date.value.set({
      year: value.year,
      month: value.month,
      day: value.day,
    })
  }
})
</script>

<template>
  <div
    class="flex border rounded-md items-center focus-within:border-ring focus-within:ring-ring/50 focus-within:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive">
    <!-- @vue-expect-error - DatePicker is a custom component that does not have a type definition -->
    <DatePicker v-model="dateWithoutTime" class="h-full flex-1 border-none bg-transparent!" />
    <Input v-model="date.hour" type="number" min="0" max="23"
      class="max-w-16 border-none text-center ring-0! outline-none! shadow-none! ring-offset-0! " />
    :
    <Input v-model="date.minute" type="number" min="0" max="59"
      class="max-w-16 border-none  text-center ring-0! outline-none! shadow-none! ring-offset-0! " />
  </div>
</template>
