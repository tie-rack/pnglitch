const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const mode = process.env.WEBPACK_MODE || 'development';

console.log(`⚙️  webpack mode: ${mode}\n`);

module.exports = {
  entry: "./js/index.js",
  mode: mode,
  output: {
    path: dist,
    filename: "bundle.js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "..", "pnglitch-wasm")
    }),
  ]
};
