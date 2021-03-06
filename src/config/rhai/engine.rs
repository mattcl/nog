use super::{functions, lib, modules, syntax, types};
use crate::config::Config;
use lazy_static::lazy_static;
use log::{debug, error};
use rhai::{
    module_resolvers::{FileModuleResolver, ModuleResolversCollection},
    Engine, FnPtr, Scope,
};
use std::{
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use winapi::um::wingdi::{GetBValue, GetGValue, GetRValue, RGB};

lazy_static! {
    pub static ref MODE: Mutex<Option<String>> = Mutex::new(None);
    pub static ref ENGINE: Mutex<Engine> = Mutex::new(Engine::new());
    pub static ref SCOPE: Mutex<Scope<'static>> = Mutex::new(Scope::new());
    pub static ref AST: Mutex<rhai::AST> = Mutex::new(rhai::AST::default());
    pub static ref CALLBACKS: Mutex<Vec<FnPtr>> = Mutex::new(Vec::new());
}

pub fn add_callback(fp: FnPtr) -> usize {
    let mut callbacks = CALLBACKS.lock().unwrap();
    let idx = callbacks.len();
    callbacks.push(fp);
    idx
}

pub fn call(idx: usize) {
    let engine = ENGINE.lock().unwrap();
    let ast = AST.lock().unwrap();
    let callbacks = CALLBACKS.lock().unwrap();
    let _ = callbacks[idx]
        .call_dynamic(&*engine, &*ast, None, [])
        .map_err(|e| error!("{}", e.to_string()));
}

fn build_relative_resolver(config_path: &PathBuf) -> FileModuleResolver {
    FileModuleResolver::new_with_path_and_extension(config_path.clone(), "nog")
}

pub fn parse_config() -> Result<Config, String> {
    let mut engine = Engine::new();
    let mut scope = Scope::new();
    let mut config = Arc::new(Mutex::new(Config::default()));

    syntax::init(&mut engine, &mut config).unwrap();
    types::init(&mut engine);
    functions::init(&mut engine);
    lib::init(&mut engine);

    *CALLBACKS.lock().unwrap() = Vec::new();

    let mut resolver_collection = ModuleResolversCollection::new();

    let modules_resolver = modules::new();
    resolver_collection.push(modules_resolver);

    let mut config_path: PathBuf = dirs::config_dir().unwrap_or_default();

    config_path.push("nog");

    let relative_resolver = build_relative_resolver(&config_path);

    resolver_collection.push(relative_resolver);

    engine.set_module_resolver(Some(resolver_collection));
    engine.set_max_expr_depths(0, 0);

    if !config_path.exists() {
        debug!("nog folder doesn't exist yet. Creating the folder");
        std::fs::create_dir(config_path.clone()).map_err(|e| e.to_string())?;
    }

    config_path.push("config.nog");

    if !config_path.exists() {
        debug!("config file doesn't exist yet. Creating the file");
        if let Ok(mut file) = std::fs::File::create(config_path.clone()) {
            debug!("Initializing config with default values");
            file.write_all(include_bytes!("../../../assets/default_config.nog"))
                .map_err(|e| e.to_string())?;
        }
    }

    debug!("Parsing config file");
    let ast = engine
        .compile_file_with_scope(&scope, config_path)
        .map_err(|e| e.to_string())?;

    debug!("Running config file");
    engine
        .consume_ast_with_scope(&mut scope, &ast)
        .map_err(|e| e.to_string())?;

    *ENGINE.lock().unwrap() = engine;
    *SCOPE.lock().unwrap() = scope;
    *AST.lock().unwrap() = ast;

    let mut config = config.lock().unwrap().clone();

    config.bar.color = RGB(
        GetBValue(config.bar.color as u32),
        GetGValue(config.bar.color as u32),
        GetRValue(config.bar.color as u32),
    ) as i32;

    Ok(config)
}
