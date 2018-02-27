extern crate toml;
use std::string::String;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Read};
use std::io;
use std::env;
pub fn print() -> Result {
    let config = Config {
        host: "localhost:3009/roar/".to_string(),
    };

    let serialized = toml::to_string(&config).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Config = toml::from_str(&serialized).unwrap();
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

    let config: Config = get_raw_config(file)?;
    Ok(config)
}

fn open_file() -> File {
    let home = "~/.roar/config.toml";
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
    unimplemented!()
}

fn parse_config(file: File) -> Result<Config, ConfigError> {
    let mut buf = String::new();
    BufReader::new(file)
        .read_to_string(&mut buf)
        .map_err(|e| ConfigError::IOError(e))?;
    let res = toml::from_str::<Config>(&buf).map_err(|e| ConfigError::ParseError(e))?;
    Ok(res)
}

#[derive(Debug)]
enum ConfigError {
    IOError(io::Error),
    ParseError(toml::de::Error),
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
}
