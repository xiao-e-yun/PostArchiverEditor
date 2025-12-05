<script setup lang="ts">
import {isEmpty} from 'lodash-es'
import {Button} from '../ui/button'
import {ButtonGroup} from '../ui/button-group'

const props = defineProps<{
  changes: Record<string, unknown>
  deleteLabel?: string
}>()

const emit = defineEmits<{
  save: []
  discard: []
  delete: []
}>()

const hasChanges = () => !isEmpty(props.changes)
</script>

<template>
  <div class="flex justify-between">
    <Button variant="destructive" @click="emit('delete')">{{ deleteLabel ?? 'Delete' }}</Button>
    <ButtonGroup v-if="hasChanges()" class="flex gap-2 justify-end">
      <Button variant="destructive" @click="emit('discard')">Discard</Button>
      <Button @click="emit('save')">Save</Button>
    </ButtonGroup>
  </div>
</template>
