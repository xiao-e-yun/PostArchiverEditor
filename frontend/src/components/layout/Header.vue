<script setup lang="ts">
import { useActiveTab, useSettingsTab } from '@/utils'
import { breakpointsTailwind, useBreakpoints, useDark } from '@vueuse/core'
import { startCase } from 'lodash-es'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select'
import { MoonIcon, SunIcon } from 'lucide-vue-next'
import { CategoryType } from '@/category'

const activeTab = useActiveTab()
const settingsTab = useSettingsTab()

const list = [
  CategoryType.Post,
  CategoryType.Author,
  CategoryType.Tag,
  CategoryType.Collection,
  CategoryType.Platform,
  CategoryType.FileMeta,
] as const

const darkMode = useDark()
const breakpoints = useBreakpoints(breakpointsTailwind)
const isSm = breakpoints.greater('sm')
</script>

<template>
  <header class="border-b flex justify-between select-none bg-secondary">
    <div v-if="isSm" class="flex">
      <button
        v-for="item in list"
        :key="item"
        class="px-2 hover:bg-input"
        @click="activeTab = item; settingsTab = false"
        :class="{ 'bg-input': activeTab === item && !settingsTab }"
      >
        {{ startCase(item) }}
      </button>
    </div>
    <div v-else>
      <Select v-model="activeTab">
        <SelectTrigger class="w-32 border-none rounded-none cursor-pointer">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem v-for="item in list" :key="item" :value="item">
            {{ startCase(item) }}
          </SelectItem>
        </SelectContent>
      </Select>
    </div>
    <span class="text-sm opacity-50 flex items-center max-sm:hidden">Post Archiver Editor</span>
    <div class="flex">
      <button
        class="px-2 hover:bg-input"
        @click="settingsTab = !settingsTab"
        :class="{ 'bg-input': settingsTab }"
      >
        Settings
      </button>
      <button class="px-1 py-0.5 hover:bg-input" @click="darkMode = !darkMode">
        <MoonIcon v-if="darkMode" :size="18" />
        <SunIcon v-else :size="18" />
      </button>
    </div>
  </header>
</template>
