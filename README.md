# Deno plugin

[Deno](https://deno.land/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands. Requires proto >= v0.12.

```shell
proto install deno-test
proto list-remote deno-test
```
