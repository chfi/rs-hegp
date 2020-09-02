Rust HEGP animation
=========================

Build the Rust library using [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) from the repository root:

```bash
wasm-pack build
```

The example site itself is in the `www` directory, use NPM to get the
JS dependencies:

```bash
cd www
npm install
```

Webpack is used for packaging, and there are NPM scripts that takes
care of using it.

Start a development server:

```bash
npm run start
```

Build the site, output will be in the `dist` dir under `www`:

```bash
npm run build
```
