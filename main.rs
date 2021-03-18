/// ENV clock_ms
fn clock_ms() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_millis() as u32
}

/// Wasmtime coremark
fn wasmtime_coremark(b: &[u8]) {
    use wasmtime::{Linker, Module, Store};

    let store = Store::default();
    let mut linker = Linker::new(&store);

    linker
        .func("env", "clock_ms", || clock_ms())
        .expect("Link clock_ms failed");

    println!("Running CoreMark 1.0... ");

    let res = linker
        .instantiate(
            &Module::new(store.engine(), b).expect("Init wasm module failed in wasmtime coremark"),
        )
        .expect("Link module core-mark failed")
        .get_func("run")
        .expect("Could not find function `run` in the coremark")
        .call(&[])
        .expect("failed running coremark in wasmtime");

    println!("Result: {:?}", res);
}

/// WASMi coremark
fn wasmi_coremark(b: &[u8]) {
    use wasmi::{
        Error, Externals, FuncInstance, FuncRef, HostError, ImportsBuilder, ModuleImportResolver,
        ModuleInstance, RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType,
    };

    #[derive(Debug)]
    struct E;

    impl core::fmt::Display for E {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Execute imports functions in env failed")
        }
    }

    impl HostError for E {}

    /// Env resolver for wasmi
    struct EnvResolver;

    impl Externals for EnvResolver {
        fn invoke_index(
            &mut self,
            index: usize,
            _args: RuntimeArgs,
        ) -> Result<Option<RuntimeValue>, Trap> {
            match index {
                0 => Ok(Some(RuntimeValue::from(clock_ms()))),
                _ => Err(Trap::new(TrapKind::Host(Box::new(E)))),
            }
        }
    }

    impl ModuleImportResolver for EnvResolver {
        fn resolve_func(&self, field_name: &str, _signature: &Signature) -> Result<FuncRef, Error> {
            Ok(FuncInstance::alloc_host(
                Signature::new(&[][..], Some(ValueType::I32)),
                match field_name {
                    "clock_ms" => 0,
                    _ => {
                        return Err(Error::Instantiation(format!(
                            "Export {} not found",
                            field_name
                        )))
                    }
                },
            ))
        }
    }

    println!("Running CoreMark 1.0... ");

    let res = ModuleInstance::new(
        &wasmi::Module::from_buffer(
            wabt::wat2wasm(b).expect("Failed to parse `coremark-mininal.wat`"),
        )
        .expect("Failed to parse parsed `coremark-mininal.wasm`"),
        &ImportsBuilder::default().with_resolver("env", &EnvResolver),
    )
    .expect("Init wasmi module of coremark-minial failed")
    .assert_no_start()
    .invoke_export("run", &[], &mut EnvResolver);

    println!("Result: {:?}", res);
}

fn main() {
    let bytes = include_bytes!("coremark-minimal.wat");
    wasmtime_coremark(bytes);
    wasmi_coremark(bytes);
}
