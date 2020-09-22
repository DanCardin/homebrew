<template lang="pug">
div
  h3
    span Brews &nbsp;
    a.badge(@click="newBrew")
      fa(icon="plus", size="xs")
  .row(v-for="brew in brews")
    .col
      .card.m-2.border-secondary
        .card-header.text-uppercase(@click="selectBrew(brew.id)")
          span Name: {{ brew.name }}
        .card-body.text-secondary.p-0
          .mx-1
            span
</template>

<script setup lang="ts">
import router from "/@client/router";
import { brewsStore } from "/@client/store/brews";
import { onMounted } from "vue";

export const brews = brewsStore.getState().brews;

onMounted(async () => {
  await brewsStore.getBrews();
});

export async function newBrew() {
  const brew = await brewsStore.newBrew();
  router.push({ name: "brew", params: { brewId: brew.id } });
}
export function selectBrew(id: number) {
  router.push({ name: "brew", params: { brewId: id } });
}
</script>
