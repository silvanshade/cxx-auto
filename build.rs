use serde::Deserialize;
use std::{collections::HashMap, process::Command};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct CxxAutoContext {
    cxx_compiler: String,
    compile_definitions: HashMap<String, String>,
    compile_options: Vec<String>,
}

impl CxxAutoContext {
    pub fn load(metadata: &cargo_metadata::Metadata) -> BoxResult<Self> {
        let mut cmd = Command::new("cmake");
        cmd.args(["-G", "Ninja"]);
        cmd.args(["-S", "."]);
        cmd.args(["-B", "build"]);
        cmd.status()?;

        let mut cmd = Command::new("cmake");
        cmd.args(["--build", "build"]);
        cmd.args(["--target", "emit_cxx_auto_context"]);
        cmd.status()?;

        let path = metadata.workspace_root.join("build/cxx-auto-context.json");
        let json = std::fs::read_to_string(path)?;
        let context = serde_json::from_str::<CxxAutoContext>(&json)?;

        Ok(context)
    }
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=cxx");
    println!("cargo:rerun-if-changed=gen");

    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    let cmake_context = CxxAutoContext::load(&metadata)?;

    let mut build = cxx_build::bridge("src/gen/ctypes.rs");
    build.compiler(cmake_context.cxx_compiler);
    for option in cmake_context.compile_options {
        build.flag_if_supported(&option);
    }
    for (definiendum, definiens) in cmake_context.compile_definitions {
        build.define(
            definiendum.as_ref(),
            if definiens.is_empty() {
                None
            } else {
                Some(definiens.as_ref())
            },
        );
    }
    build.try_compile("cxx-auto")?;

    Ok(())
}
