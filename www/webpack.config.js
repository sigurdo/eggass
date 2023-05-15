const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const {GenerateSW} = require("workbox-webpack-plugin");

module.exports = {
  entry: "./index_loader.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index_loader.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      "index.html",
      "register_serviceworker.js",
      "manifest.json",
      "icon.ico",
      "icon.png",
      "node_modules/bootstrap/dist/css/bootstrap.min.css",
      "node_modules/bootstrap/dist/js/bootstrap.bundle.min.js",
    ]),
    new GenerateSW({
      swDest: "serviceworker.js",
    }),
  ],
};
