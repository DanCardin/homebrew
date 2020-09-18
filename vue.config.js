const path = require("path");

module.exports = {
  chainWebpack: config => {
    // config.resolve.alias.set("@", path.resolve("src/client"));
  },
  devServer: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:8000",
        pathRewrite: {
          "/api": ""
        }
      }
    }
  }
};
