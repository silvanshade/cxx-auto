type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

fn main() -> BoxResult<()> {
    // NOTE: cxx-build an empty bridge so that `cxx/include/**/*.hxx` is exported to dependencies
    cxx_build::bridge("src/gen/ctypes.rs")
        .compiler("clang++")
        .flag_if_supported("-fno-rtti")
        .flag_if_supported("-std=gnu++2b")
        .flag_if_supported("-Werror")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-pedantic")
        .flag_if_supported("-Wno-ambiguous-reversed-operator")
        .flag_if_supported("-Wno-deprecated-anon-enum-enum-conversion")
        .flag_if_supported("-Wno-deprecated-builtins")
        .flag_if_supported("-Wno-dollar-in-identifier-extension")
        .flag_if_supported("-Wno-nested-anon-types")
        .flag_if_supported("-Wno-unused-parameter")
        .try_compile("cxx-auto")?;
    println!("cargo:rerun-if-changed=cxx");
    println!("cargo:rerun-if-changed=gen");
    Ok(())
}
