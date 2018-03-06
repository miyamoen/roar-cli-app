use clap::{AppSettings, Arg, SubCommand};

pub fn create_command() -> Cmd {
    let app = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgsNegateSubcommands)
        .subcommands(vec![
            SubCommand::with_name("config")
                .about("Configure roar")
                .subcommands(vec![
                    SubCommand::with_name("show").about("Show configuration"),
                    SubCommand::with_name("new")
                        .about("Create new configuration file on your home directory")
                        .arg(
                            Arg::with_name("force")
                                .short("f")
                                .long("force")
                                .help("Over writing flag"),
                        ),
                ]),
            SubCommand::with_name("apps")
                .about("Operate lightning roar apps")
                .subcommands(vec![
                    SubCommand::with_name("list").about("Show all lightning roar apps"),
                    SubCommand::with_name("create")
                        .about("Create lightning roar app")
                        .arg(
                            Arg::with_name("name")
                                .help("App name that you want to register on lightning roar")
                                .required(true)
                                .takes_value(true)
                                .value_name("APP NAME"),
                        ),
                ]),
            SubCommand::with_name("app")
                .about("Operate lightning roar app")
                .arg(
                    Arg::with_name("app_id")
                        .help("App id that you want to send entry to on lightning roar")
                        .required(true)
                        .takes_value(true)
                        .value_name("APP ID"),
                )
                .arg(
                    Arg::with_name("title")
                        .help("Entry title")
                        .required(true)
                        .takes_value(true)
                        .value_name("ENTRY TITLE"),
                )
                .arg(
                    Arg::with_name("n")
                        .help("Send n messages")
                        .short("n")
                        .takes_value(true)
                        .value_name("N"),
                ),
        ]);

    match app.get_matches().subcommand() {
        ("config", Some(matches)) => match matches.subcommand() {
            ("show", Some(_)) => Cmd::Config(ConfigCmd::Show),
            ("new", Some(new_matches)) => {
                if new_matches.is_present("force") {
                    Cmd::Config(ConfigCmd::New(OverWriting::Force))
                } else {
                    Cmd::Config(ConfigCmd::New(OverWriting::NotExists))
                }
            }
            _ => Cmd::None("invalid config command!".to_string()),
        },
        ("apps", Some(matches)) => match matches.subcommand() {
            ("list", Some(_)) => Cmd::Apps(AppsCmd::List),
            ("create", Some(matches)) => {
                // `name` is a required value. `unwrap` does not panic.
                let app_name = matches.value_of("name").unwrap();
                Cmd::Apps(AppsCmd::Create(app_name.to_string()))
            }

            ("", _) => Cmd::Apps(AppsCmd::List),
            _ => Cmd::None("invalid apps command!".to_string()),
        },
        ("app", Some(matches)) => {
            // `app_id` and `title` is a required value. `unwrap` does not panic.
            let app_id = matches.value_of("app_id").unwrap().parse::<i32>().unwrap();
            let title = matches.value_of("title").unwrap().to_string();
            let n = matches
                .value_of("n")
                .map(|str| str.parse::<i32>().unwrap())
                .unwrap_or(1);
            Cmd::App(AppCmd::Send {
                app_id: app_id,
                title: title,
                n: n,
            })
        }
        _ => Cmd::None("invalid command!".to_string()),
    }
}

pub enum Cmd {
    Config(ConfigCmd),
    Apps(AppsCmd),
    App(AppCmd),
    None(String),
}

pub enum ConfigCmd {
    Show,
    New(OverWriting),
}

pub enum OverWriting {
    Force,
    NotExists,
}

pub enum AppsCmd {
    List,
    Create(String),
}

pub enum AppCmd {
    Send { app_id: i32, title: String, n: i32 },
}
