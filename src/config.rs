extern crate toml;
use std::string::String;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Read};
use std::io;
use std::env;
pub fn print() {
    let config = RawConfig {
        host: "localhost:3009/roar/",
    };

    let serialized = toml::to_string(&config).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: RawConfig = toml::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let filename = "~/.roar/config.toml";
    let fname = "./roar.toml";
    let path = Path::new(fname);
    println!("path : {:?}", path);
    println!("path components : {:?}", path.components());

    match env::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }
    // `File`構造体の`open`関連関数でファイルを開ける。
    // 失敗する可能性があるので結果は`Result`で返される。
    // 下の方でもう一度`filename`を使うためにここでは`&filename`と参照で渡していることに注意。
    let file = match File::open(path) {
        // 成功すれば取り出す。
        Ok(file) => file,
        // ファイルが見つからないなどのエラーの場合はそのままプログラム終了
        Err(e) => {
            println!("An error occurred while opening file {}:{}", fname, e);
            return;
        }
    };

    get_raw_config(file);
    // Fileをそのまま使うと遅いのと`lines`メソッドを使うために`BufReader`に包む。
    // この`new`もただの関連関数。
    // let input = BufReader::new(file);
    // // `BufReader`が実装するトレイトの`BufRead`にある`lines`メソッドを呼び出す。
    // // 返り値はイテレータなので`for`式で繰り返しができる
    // for line in input.lines() {
    //     // 入力がUTF-8ではないなどの理由で行のパースに失敗することがあるので
    //     // `line`もResultに包まれている。
    //     let line = match line {
    //         Ok(line) => line,
    //         // 失敗したらそのまま終了することにする。
    //         Err(e) => {
    //             println!("An error occurred while reading a line {}", e);
    //             return;
    //         }
    //     };
    //     println!("{}", line);
    // }
}

fn get_raw_config<'a>(file: File) -> Result<RawConfig<'a>, ConfigError> {
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e| ConfigError::IOError(e))?;

    let res = toml::from_str::<RawConfig>(&buf).map_err(|e| ConfigError::ParseError(e))?;
    println!("res : {:?}", res);

    unimplemented!()
}

#[derive(Debug)]
enum ConfigError {
    IOError(io::Error),
    ParseError(toml::de::Error),
}

#[derive(Serialize, Deserialize, Debug)]
struct RawConfig<'a> {
    host: &'a str,
}

struct Config<'a> {
    host: &'a str,
}
