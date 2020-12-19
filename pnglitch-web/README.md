# PNGlitch Web

A web page interface for PNGlitch. Available at
[kilosecond.com/pnglitch][pnglitch].

## Developing

pnglitch-web uses [webpack][webpack] to build and bundle
pnglitch-wasm.

* `npm run start` – Build and serve the project locally for
  development at `http://localhost:8080`.

* `npm run build` – Bundle the project for deployment. For a
  production build, set the `WEBPACK_MODE` env var to `production`.

[webpack]: https://webpack.js.org/
[pnglitch]: https://kilosecond.com/pnglitch/
