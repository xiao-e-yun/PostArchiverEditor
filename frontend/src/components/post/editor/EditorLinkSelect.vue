<template>
  <Popover v-model:open="opened">
    <PopoverTrigger as-child>
      <Button variant="outline" :class="{'is-active': isActive()}" class="[&.is-active]:bg-secondary">
        <Link />
      </Button>
    </PopoverTrigger>
    <PopoverContent class="w-80" as-child @interact-outside.prevent>
      <Input v-model="url" lazy placeholder="Enter URL" class="mb-2" />
    </PopoverContent>
  </Popover>
</template>

<script setup lang="ts">
import {inject, ref, watch} from 'vue';
import {key} from '.';
import {Button} from '@/components/ui/button';
import {Link} from 'lucide-vue-next';
import {Popover, PopoverContent, PopoverTrigger} from '@/components/ui/popover';
import {Input} from '@/components/ui/input';
import {computed} from 'vue';

const editor = inject(key)!;

function isActive() {
  if (!editor.value) return false;
  return editor.value.isActive("link");
}

const url = computed({
  get() {
    return editor?.value?.getAttributes("link")?.href || "";
  },
  set(href: string) {
    if (!editor.value) return;

    const cmd = editor.value.chain().extendMarkRange("link");

    if (!href) {
      cmd.unsetLink().run();
      return;
    }

    cmd.setLink({href: href}).run();
  },
})

const opened = ref(false);
watch(() => !!editor?.value?.getAttributes("link")?.href, hasLink => opened.value = hasLink)
</script>
