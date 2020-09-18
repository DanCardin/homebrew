<template lang="pug">
div
  h3
    span Brews&nbsp;
    a(@click="newBrew" class="badge")
      fa(icon="plus" size="xs")
  .row(v-for="brew in brews")
    .col
      .card.m-2.border-secondary
        .card-header.text-uppercase Name: {{ brew.name }}
        .card-body.text-secondary.p-0
          .mx-1
            input.form-control(id="targetInput")
</template>

<script lang="ts">
import { onMounted } from "vue";

import { brewsStore } from "@/client/store/brews";
import router from "@/client/router";

export default {
  setup() {
    const brews = brewsStore.getState().brews;

    onMounted(async () => {
      await brewsStore.getBrews();
    });

    async function newBrew() {
      let brew = await brewsStore.newBrew();
      router.push({name: 'brew', 'params': {brewId: brew.id} });
    }

    return {
      brews, newBrew
    }
  }
}
</script>
