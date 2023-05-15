const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const {GenerateSW} = require("workbox-webpack-plugin");

const production = process.env.PRODUCTION == 1;

const plugins = [
  new CopyWebpackPlugin([
    "index.html",
    "manifest.json",
    "icon.ico",
    "icon.png",
    "node_modules/bootstrap/dist/css/bootstrap.min.css",
    "node_modules/bootstrap/dist/js/bootstrap.bundle.min.js",
    "register_serviceworker.js",
  ]),
];

if (production) {
  plugins.push(
    new GenerateSW({
      swDest: "serviceworker.js",
    })
  );
}

module.exports = {
  entry: "./index_loader.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index_loader.js",
  },
  mode: production ? "production" : "development",
  plugins,
};
