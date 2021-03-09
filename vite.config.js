import path from "path";
import vue from "@vitejs/plugin-vue";

module.exports = {
  plugins: [vue()],
  alias: {
    "/@client": path.resolve(__dirname, "src/client"),
  },
  proxy: {
    "/api": {
      target: "http://127.0.0.1:8000",
      rewrite: (path) => path.replace(/^\/api/, ""),
    },
  },
};
