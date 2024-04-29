#[derive(Debug)]
pub enum ErrNo {
    Success = 0,              // No error occurred.
    InvalidArgument = 1,      // Caller module passed an invalid argument.
    InvalidEncoding = 2,      // Invalid encoding.
    MissingMemory = 3,        // Caller module is missing a memory export.
    Busy = 4,                 // Device or resource busy.
    RuntimeError = 5,         // Runtime Error.
    UnsupportedOperation = 6, // Unsupported Operation.
    TooLarge = 7,             // Too Large.
    NotFound = 8,             // Not Found.
}

mod wasi_nn {
    use crate::ErrNo;
    use ::function_name::named;
    use wasmedge_plugin_sdk::{
        error::CoreError,
        memory::Memory,
        module::{PluginModule, SyncInstanceRef},
        types::{ValType, WasmVal},
    };

    fn parse_opts() {
        println!("handle opt");
        unsafe {
            println!("nn-preload: {:?}", (*crate::nn_preload()).to_string());
        }
    }

    pub fn create_module() -> PluginModule<()> {

        #[named]
        fn load<'a>(
            _inst: &'a mut SyncInstanceRef,
            _memory: &'a mut Memory,
            _data: &'a mut (),
            _args: Vec<WasmVal>,
        ) -> Result<Vec<WasmVal>, CoreError> {
            println!("host function: {:?}", function_name!());
            Ok(vec![WasmVal::I32(ErrNo::Success as i32)])
        }

        #[named]
        fn load_by_name<'a>(
            _inst: &'a mut SyncInstanceRef,
            _main_memory: &'a mut Memory,
            _data: &'a mut (),
            _args: Vec<WasmVal>,
        ) -> Result<Vec<WasmVal>, CoreError> {
            println!("host function: {:?}", function_name!());
            Ok(vec![WasmVal::I32(ErrNo::Success as i32)])
        }

        #[named]
        fn init_execution_context<'a>(
            _inst: &'a mut SyncInstanceRef,
            _main_memory: &'a mut Memory,
            _data: &'a mut (),
            _args: Vec<WasmVal>,
        ) -> Result<Vec<WasmVal>, CoreError> {
            println!("host function: {:?}", function_name!());
            Ok(vec![WasmVal::I32(ErrNo::Success as i32)])
        }

        #[named]
        fn set_input<'a>(
            _inst: &'a mut SyncInstanceRef,
            _main_memory: &'a mut Memory,
            _data: &'a mut (),
            _args: Vec<WasmVal>,
        ) -> Result<Vec<WasmVal>, CoreError> {
            println!("host function: {:?}", function_name!());
            Ok(vec![WasmVal::I32(ErrNo::Success as i32)])
        }

        #[named]
        fn compute<'a>(
            _inst: &'a mut SyncInstanceRef,
            _main_memory: &'a mut Memory,
            _data: &'a mut (),
            _args: Vec<WasmVal>,
        ) -> Result<Vec<WasmVal>, CoreError> {
            println!("host function: {:?}", function_name!());
            Ok(vec![WasmVal::I32(ErrNo::Success as i32)])
        }

        #[named]
        fn get_output<'a>(
            _inst: &'a mut SyncInstanceRef,
            _main_memory: &'a mut Memory,
            _data: &'a mut (),
            _args: Vec<WasmVal>,
        ) -> Result<Vec<WasmVal>, CoreError> {
            println!("host function: {:?}", function_name!());
            Ok(vec![WasmVal::I32(ErrNo::Success as i32)])
        }

        parse_opts();

        let mut module = PluginModule::create("wasi_ephemeral_nn", ()).unwrap();
        module
            .add_func("load", (vec![ValType::I32; 5], vec![ValType::I32]), load)
            .unwrap();
        module
            .add_func(
                "load_by_name",
                (vec![ValType::I32; 3], vec![ValType::I32]),
                load_by_name,
            )
            .unwrap();
        module
            .add_func(
                "init_execution_context",
                (vec![ValType::I32; 2], vec![ValType::I32]),
                init_execution_context,
            )
            .unwrap();
        module
            .add_func(
                "set_input",
                (vec![ValType::I32; 2], vec![ValType::I32]),
                set_input,
            )
            .unwrap();
        module
            .add_func(
                "compute",
                (vec![ValType::I32; 2], vec![ValType::I32]),
                compute,
            )
            .unwrap();
        module
            .add_func(
                "get_output",
                (vec![ValType::I32; 2], vec![ValType::I32]),
                get_output,
            )
            .unwrap();
        module
    }
}

use wasi_nn::create_module;
use wasmedge_plugin_sdk::plugin::{option_string, register_plugin, OptionString};
register_plugin!(
    plugin_name = "wasi_nn",
    plugin_description = "burn framework adapter as wasi-nn plugin",
    version = (0,0,0,1),
    modules = [
        {"wasi_nn", "wasinn with burn backend module", create_module}
    ],
    options = [
        {
            "nn-preload",
            "Allow preload models from wasinn plugin. Each NN model can be specified as --nn-preload `COMMAND`.",
            OptionString,
            option_string!("none")
        }
    ]
);
