#![deny(clippy::all)]
#![deny(unsafe_code)]

use cxx_xtask::command::Context;

fn main() -> cxx_xtask::BoxResult<()> {
    let help = r#"
xtask

USAGE:
    xtask [SUBCOMMAND]

FLAGS:
    -h, --help          Prints this message or the help of the subcommand(s)

SUBCOMMANDS:
    build
    check
    clang
    clippy
    cmake
    doc
    edit [EDITOR]
    fmt
    help                Prints this message
    init
    miri
    tarpaulin
    test
    udeps
    valgrind
"#
    .trim();

    let project_root = cxx_xtask::workspace::project_root()?;
    let config = cxx_xtask::config::Config::load(&project_root)?;

    let mut args: Vec<_> = std::env::args_os().collect();
    // remove "xtask" argument
    args.remove(0);

    let tool_args = if let Some(dash_dash) = args.iter().position(|arg| arg == "--") {
        let tool_args = args.drain(dash_dash + 1 ..).collect();
        args.pop();
        tool_args
    } else {
        Vec::new()
    };

    let mut args = cxx_xtask::pico_args::Arguments::from_vec(args);

    let subcommand = args.subcommand()?;
    if let Some(subcommand) = subcommand.as_deref() {
        let mut context = Context::new(&config, &mut args, tool_args);
        let result = match subcommand {
            "build" => cxx_xtask::command::build(context),
            "check" => cxx_xtask::command::check(context),
            "clang" => {
                let subcommand: String = context.args.free_from_str()?;
                if context.tool_args.is_empty() {
                    let default_args: &[&str] = match &*subcommand {
                        "format" => &["-r", "./cxx"],
                        "tidy" => &["-p", "build", "-quiet", "./cxx"],
                        _ => &[],
                    };
                    context.tool_args.extend(default_args.iter().map(Into::into));
                    context.current_dir = Some(config.project_root_dir.clone());
                }
                context.subcommand = Some(subcommand);
                cxx_xtask::command::clang(context)
            },
            "clippy" => cxx_xtask::command::clippy(context),
            "cmake" => cxx_xtask::command::cmake(context),
            "edit" => {
                let (editor, editor_args) = cxx_xtask::detection::detect_editor(context.args)?;
                if context.tool_args.is_empty() {
                    let default_args: &[&str] = match &*editor {
                        "code" | "code-insiders" => &["./cxx-auto.code-workspace"],
                        _ => &[],
                    };
                    context.tool_args.extend(default_args.iter().map(Into::into));
                    context.current_dir = Some(config.project_root_dir.clone());
                }
                cxx_xtask::command::edit(context, &editor, editor_args)
            },
            "doc" => cxx_xtask::command::doc(context),
            "fmt" => cxx_xtask::command::fmt(context),
            "miri" => cxx_xtask::command::miri(context),
            "tarpaulin" => cxx_xtask::command::tarpaulin(context),
            "test" => cxx_xtask::command::test(context),
            "udeps" => cxx_xtask::command::udeps(context),
            "valgrind" => cxx_xtask::command::valgrind(context),
            "help" => {
                println!("{help}\n");
                Ok(None)
            },
            subcommand => Err(format!("unrecognized subcommand `{subcommand}`").into()),
        };
        cxx_xtask::handler::subcommand_result(subcommand, result);
        cxx_xtask::handler::result(cxx_xtask::handler::unused(&args));
    } else {
        println!("{help}\n");
    }

    Ok(())
}
