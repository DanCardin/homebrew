/* eslint-disable @typescript-eslint/no-var-requires */
const path = require("path");

module.exports = {
  alias: {
    "/@client/": path.resolve(__dirname, "src/client"),
  },
  proxy: {
    "/api": {
      target: "http://127.0.0.1:8000",
      rewrite: (path) => path.replace(/^\/api/, ""),
    },
  },
};
