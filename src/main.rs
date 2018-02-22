#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate reqwest;

use std::io::Read;
use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

fn main() {
    let app = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("pa")               // 位置引数を定義
            .help("sample positional argument")     // ヘルプメッセージ
            .required(true), // この引数は必須であることを定義
        )
        .arg(
            Arg::with_name("flg")              // フラグを定義
            .help("sample flag")                // ヘルプメッセージ
            .short("f")                         // ショートコマンド
            .long("flag"), // ロングコマンド
        )
        .arg(
            Arg::with_name("opt")              // オプションを定義
            .help("sample option")              // ヘルプメッセージ
            .short("o")                         // ショートコマンド
            .long("opt")                        // ロングコマンド
            .takes_value(true), // 値を持つことを定義
        )
        .subcommand(
            SubCommand::with_name("sub")// サブコマンドを定義
            .about("sample subcommand")         // このサブコマンドについて
            .arg(Arg::with_name("subflg")       // フラグを定義
                .help("sample flag by sub")     // ヘルプメッセージ
                .short("f")                     // ショートコマンド
                .long("flag")                   // ロングコマンド
            ),
        );

    // 引数を解析
    let matches = app.get_matches();

    // paが指定されていれば値を表示
    if let Some(o) = matches.value_of("pa") {
        println!("Value for pa: {}", o);
    }

    // optが指定されていれば値を表示
    if let Some(o) = matches.value_of("opt") {
        println!("Value for opt: {}", o);
    }

    // flgのON/OFFで表示するメッセージを切り替え
    println!(
        "flg is {}",
        if matches.is_present("flg") {
            "ON"
        } else {
            "OFF"
        }
    );

    // subサブコマンドの解析結果を取得
    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub"); // subが指定されていればメッセージを表示
                              // subflgのON/OFFで表示するメッセージを切り替え
        println!(
            "subflg is {}",
            if matches.is_present("subflg") {
                "ON"
            } else {
                "OFF"
            }
        );
    }
    // let mut resp = reqwest::get("https://www.rust-lang.org").unwrap();
    //
    // let body = resp.text().unwrap();
    // println!["{:?}", body];

    let point = Point { x: 1, y: 2 };

    let serialized = toml::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = toml::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
