<script setup lang="ts">
import { parseDateTime, parseZonedDateTime, type DateValue } from '@internationalized/date'
import { CalendarIcon } from 'lucide-vue-next'
import { computed } from 'vue'
import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { cn } from '@/lib/utils'

const model = defineModel<string>()
const date = computed<DateValue>({
  get: () => parseDateTime(model.value!.slice(0, 10)),
  set: (val: DateValue) => model.value = val.toString().slice(0, 10) + model.value!.slice(10),
})
</script>

<template>
  <Popover>
    <PopoverTrigger as-child>
      <Button
        variant="outline"
        :class="cn(
          'w-[160px] justify-start text-left font-normal',
          !date && 'text-muted-foreground',
        )"
        v-bind="$attrs"
      >
        <CalendarIcon class="mr-2 h-4 w-4" />
        {{ date.year }}/{{ date.month }}/{{ date.day }}
      </Button>
    </PopoverTrigger>
    <PopoverContent class="w-auto p-0">
      <Calendar
        v-model="date"
        :initial-focus="true"
        layout="month-and-year"
      />
    </PopoverContent>
  </Popover>
</template>

