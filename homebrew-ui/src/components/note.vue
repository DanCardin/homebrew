<template lang="pug">
button.h-5.w-5.rounded-full.items-center.justify-center.bg-yellow-200(
  class="hover:bg-yellow-200 hover:ring-1 hover:ring-yellow-500 hover:opacity-100",
  :class="buttonClass",
  @mouseover="onHover(true)",
  @mouseout="onHover(false)",
  @click="onClick($event)"
)
  .relative.mx-2.text-left(
    v-show="showNotes",
    @click.stop,
    @focusout="leaveTooltip",
    tabindex="-1"
  )
    .bg-yellow-200.text-yellow-500.border-2.border-yellow-600.text-xs.rounded.py-1.px-4.-left-4.bottom-2.bottom-full.absolute
      ul.list-inside.list-disc
        li.select-text(v-for="note of noteStore.get(batchId, target)") {{ note.time }} {{ note.value }}
      input.text-sm.placeholder-yellow-500.text-yellow-500.bg-yellow-200.border-0(
        class="focus:ring-0",
        type="text",
        placeholder="...",
        @keydown.enter.prevent="createNote($event.target.value)",
        :value="noteValue",
        ref="newNote"
      )
      svg.text-yellow-500.h-3.left-0.ml-3.top-full.absolute(
        x="0px",
        y="0px",
        viewBox="0 0 255 255",
        xml:space="preserve"
      )
        polygon.fill-current(points="0,0 127.5,127.5 255,0")

  span.text-yellow-600 +
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { useNoteStore } from "@/store/note";

export default defineComponent({
  props: {
    target: {
      type: String,
      required: true,
    },
    batchId: {
      type: Number,
      required: true,
    },
  },

  emits: ["click"],

  setup(props, { emit }) {
    const noteStore = useNoteStore();

    const hovering = ref(false);
    const clicked = ref(false);

    const showNotes = computed(() => clicked.value || hovering.value);

    const newNote = ref<HTMLInputElement | null>(null);
    const noteValue = ref("");

    const notes = ref([]);

    return {
      buttonClass: computed(() => {
        let classString = showNotes.value ? "opacity-100" : "opacity-30";
        if (clicked.value) {
          classString += " ring-1 ring-black hover:ring-black";
        }
        return classString;
      }),
      onHover(value) {
        hovering.value = value;
      },
      onClick(event) {
        clicked.value = !clicked.value;
        if (clicked.value && newNote.value) {
          newNote.value.focus();
        }
        event.stopPropagation();
      },
      async createNote(value) {
        if (!value) {
          return;
        }

        noteStore.newNote(props.batchId, props.target, value);
        noteValue.value = "";
      },
      leaveTooltip() {
        clicked.value = false;
      },
      showNotes,
      newNote,
      noteValue,
      clicked,

      noteStore,
    };
  },
});
</script>
