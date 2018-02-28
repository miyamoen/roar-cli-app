use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

pub fn create_command() -> Cmd {
    let app = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgsNegateSubcommands)
        .subcommand(
            SubCommand::with_name("config")
                .about("Configure roar")
                .subcommand(SubCommand::with_name("show").about("Show configuration")),
        )
        .subcommand(SubCommand::with_name("list").about("Show roar all feeds"));

    match app.get_matches().subcommand() {
        ("config", Some(matches)) => match matches.subcommand() {
            ("show", Some(_)) => Cmd::Config(ConfigCmd::Show),
            _ => Cmd::None("invalid config command!".to_string()),
        },
        ("list", Some(matches)) => Cmd::List,
        _ => Cmd::None("invalid command!".to_string()),
    }
}

pub enum Cmd {
    Config(ConfigCmd),
    List,
    None(String),
}

pub enum ConfigCmd {
    Show,
}
