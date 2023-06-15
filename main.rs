/// ENV clock_ms
fn clock_ms() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_millis() as u32;
    println!("clock_ms(): {now}");
    now
}

/// Wasmtime coremark
fn wasmtime_coremark(b: &[u8]) -> f32 {
    use core::slice;
    use wasmtime::{Linker, Module, Store, Val};

    let mut store = <Store<()>>::default();
    let engine = store.engine();
    let mut linker = Linker::new(&engine);

    linker
        .func_wrap("env", "clock_ms", || clock_ms())
        .expect("Link clock_ms failed");
    let module =
        Module::new(store.engine(), b).expect("Init wasm module failed in wasmtime coremark");

    let mut result = Val::F32(0);
    linker
        .instantiate(&mut store, &module)
        .expect("Link module core-mark failed")
        .get_func(&mut store, "run")
        .expect("Could not find function `run` in the coremark")
        .call(&mut store, &[], slice::from_mut(&mut result))
        .expect("Failed running coremark in wasmtime");
    match result {
        Val::F32(bits) => f32::from_bits(bits),
        _ => panic!(
            "failed running coremark using Wasmtime. expected F32 result but got: {:?}",
            result
        ),
    }
}

/// Wasm3 coremark
fn wasm3_coremark(b: &[u8]) -> f32 {
    use wasm3::{Environment, Module};

    let env = Environment::new().expect("Unable to create environment");
    let rt = env.create_runtime(2048).expect("Unable to create runtime");
    let mut module = rt
        .load_module(Module::parse(&env, &b[..]).expect("Unable to parse module"))
        .expect("Unable to load module");

    module
        .link_function::<(), u32>("env", "clock_ms", clock_ms_wrap)
        .expect("Unable to link function");

    module
        .find_function::<(), f32>("run")
        .expect("Unable to find function")
        .call()
        .expect("Calling coremark failed in wasm3")
}

wasm3::make_func_wrapper!(clock_ms_wrap: clock_ms() -> u32);

fn wasmi_coremark(wasm: &[u8]) -> f32 {
    use core::slice;
    use wasmi::{
        core::{F32},
        Value,
        Engine, Extern, Func, Linker, Module, Store,
    };

    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let mut linker = <Linker<()>>::new(&engine);
    let clock_ms = Func::wrap(&mut store, || clock_ms() as i32);
    linker
        .define("env", "clock_ms", clock_ms)
        .expect("failed to define `clock_ms` for wasmi");

    let module = Module::new(&engine, wasm)
        .expect("compiling and validating Wasm module failed in wasmi coremark");
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("linking module core-mark failed in wasmi")
        .ensure_no_start(&mut store)
        .expect("failed to start module instance in wasmi");
    let mut result = Value::F32(F32::from(0.0));
    let run = instance
        .get_export(&store, "run")
        .and_then(Extern::into_func)
        .expect("could not find function `run` in the coremark `.wasm`");
    run.call(&mut store, &[], slice::from_mut(&mut result))
        .expect("failed running coremark in wasmi");
    match result {
        Value::F32(value) => value.into(),
        unexpected => panic!("wasmi result expected `F32` but found: {:?}", unexpected),
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let help = || println!("usage: {} [wasmtime|wasm3|wasmi: string]", args[0]);
    let coremark_wasm = include_bytes!("coremark-minimal.wasm");

    match args.len() {
        2 => {
            let engine = args[1].as_str();

            println!(
                "Running Coremark 1.0 using {}... [should take 12..20 seconds]",
                engine
            );

            match engine {
                "wasmtime" => println!("Result: {}", wasmtime_coremark(coremark_wasm)),
                "wasm3" => println!("Result: {}", wasm3_coremark(coremark_wasm)),
                "wasmi" => println!("Result: {}", wasmi_coremark(coremark_wasm)),
                _ => help(),
            }
        }
        _ => help(),
    }
}
