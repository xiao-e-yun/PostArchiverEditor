<script setup lang="ts">
import {cn} from '@/lib/utils';
import {useVModel} from '@vueuse/core';
import type {HTMLAttributes} from 'vue';

const props = defineProps<{
  defaultValue?: string | number;
  modelValue?: string | number;
  class?: HTMLAttributes['class'];
  lazy?: boolean; // Aggiungi questa prop
  number?: boolean; // Aggiungi questa prop per specificare se l'input è di tipo numero
}>();

const emits = defineEmits<{
  (e: 'update:modelValue', payload: string | number): void;
}>();

const modelValue = useVModel(props, 'modelValue', emits, {
  passive: true,
  defaultValue: props.defaultValue,
});

// Add number support
function getValue(event: Event) {
  const value = (event.target as HTMLInputElement).value;
  return props.number ? Number(value) : value;
}
// Add lazy support
const handleInput = (event: Event) => {
  const value = getValue(event);
  if (!props.lazy) modelValue.value = value;
};

const handleChange = (event: Event) => {
  const value = getValue(event);
  if (props.lazy) modelValue.value = value;
};
</script>

<template>
  <input :value="modelValue" :class="cn(
    'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
    props.class,
  )
    " @input="handleInput" @change="handleChange" />
</template>
