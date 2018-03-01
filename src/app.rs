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
        .subcommand(
            SubCommand::with_name("apps")
                .about("Operate lightning roar apps")
                .subcommand(SubCommand::with_name("list").about("Show lightning roar all apps")),
        );

    match app.get_matches().subcommand() {
        ("config", Some(matches)) => match matches.subcommand() {
            ("show", Some(_)) => Cmd::Config(ConfigCmd::Show),
            _ => Cmd::None("invalid config command!".to_string()),
        },
        ("apps", Some(matches)) => match matches.subcommand() {
            ("list", Some(_)) => Cmd::Apps(AppsCmd::List),
            _ => Cmd::None("invalid apps command!".to_string()),
        },
        _ => Cmd::None("invalid command!".to_string()),
    }
}

pub enum Cmd {
    Config(ConfigCmd),
    Apps(AppsCmd),
    None(String),
}

pub enum ConfigCmd {
    Show,
}

pub enum AppsCmd {
    List,
}
