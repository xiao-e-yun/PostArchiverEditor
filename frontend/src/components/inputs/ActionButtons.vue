<script setup lang="ts">
import { isEmpty } from 'lodash-es'
import { Button } from '../ui/button'
import { ButtonGroup } from '../ui/button-group'
import { computed } from 'vue'
import { injectData } from '../main/utils'

const data = injectData()
const emit = defineEmits<{
  save: []
  delete: []
}>()

const isChanged = computed(() => !isEmpty(data.value.changes))

const discard = () => (data.value.changes = {})
</script>

<template>
  <div class="flex justify-between">
    <Button variant="destructive" @click="emit('delete')">Delete</Button>
    <ButtonGroup v-if="isChanged" class="flex gap-2 justify-end">
      <Button variant="destructive" @click="discard">Discard</Button>
      <Button @click="emit('save')">Save</Button>
    </ButtonGroup>
  </div>
</template>
