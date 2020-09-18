<template lang="pug">
div
  .card.m-2.border-secondary
    .card-header.text-uppercase Beer Info
    .card-body.text-secondary
      beer-info
    beer-targets

  .card.m-2.border-secondary
    .card-header.text-uppercase Ingredients
    .card-body.text-left.text-secondary
      .row
        .col-5
          table.table.table-bordered
            thead
              tr
                th(width='75%') Grain
                th Weight
            tbody
              tr
                td
                  input.form-control
                td
                  input.form-control
        .col-7
          table.table.table-bordered
            thead
              tr
                th(width='65%')
                  span Hops
                  span.text-sm.text-secondary &nbsp;(Form/AA)
                th(width='20%') Time
                th(width='15%') Weight
            tbody
              tr
                td
                  input.form-control
                td
                  input.form-control
                td
                  input.form-control
      .row
        .col-5
          table.table.table-bordered
            thead
              tr
                th(width='75%')
                  span H20 Treatments
                th Weight
            tbody
              tr
                td
                  input.form-control
                td
                  input.form-control
        .col-7
          table.table.table-bordered
            thead
              tr
                th(width='75%') Other Ingredients
                th Weight/Amt
            tbody
              tr
                td
                  input.form-control
                td
                  input.form-control
    .card-footer

  .card.m-2.border-secondary
    .card-header.text-uppercase Brew Milestones
    .card-body.text-secondary.p-0
      table.table.table-bordered.mb-0
        thead
          tr.text-uppercase
            th Strike
            th Mash In
            th Mash Out
            th Sparge
            th Boil
            th Knockout
        tbody
          tr.text-uppercase
            td.p-1
              .d-flex.text-sm.text-secondary
                .mx-1
                  label.mb-0(for="targetInput") time
                  input.form-control(id="targetInput")
                .mx-1
                  label.mb-0(for="actualInput") temp
                  input.form-control(id="actualInput")
                .mx-1
                  label.mb-0(for="actualInput") vol
                  input.form-control(id="actualInput")
            td.p-1
              .d-flex.text-sm.text-secondary
                .mx-1
                  label.mb-0(for="targetInput") time
                  input.form-control(id="targetInput")
                .mx-1
                  label.mb-0(for="actualInput") temp
                  input.form-control(id="actualInput")
            td.p-1
              .d-flex.text-sm.text-secondary
                .mx-1
                  label.mb-0(for="targetInput") time
                  input.form-control(id="targetInput")
                .mx-1
                  label.mb-0(for="actualInput") temp
                  input.form-control(id="actualInput")
            td.p-1
              .d-flex.text-sm.text-secondary
                .mx-1
                  label.mb-0(for="targetInput") time
                  input.form-control(id="targetInput")
                .mx-1
                  label.mb-0(for="actualInput") temp
                  input.form-control(id="actualInput")
                .mx-1
                  label.mb-0(for="actualInput") vol
                  input.form-control(id="actualInput")
            td.p-1
              .d-flex.text-sm.text-secondary
                .mx-1
                  label.mb-0(for="targetInput") time
                  input.form-control(id="targetInput")
                .mx-1
                  label.mb-0(for="actualInput") gravity
                  input.form-control(id="actualInput")
                .mx-1
                  label.mb-0(for="actualInput") vol
                  input.form-control(id="actualInput")
            td.p-1
              .d-flex.text-sm.text-secondary
                .mx-1
                  label.mb-0(for="targetInput") time
                  input.form-control(id="targetInput")
                .mx-1
                  label.mb-0(for="actualInput") gravity
                  input.form-control(id="actualInput")
                .mx-1
                  label.mb-0(for="actualInput") vol
                  input.form-control(id="actualInput")
    .card-footer

  .row
    .col-5
      .card.m-2.border-secondary
        .card-header.text-uppercase Fermentation
        .card-body.text-secondary.p-0
          .row.card-title.text-left.m-2
            .col
              label.mb-0(for="targetInput") Yeast
              input.form-control(id="targetInput")
            .col.text-right
              label.mb-0(for="targetInput") Volume
              input.form-control(id="targetInput")
          table.table.table-bordered.mb-0
            thead
              tr.text-uppercase
                th Date
                th Temp
                th Gravity
            tbody
              tr.text-uppercase
                td.p-1
                  .mx-1
                    input.form-control(id="targetInput")
                td.p-1
                  .mx-1
                    input.form-control(id="actualInput")
                td.p-1
                  .mx-1
                    input.form-control(id="actualInput")

    .col-7
      .row
        .col
          .card.m-2.border-secondary
            .card-header.text-uppercase Transfers
            .card-body.text-secondary.p-0
              table.table.table-bordered.mb-0
                thead
                  tr.text-uppercase
                    th Primary
                    th Secondary
                    th Packaging
                tbody
                  tr.text-uppercase
                    td.p-1
                      .d-flex.text-sm.text-secondary
                        .mx-1
                          label.mb-0(for="targetInput") Date
                          input.form-control(id="targetInput")
                        .mx-1
                          label.mb-0(for="actualInput") Gravity
                          input.form-control(id="actualInput")
                    td.p-1
                      .d-flex.text-sm.text-secondary
                        .mx-1
                          label.mb-0(for="targetInput") Date
                          input.form-control(id="targetInput")
                        .mx-1
                          label.mb-0(for="actualInput") Gravity
                          input.form-control(id="actualInput")
                    td.p-1
                      .d-flex.text-sm.text-secondary
                        .mx-1
                          label.mb-0(for="targetInput") Date
                          input.form-control(id="targetInput")
                        .mx-1
                          label.mb-0(for="actualInput") Gravity
                          input.form-control(id="actualInput")
      .row
        .col
          .card.m-2.border-secondary
            .card-header.text-uppercase Notes
            .card-body.text-secondary.p-0
              .mx-1
                input.form-control(id="targetInput")
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import BeerInfo from "@/client/components/beer-info.vue";
import BeerTargets from "@/client/components/beer-targets.vue";

@Options({ components: {BeerInfo, BeerTargets} })
export default class BrewLog extends Vue {}
</script>

<style scoped lang="scss">
  .text-sm {
    font-size: .6em;
  }
</style>
