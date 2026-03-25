<script setup lang="ts">
import {ref} from 'vue';
import {Button} from '../ui/button';
import {ButtonGroup} from '../ui/button-group';
import {Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger} from '../ui/dialog';

const open = ref(false)

defineProps<{
	disabled?: boolean
}>()

defineEmits<{
	(e: 'save'): void
	(e: 'delete'): void
}>()
</script>

<template>
	<ButtonGroup class="ml-auto">
		<Dialog v-model:open="open">
			<DialogTrigger as-child>
				<Button variant="destructive" size="sm">
					Delete
				</Button>
			</DialogTrigger>
			<DialogContent :showCloseButton="false">
				<DialogHeader>
					<DialogTitle>Are you absolutely sure?</DialogTitle>
					<DialogDescription>
						This action cannot be undone.
					</DialogDescription>
				</DialogHeader>
        <div class="ml-auto flex gap-4">
          <Button @click="open = false">
            Cancel
          </Button>
          <Button variant="destructive" @click="$emit('delete')">
            Delete
          </Button>
        </div>
			</DialogContent>
    </Dialog>
    <Button size="sm" @click="$emit('save')" :disabled="disabled">
      Save
    </Button>
	</ButtonGroup>
</template>
