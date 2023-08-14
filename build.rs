use std::{path::PathBuf, process::Command};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

struct ClangConfig {
    suffix: String,
    version: String,
    matcher: String,
}

impl ClangConfig {
    pub fn load() -> BoxResult<ClangConfig> {
        let text = std::fs::read_to_string("xtask.toml")?;
        let toml = text.parse::<toml::Value>()?;

        let suffix = toml["clang"]["suffix"]
            .as_str()
            .map(String::from)
            .ok_or("missing `clang.suffix`")?;
        let version = toml["clang"]["version"]
            .as_str()
            .map(String::from)
            .ok_or("missing `clang.version`")?;
        let matcher = toml["clang"]["matchers"]["clang"]
            .as_str()
            .map(String::from)
            .ok_or("missing `clang.matchers.clang`")?;

        Ok(Self {
            suffix,
            version,
            matcher,
        })
    }
}

fn detect_clang_cxx() -> BoxResult<String> {
    let clang_config = ClangConfig::load()?;

    let tool = format!("clang++{}", &clang_config.suffix);
    if let Some(tool) = validate_clang_cxx(&clang_config.matcher, &tool)? {
        return Ok(tool);
    }

    let version = &clang_config.version;
    let major_version = version.split('.').next().unwrap_or(version);
    let formula = format!("llvm@{major_version}");
    if let Some(prefix) = detect_homebrew_prefix(&formula)? {
        if let Some(parent) = prefix.parent() {
            let path = parent.join(formula).join("bin").join("clang++");
            let tool = path.to_string_lossy();
            if let Some(tool) = validate_clang_cxx(&clang_config.matcher, &tool)? {
                return Ok(tool);
            }
        }
    }

    Err(format!("unable to find clang++{}", clang_config.suffix).into())
}

fn detect_homebrew_prefix(formula: &str) -> BoxResult<Option<PathBuf>> {
    let mut cmd = Command::new("brew");
    cmd.args(["--prefix", formula]);
    let output = cmd.output()?;
    if output.status.success() {
        if let Ok(prefix) = String::from_utf8(output.stdout) {
            return Ok(Some(PathBuf::from(prefix.trim())));
        }
    }
    Ok(None)
}

fn validate_clang_cxx(matcher: &str, tool: &str) -> BoxResult<Option<String>> {
    let matcher = regex::Regex::new(matcher)?;
    let mut cmd = Command::new(tool);
    cmd.arg("--version");
    if let Ok(output) = cmd.output() {
        if output.status.success() {
            let haystack = String::from_utf8(output.stdout)?;
            if let Some(version) = matcher
                .captures(&haystack)
                .and_then(|captures| captures.get(1).map(|m| m.as_str()))
            {
                if version.starts_with(version) {
                    return Ok(Some(String::from(tool)));
                }
            }
        }
    }
    Ok(None)
}

fn main() -> BoxResult<()> {
    let clang_cxx = detect_clang_cxx()?;
    println!("cargo:warning=detected clang++: {clang_cxx}");
    // NOTE: cxx-build an empty bridge so that `cxx/include/**/*.hxx` is exported to dependencies
    cxx_build::bridge("src/gen/ctypes.rs")
        .compiler(clang_cxx)
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
    println!("cargo:rerun-if-changed=xtask.toml");
    println!("cargo:rerun-if-changed=cxx");
    println!("cargo:rerun-if-changed=gen");
    Ok(())
}
