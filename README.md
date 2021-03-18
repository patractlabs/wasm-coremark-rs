# WASMi Coremark

This repo provides script for running the [coremark-minimal.wasm][0] using 
wasmtime and wasmi.

## Usage

```
usage: bm [wasmi|wasmitime: string] [times: number]
```

## Result

> 24 Cores, 64 GB MEM, ubuntu 20.04

| wasmtime   | wasm3    | wasmi    |
|------------|----------|----------|
| 1185203011 | 1753.258 | 15.01727 |


## LICENSE

MIT

[0]: https://github.com/wasm3/wasm-coremark
