# Deno plugin

[Deno](https://deno.land/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

This plugin is built-in to proto, but if you want to override it with an explicit version, add the following to `.prototools`.

```toml
[plugins]
deno = "source:https://github.com/moonrepo/deno-plugin/releases/download/vX.Y.Z/deno_plugin.wasm"
```

## Configuration

Deno plugin does not support configuration.

## Hooks

Deno plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install deno-test
proto list-remote deno-test
```
