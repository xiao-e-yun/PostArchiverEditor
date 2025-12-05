<script setup lang="ts">
import JsonEditor from 'json-editor-vue'
import { Mode } from 'vanilla-jsoneditor'
import { useDark } from '@vueuse/core'

import 'vanilla-jsoneditor/themes/jse-theme-dark.css'

const model = defineModel<object>()

const isDark = useDark()
</script>

<template>
  <JsonEditor
    v-model="model"
    :class="{ 'jse-theme-dark': isDark }"
    :mode="Mode.tree"
    :main-menu-bar="false"
    :navigation-bar="false"
  />
</template>

<style>
.jse-contents {
  border-radius: 0.375rem; /* rounded-md */
}

.jse-tree-mode {
  border-radius: 0.375rem; /* rounded-md */
  border: 1px solid var(--border-input);
}
</style>