<script setup lang="ts">
import {computed, ref, triggerRef, watch} from "vue";
import type {WithRelations, Totalled} from "@/types";
import {useChangeable, useCommonSaveAPI, useDifference, usePlatforms} from "@/lib/utils";
import {refManualReset, useFetch, watchDeep} from "@vueuse/core";
import type {Alias} from "post-archiver";
import {Table, TableHeader, TableHead, TableRow, TableBody, TableCell} from "@/components/ui/table";
import {Button} from "@/components/ui/button";
import {Trash} from "lucide-vue-next";
import {ButtonGroup} from "@/components/ui/button-group";
import PlatformSelect from "@/components/platform/PlatformSelect.vue";

const props = defineProps<{
  author: number,
}>()

const url = computed(() => `/api/authors/${props.author}/aliases`)
const {data: raw} = useFetch(url, {refetch: true}).json<WithRelations<Totalled<Alias>>>();

const changed = useChangeable(raw)
const difference = useDifference(raw, changed)

const platforms = usePlatforms(raw)

function remove(index: number) {
  changed.value!.items.splice(index, 1)
  triggerChanged()
}

const triggerChanged = () => triggerRef(changed)

const getDefaultAlias = () => ({
  platform: null as unknown as number,
  target: props.author,
  source: "",
  link: "",
})

const newAliasCounter = ref(0)
const newAlias = ref<Alias>(getDefaultAlias())
watchDeep(newAlias, alias => {
  if (alias.source === "" || alias.platform === null) return
  changed.value!.items.push(alias)
  triggerChanged()
  newAlias.value = getDefaultAlias()
  newAliasCounter.value++
})

const save = useCommonSaveAPI("author's aliases", url, raw, changed, difference)
const reset = () => triggerRef(raw)
</script>

<template>
  <div class="border rounded-md">
    <div class="p-2 font-bold flex items-center justify-between">
      Aliases
      <div class="flex items-center gap-2">
        <ButtonGroup v-if="difference" class="h-6 inline-flex">
          <Button variant="ghost" class="p-2" @click="reset">
            Reset
          </Button>
          <Button variant="ghost" class="p-2" @click="save">
            Save
          </Button>
        </ButtonGroup>
        <span v-if="raw" class="text-sm text-muted">({{ raw.total }})</span>
      </div>
    </div>

    <Table class="overflow-auto">
      <TableHeader>
        <TableRow>
          <TableHead class="w-16">Platform</TableHead>
          <TableHead class="w-16">Source</TableHead>
          <TableHead class="w-64">Link</TableHead>
          <TableHead class="w-6"></TableHead>
        </TableRow>
      </TableHeader>
      <TableBody v-if="changed">
        <TableRow v-for="alias, i in changed.items" :key="alias.source">
          <TableCell>
            <PlatformSelect v-model="alias.platform" :platforms="platforms"
            @update:model-value="triggerChanged" class="w-full border-none! shadow-none!" />
          </TableCell>
          <TableCell>
            <input v-model.lazy="alias.source" @change="triggerChanged" class="p-1 w-full focus:outline-none bg-transparent field-sizing-content" />
          </TableCell>
          <TableCell>
            <input v-model.lazy="alias.link"  @change="triggerChanged" class="p-1 w-full focus:outline-none bg-transparent field-sizing-content" />
          </TableCell>
          <TableCell>
            <Button variant="destructive" size="icon" class="float-right" @click="remove(i)">
              <Trash class="w-4 h-4" />
            </Button>
          </TableCell>
        </TableRow>

        <TableRow>
          <TableCell>
            <PlatformSelect v-model="newAlias.platform" :platforms="platforms" class="w-full border-none! shadow-none!" :key="newAliasCounter" />
          </TableCell>
          <TableCell>
            <input v-model.lazy="newAlias.source" placeholder="Source" class="p-1 w-full focus:outline-none bg-transparent field-sizing-content" />
          </TableCell>
          <TableCell>
            <input v-model.lazy="newAlias.link" placeholder="Link (optional)" class="p-1 w-full focus:outline-none bg-transparent field-sizing-content" />
          </TableCell>
          <TableCell>
            <!-- Placeholder -->
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
