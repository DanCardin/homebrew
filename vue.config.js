/* eslint-disable @typescript-eslint/no-var-requires */
const path = require("path");

module.exports = {
  pwa: {
    name: "Homebrew",
    iconPaths: {
      favicon16: "favicon.svg",
      favicon32: "favicon.svg",
      appleTouchIcon: "favicon.svg",
      maskIcon: "favicon.svg",
      msTileImage: "favicon.svg",
    },
  },
  configureWebpack: {
    devtool: "source-map",
  },
  chainWebpack: (config) => {
    config.resolve.alias.set("/@client", path.resolve("src/client"));
  },
  devServer: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:8000",
        pathRewrite: {
          "/api": "",
        },
      },
    },
  },
};
