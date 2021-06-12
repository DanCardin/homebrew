import { ref } from "vue";

export default function debounced(timeoutCount = 800, { onChange } = null) {
  let timeoutRef = null;
  const handle = ref("");
  const value = ref("");

  function listener(e) {
    if (timeoutRef !== null) {
      clearTimeout(timeoutRef);
    }
    handle.value = e.target.value;

    function timeoutHandler() {
      value.value = e.target.value;
      if (onChange !== null) {
        onChange(value.value);
      }
    }

    timeoutRef = setTimeout(timeoutHandler, timeoutCount);
  }

  function reset() {
    handle.value = "";
    value.value = "";
  }

  return { handle, value, listener, reset };
}
