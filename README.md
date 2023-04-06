# Will probably dioxus server fn example

_Dioxus server fns!_

## Quickstart

If you don't care about wasm size, you can ignore the rest of the stuff below

```sh
git clone https://github.com/swlkr/dioxus-server-fns
cd dioxus-server-fns
dioxus build --features front
cargo run --features ssr --no-default-features
```

*Quick aside, do not change the name of the ssr feature. I tried "backend" and it fails because there's a macro in the server_fn_macro crate that's hardcoded with #[cfg(feature = "ssr")]*

Then visit the page in your browser:

```
open http://localhost:9001
```

## Optimizing WASM Size

You do care about wasm size after all, eh? Welcome to the bleeding edge.

This uses a fork of the dioxus-cli that adds plugin support to build a smaller version of the wasm file.

We'll grab that first:

```sh
git clone https://github.com/mrxiaozhuox/dioxus-cli
cd dioxus-cli
cargo install --path .
```

Next up, we'll need to install the wasm-opt-rs crate and plugin:

```sh
cargo install wasm-opt --locked
# make sure you're in the project root (single-file-dioxus-ssr-hydrate)
dioxus plugin add --git https://github.com/brson/wasm-opt-rs.git
```

Finally:

```sh
dioxus build --release --features frontend
```

Should give you a wasm size around ~280KB
