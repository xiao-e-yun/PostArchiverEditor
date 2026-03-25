<script setup lang="ts">
import {Folders, Newspaper, Quote, Tags, UsersRound} from "lucide-vue-next";
</script>

<template>
  <div class="flex border-t">
    <RouterLink to="/posts" class="hover:bg-secondary [&.router-link-active]:bg-secondary">
      <Newspaper class="p-1 w-10 h-10" />
    </RouterLink>
    <RouterLink to="/authors" class="hover:bg-secondary [&.router-link-active]:bg-secondary">
      <UsersRound class="p-1 w-10 h-10" />
    </RouterLink>
    <RouterLink to="/tags" class="hover:bg-secondary [&.router-link-active]:bg-secondary">
      <Tags class="p-1 w-10 h-10" />
    </RouterLink>
    <RouterLink to="/collections" class="hover:bg-secondary [&.router-link-active]:bg-secondary">
      <Folders class="p-1 w-10 h-10" />
    </RouterLink>
    <RouterLink to="/platforms" class="hover:bg-secondary [&.router-link-active]:bg-secondary">
      <Quote class="p-1 w-10 h-10" />
    </RouterLink>
  </div>
</template>
