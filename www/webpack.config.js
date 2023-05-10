const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      'index.html',
      'serviceworker/register_serviceworker.js',
      'serviceworker/serviceworker.js',
      'manifest.json',
      'icon/icon.ico',
      'icon/icon.png',
    ])
  ],
};
