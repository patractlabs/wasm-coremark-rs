use std::time::{SystemTime, UNIX_EPOCH};

/// Wasmtime coremark
fn wasmtime_coremark(b: &[u8]) {
    use wasmtime::{Linker, Module, Store};

    let store = Store::default();
    let mut linker = Linker::new(&store);

    linker
        .func("env", "clock_ms", || {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Clock may have gone backwards")
                .as_millis() as i32
        })
        .expect("Link clock_ms failed");

    let res = linker
        .instantiate(
            &Module::new(store.engine(), b).expect("Init wasm module failed in wasmtime coremark"),
        )
        .expect("Link module core-mark failed")
        .get_func("run")
        .expect("Could not find function `run` in the coremark")
        .call(&[])
        .expect("failed running coremark in wasmtime");

    println!("{:?}", res);
}

fn main() {
    let bytes = include_bytes!("coremark-minimal.wat");
    wasmtime_coremark(bytes);
}
