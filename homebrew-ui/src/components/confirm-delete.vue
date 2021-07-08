<template lang="pug">
transition(name="modal-fade")
  .fixed.z-10.inset-0.overflow-y-auto
    .flex.items-end.justify-center.min-h-screen.py-4.px-4.pb-20.text-center(
      class="sm:block sm:p-0"
    )
      .fixed.inset-0.transition-opacity(aria-hidden)
        .absolute.inset-0.bg-gray-500.opacity-75

      // This element is to trick the browser into centering the modal contents.
      span.hidden.inline-block.align-middle.h-screen(aria-hidden) &#8203;
      .inline-block.align-bottom.bg-white.rounded-lg.text-left.overflow-hidden.shadow-xl.transform.transition-all.my-8.align-middle.max-w-lg.w-full(
        role="dialog",
        aria-modal="true",
        aria-labelledby="modal-headline"
      )
        .bg-white.px-4.py-5.pb-4(class="sm:p-6 sm:pb-4")
          .flex.items-start
            .mx-auto.flex-shrink-0.flex.items-center.justify-center.h-12.w-12.rounded-full.bg-red-100(
              class="sm:mx-0 sm:h-10 sm:w-10"
            )
              exclamation-icon.w-6.h-6.text-red-100
            .mt-3.text-center(class="sm:mt-0 sm:ml-4 sm:text-left")
              h3#modal-headline.text-lg.leading-6.font-medium.text-gray-900 {{ modalHeadline }}

              .mt-2
                p.text-sm.text-gray-500 {{ deleteMessage }}

        .bg-gray-50.px-4.py-3(class="sm:px-6 sm:flex sm:flex-row-reverse")
          button.w-full.inline-flex.justify-center.rounded-md.border.border-transparent.shadow-sm.px-4.py-2.bg-red-600.text-base.font-medium.text-white(
            type="button",
            @click="deleteRecord",
            class="hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"
          ) Delete
          button.mt-3.w-full.inline-flex.justify-center.rounded-md.border.border-gray-300.shadow-sm.px-4.py-2.bg-white.text-base.font-medium.text-gray-700(
            type="button",
            @click="close",
            class="hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
          ) Cancel
</template>

<script lang="ts">
import { ExclamationIcon } from "@heroicons/vue/outline";
import { computed, defineComponent } from "vue";

export default defineComponent({
  components: { ExclamationIcon },
  props: {
    modalHeadline: {
      type: String,
      required: true,
    },
    deleteMessage: {
      type: String,
      required: true,
    },
  },
  setup(props, { emit }) {
    return {
      close: () => {
        emit("close");
      },
      deleteRecord: () => {
        emit("deleteRecordEvent");
      },
    };
  },
});
</script>
