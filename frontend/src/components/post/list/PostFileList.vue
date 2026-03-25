<script setup lang="ts">
import {useEventBus, useFileDialog} from '@vueuse/core';
import type {FileMeta, FileMetaId, PostId} from 'post-archiver';
import {insertFileKey} from '../editor';
import {parseFileMetaToUrl, toastResponse} from '@/lib/utils';
import {File, Image, Plus, Upload, X} from 'lucide-vue-next';
import {ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from '@/components/ui/context-menu';
import {Button} from '@/components/ui/button';

const props = defineProps<{
  post: PostId
  fileMetas: FileMeta[]
}>()

const emit = defineEmits<{
  (e: 'refresh'): void
}>()

const thumb = defineModel<FileMetaId | null>("thumb")

const bus = useEventBus(insertFileKey)

async function deleteFileMeta(fileMeta: FileMeta) {
  if (
    await toastResponse(
      fetch(`/api/files/${fileMeta.id}`, {method: 'DELETE'}),
      "Deleted file",
      "Failed to delete file"
    )
  ) {
    emit('refresh')
    if (thumb.value === fileMeta.id) {thumb.value = null}
  }
}

const {open, onChange} = useFileDialog({
  reset: true,
})

onChange(async (files) => {
  if (!files?.length) return
  console.log("Uploading file", files)
  const formData = new FormData();

  for (const file of files) {
    formData.append('files', file)
  }

  if (
    await toastResponse(fetch(`/api/posts/${props.post}/files`, {
      method: 'PUT',
      body: formData,
    }), `Uploaded ${files.length} files`, "Failed to upload files")
  ) emit('refresh')
})
</script>

<template>
  <div class="rounded-md border px-2 pt-2">
    <h2 class="text-xl font-bold border-b pb-1 flex justify-between items-center">
      Files
      <Button variant="ghost" size="sm" @click="open">
        <Upload />
      </Button>
    </h2>
    <div v-if="fileMetas" class="flex flex-col gap-2 max-h-[80vh] overflow-auto py-2">
      <ContextMenu v-for="fileMeta in fileMetas" :key="fileMeta.id">
        <ContextMenuTrigger as="div" class="flex items-center gap-2 relative
                          border cursor-pointer rounded-md hover:bg-secondary/50" @click="bus.emit(fileMeta)">
          <div v-if="thumb === fileMeta.id" class="text-gray-500 absolute top-0 right-2">*</div>

          <img v-if="fileMeta.mime.startsWith('image/')" :src="parseFileMetaToUrl(fileMeta) + '?w=64&h=64'" alt="file"
            class="w-16 h-16 object-cover rounded-md">
          <File v-else class="p-2 w-16 h-16 bg-secondary rounded-md" />
          <span class="truncate pr-2">
            {{ fileMeta.filename }}</span>
        </ContextMenuTrigger>
        <ContextMenuContent>
          <ContextMenuItem @click="bus.emit(fileMeta)">
            <Plus />
            Insert to content
          </ContextMenuItem>
          <ContextMenuCheckboxItem @click="thumb = fileMeta.id" :model-value="thumb === fileMeta.id">
            Set as cover
          </ContextMenuCheckboxItem>
          <ContextMenuItem variant="destructive" @click="deleteFileMeta(fileMeta)">
            <X />
            Remove
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>
    </div>
  </div>
</template>
