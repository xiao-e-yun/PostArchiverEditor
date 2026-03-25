<script setup lang="ts">
import type {ZonedDateTime} from '@internationalized/date'
import {DateFormatter, today} from '@internationalized/date'

import {CalendarIcon} from 'lucide-vue-next'
import {cn} from '@/lib/utils'
import {Button} from '@/components/ui/button'
import {Calendar} from '@/components/ui/calendar'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'

const defaultPlaceholder = today("UTC")

const date = defineModel<ZonedDateTime>()

const df = new DateFormatter('zh-CN', {
  timeZone: "UTC",
})
</script>

<template>
  <Popover v-slot="{close}">
    <PopoverTrigger as-child>
      <Button variant="outline" :class="cn('justify-start text-left font-normal', !date &&
        'text-muted-foreground')" v-bind="$attrs">
        <CalendarIcon />
        {{ date ? df.format(date.toDate()) : "Pick a date" }}
      </Button>
    </PopoverTrigger>
    <PopoverContent class="w-auto p-0 z-200" align="start" @open-auto-focus.prevent>
      <Calendar v-model="date" :default-placeholder="defaultPlaceholder" layout="month-and-year" @update:model-value="close" />
    </PopoverContent>
  </Popover>
</template>
