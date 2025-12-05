<script setup lang="ts">
import {computed} from 'vue'
import {Input} from '../ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../ui/select'

const model = defineModel<string>({required: true})

const MIME_TYPES = [
  'application',
  'audio',
  'example',
  'font',
  'haptics',
  'image',
  'message',
  'model',
  'multipart',
  'text',
  'video',
] as const

const mimeType = computed({
  get: () => {
    const [type] = model.value.split('/')
    return type || ''
  },
  set: (val: string) => {
    const [, subtype] = model.value.split('/')
    model.value = `${val}/${subtype || ''}`
  },
})

const mimeSubtype = computed({
  get: () => {
    const parts = model.value.split('/')
    return parts[1] || ''
  },
  set: (val: string) => {
    const [type] = model.value.split('/')
    model.value = `${type || ''}/${val}`
  },
})
</script>

<template>
  <div class="flex flex-col gap-1">
    <span class="text-sm ml-2">Mime:</span>
    <div
      class="rounded-md bg-input px-3 py-1 text-sm shadow-sm mt-1 flex items-center gap-2 border border-transparent h-9"
    >
      <Select v-model="mimeType">
        <SelectTrigger class="w-1/2 bg-transparent! border-0 shadow-none h-7 px-2">
          <span />
          <SelectValue placeholder="type" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem v-for="type in MIME_TYPES" :key="type" :value="type">
            {{ type }}
          </SelectItem>
        </SelectContent>
      </Select>
      <span class="text-muted-foreground">/</span>
      <Input
        v-model="mimeSubtype"
        class="w-1/2 bg-transparent! p-0 m-0 border-0 focus:ring-0 text-center focus:outline-0 h-7"
        placeholder="subtype"
      />
    </div>
  </div>
</template>
