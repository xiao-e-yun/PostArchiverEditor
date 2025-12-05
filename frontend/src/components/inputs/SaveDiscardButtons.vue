<script setup lang="ts">
import {isEmpty} from 'lodash-es'
import {Button} from '../ui/button'
import {ButtonGroup} from '../ui/button-group'

const props = defineProps<{
  changes: Record<string, unknown>
}>()

const emit = defineEmits<{
  save: []
  discard: []
}>()

const hasChanges = () => !isEmpty(props.changes)
</script>

<template>
  <ButtonGroup v-if="hasChanges()" class="flex gap-2 justify-end">
    <Button variant="destructive" @click="emit('discard')">Discard</Button>
    <Button @click="emit('save')">Save</Button>
  </ButtonGroup>
</template>
