<template>
  <Popover>
    <PopoverTrigger as-child>
      <Button variant="outline">
        <Files />
      </Button>
    </PopoverTrigger>
    <PopoverContent class="w-64 md:w-sm flex flex-col gap-2 border-0 p-2 bg-transparent shadow-none">
      <ButtonGroup class="w-full gap-0!">
        <ButtonGroupText class="flex-1">
          <Files /> {{ count }}
        </ButtonGroupText>
        <ButtonGroupText class="w-full truncate">{{ path }}</ButtonGroupText>
        <Button variant="outline" @click="open">
          <Upload />
        </Button>
      </ButtonGroup>

      <!-- if over md, show 2 files per row, otherwise show 1 file per row -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
        <Button variant="outline" class="w-full" @click="insertFile">File 1</Button>
        <Button variant="outline" class="w-full" @click="insertFile">File 2</Button>
        <Button variant="outline" class="w-full" @click="insertFile">File 3</Button>
        <Button variant="outline" class="w-full" @click="insertFile">File 4</Button>
      </div>
    </PopoverContent>
  </Popover>
</template>

<script setup lang="ts">
import {computed, inject, ref} from 'vue';
import {key} from '.';
import {Button} from '@/components/ui/button';
import {Files, Upload} from 'lucide-vue-next';
import {Popover, PopoverContent, PopoverTrigger} from '@/components/ui/popover';
import {ButtonGroup, ButtonGroupText} from '@/components/ui/button-group';
import {useFileDialog} from '@vueuse/core';

const editor = inject(key)!;

const count = ref(1);
const postId = ref(1);

const path = computed(() => {
  return `/${Math.floor(postId.value / 2048)}/${postId.value % 2048}/`
})

const {open, onChange} = useFileDialog({multiple: true})
onChange(($files) => {
  if (!$files) return;
  const files = Array.from($files);

  console.log("Uploading: ", files.map(f => f.name))
})

function insertFile() {
  if (!editor.value) return;
  editor.value.chain().focus().insertContent({
    type: "image",
    attrs: {
      src: "https://example.com/file.pdf",
      name: "file.pdf",
      alt: "ID:114514",
    }
  }).run();
}
</script>
