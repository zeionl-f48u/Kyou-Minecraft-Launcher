<script setup lang="ts">
import PageHeader from '@/layouts/components/PageHeader.vue'
</script>

<template>
  <main class="p-5 flex flex-col h-screen box-border">
    <PageHeader title="设置" />
    <section class="flex flex-1 flex-col min-h-0">
      <slot />
    </section>
  </main>
</template>
