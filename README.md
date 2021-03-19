# WASMi Coremark

This repo provides script for running the [coremark-minimal.wasm][0] using 
wasmtime and wasmi.

## Usage

```
usage: bm [wasmitime|wasm3|wasmi: string] [times: number]
```

## CoreMark

### Test Machine

| CPU Model                           | CPU Speed | RAM Speed | CPU Cores | MEM  |
|-------------------------------------|-----------|-----------|-----------|------|
| AMD Ryzen 9 5900X 12-Core Processor | 2061.209  | 3200 MT/S | 24        | 64GB |

### Result

| wasmtime  | wasm3     | wasmi    |
|-----------|-----------|----------|
| 20885.547 | 1672.8003 | 516.1896 |

The `coremark-minimal.wasm` we are using here does not produce text output like [coremark][1], just the final test result. 


## LICENSE

MIT

[0]: https://github.com/wasm3/wasm-coremark
[1]: https://github.com/eembc/coremark#log-file-format
