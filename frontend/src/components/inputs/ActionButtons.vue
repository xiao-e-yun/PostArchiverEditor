<script setup lang="ts">
import {isEmpty} from 'lodash-es'
import {Button} from '../ui/button'
import {ButtonGroup} from '../ui/button-group'

const data = defineModel<Record<string, unknown>>({required: true})

const emit = defineEmits<{
  save: []
  delete: []
}>()

const hasChanges = () => !isEmpty(data.value)

function discard() {
  data.value = {}
}
</script>

<template>
  <div class="flex justify-between">
    <Button variant="destructive" @click="emit('delete')">Delete</Button>
    <ButtonGroup v-if="hasChanges()" class="flex gap-2 justify-end">
      <Button variant="destructive" @click="discard">Discard</Button>
      <Button @click="emit('save')">Save</Button>
    </ButtonGroup>
  </div>
</template>
