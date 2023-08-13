#![deny(clippy::all)]
#![deny(unsafe_code)]

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
    edit <EDITOR>
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

    let project_root = cxx_xtask::cargo::project_root()?;
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
        let result = match subcommand {
            "build" => cxx_xtask::command::build(&config, &mut args, tool_args),
            "check" => cxx_xtask::command::check(&config, &mut args, tool_args),
            "clang" => cxx_xtask::command::clang(&config, &mut args, tool_args),
            "clippy" => cxx_xtask::command::clippy(&config, &mut args, tool_args),
            "edit" => cxx_xtask::command::edit(&config, &mut args, tool_args),
            "doc" => cxx_xtask::command::doc(&config, &mut args, tool_args),
            "fmt" => cxx_xtask::command::fmt(&config, &mut args, tool_args),
            "miri" => cxx_xtask::command::miri(&config, &mut args, tool_args),
            "tarpaulin" => cxx_xtask::command::tarpaulin(&config, &mut args, tool_args),
            "test" => cxx_xtask::command::test(&config, &mut args, tool_args),
            "udeps" => cxx_xtask::command::udeps(&config, &mut args, tool_args),
            "valgrind" => cxx_xtask::command::valgrind(&config, &mut args, tool_args),
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
