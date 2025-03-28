use clap::{Arg, ArgAction, Command};

pub struct CliConfig {
    pub input: Option<String>,
    pub output: Option<String>,
    pub no_unset: bool,
    pub no_empty: bool,
}

pub fn parse_args() -> Result<CliConfig, String> {
    let matches = Command::new("envsubst")
        .version("1.0")
        .about("Environment variables substitution")
        .arg(Arg::new("input")
             .short('i')
             .long("input")
             .value_name("FILE")
             .help("Input file (default: stdin)")
             .num_args(1))
        .arg(Arg::new("output")
             .short('o')
             .long("output")
             .value_name("FILE")
             .help("Output file (default: stdout)")
             .num_args(1))
        .arg(Arg::new("no_unset")
             .long("no-unset")
             .action(ArgAction::SetTrue)
             .help("Fail if a variable is not set"))
        .arg(Arg::new("no_empty")
             .long("no-empty")
             .action(ArgAction::SetTrue)
             .help("Fail if a variable is set but empty"))
        .get_matches();

    Ok(CliConfig {
        input: matches.get_one::<String>("input").cloned(),
        output: matches.get_one::<String>("output").cloned(),
        no_unset: matches.get_flag("no_unset"),
        no_empty: matches.get_flag("no_empty"),
    })
}
