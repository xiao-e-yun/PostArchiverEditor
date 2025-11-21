<template>
  <Input
    :name="picker"
    :class="inputClasses"
    :value="calculatedValue"
    :defaultValue="calculatedValue"
    type="tel"
    inputmode="decimal"
    @keydown="handleKeyDown"
    @wheel.prevent="handleWheel"
  />
</template>

<script setup lang="ts">
import {
  getArrowByType,
  getDateByType,
  setDateByType,
  type Period,
  type TimePickerType
} from './time-picker-utils';
import { cn } from '@/lib/utils';
import {computed, ref, watch} from 'vue';
import {Input} from '../input';

const props = defineProps<{
  picker: TimePickerType;
  period?: Period;
  class?: string;
  name?: string;
}>()

const model = defineModel<string>({ default: new Date(new Date().setHours(0, 0, 0, 0)) });
const date = computed({
  get: () => new Date(model.value),
  set: (val: Date) => model.value = val.toISOString()
});
const emit = defineEmits(['rightFocus', 'leftFocus']);

const flag = ref(false);
const prevIntKey = ref('');

const inputClasses = computed(() =>
  cn('w-[48px] text-center font-mono text-base tabular-nums caret-transparent focus:bg-accent focus:text-accent-foreground [&::-webkit-inner-spin-button]:appearance-none', props.class)
);

const calculatedValue = computed(() => getDateByType(date.value, props.picker));
watch(flag, (newFlag) => {
  if (newFlag) {
    const timer = setTimeout(() => {
      flag.value = false;
    }, 2000);
    return () => clearTimeout(timer);
  }
});

watch(() => props.period, (newPeriod) => {
  if (newPeriod) {
    date.value = setDateByType(date.value, (date.value.getHours() % 12).toString(), props.picker, newPeriod);
  }
});

const calculateNewValue = (key: string) => {
  if (props.picker === '12hours') {
    if (flag.value && prevIntKey.value === '1' && ['0', '1', '2'].includes(key)) {
      const newValue = '1' + key;
      prevIntKey.value = '';
      return newValue;
    }
    if (flag.value) {
      prevIntKey.value = '';
      return prevIntKey.value + key;
    }
    prevIntKey.value = key;
    return '0' + key;
  }
  return !flag.value ? '0' + key : calculatedValue.value.slice(1, 2) + key;
};

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Tab') return;

  e.preventDefault();

  if (['ArrowUp', 'ArrowDown'].includes(e.key)) {
    const step = e.key === 'ArrowUp' ? 1 : -1;
    const newValue = getArrowByType(calculatedValue.value, step, props.picker);
    if (flag.value) flag.value = false;
    date.value = setDateByType(date.value, newValue, props.picker, props.period);
  }
  if (e.key >= '0' && e.key <= '9') {
    const newValue = calculateNewValue(e.key);
    if (flag.value && (newValue === '10' || newValue === '11')) {
      emit('rightFocus');
    }
    flag.value = !flag.value;
    date.value = setDateByType(date.value, newValue, props.picker, props.period);
  }
};

const handleWheel = (e: WheelEvent) => {
  const step = e.deltaY < 0 ? 1 : -1;
  const newValue = getArrowByType(calculatedValue.value, step, props.picker);
  date.value = setDateByType(date.value, newValue, props.picker, props.period);
};
</script>
