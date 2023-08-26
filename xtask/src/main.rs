#![deny(clippy::all)]
#![deny(unsafe_code)]

use cxx_auto_xtask as xtask;

// FIXME: implement `xtask codecheck` subcommand

fn main() -> xtask::BoxResult<()> {
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
    codecheck
    cmake
    doc
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

    let config = xtask::config::Config::load()?;

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

    let mut args = xtask::pico_args::Arguments::from_vec(args);

    let subcommand = args.subcommand()?;
    if let Some(subcommand) = subcommand.as_deref() {
        let mut context = xtask::command::Context::new(&config, &mut args, tool_args);
        let result = match subcommand {
            "build" => xtask::command::build(context),
            "check" => xtask::command::check(context),
            "clang" => {
                if let Some(subcommand) = context.args.opt_free_from_str::<String>()? {
                    if context.tool_args.is_empty() {
                        let default_args: &[&str] = match &*subcommand {
                            "format" => &["-r", "./cxx"],
                            "tidy" => &["-p", "build", "-quiet", "./cxx"],
                            _ => &[],
                        };
                        context.tool_args.extend(default_args.iter().map(Into::into));
                        context.current_dir = Some(config.cargo_metadata.workspace_root.clone());
                    }
                    context.subcommand = Some(subcommand);
                    xtask::command::clang(context)
                } else {
                    let help = xtask::command::clang::help();
                    println!("{help}\n");
                    Ok(None)
                }
            },
            "clippy" => xtask::command::clippy(context),
            "cmake" => xtask::command::cmake(context),
            "doc" => xtask::command::doc(context),
            "fmt" => xtask::command::fmt(context),
            "miri" => xtask::command::miri(context),
            "tarpaulin" => xtask::command::tarpaulin(context),
            "test" => xtask::command::test(context),
            "udeps" => xtask::command::udeps(context),
            "valgrind" => xtask::command::valgrind(context),
            "help" => {
                println!("{help}\n");
                Ok(None)
            },
            subcommand => Err(format!("unrecognized subcommand `{subcommand}`").into()),
        };
        xtask::handler::subcommand_result(subcommand, result);
        xtask::handler::result(xtask::handler::unused(&args));
    } else {
        println!("{help}\n");
    }

    Ok(())
}
