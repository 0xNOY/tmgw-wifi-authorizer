use dirs;
use log::{error, info};
use once_cell::sync::Lazy;
use reqwest;
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::{
    env,
    fs::{create_dir_all, File},
    path::PathBuf,
    process::exit,
};

const URL: &str = "https://dhcp.tamagawa.ac.jp/index.cgi";
const TIMEOUT_SECS: u64 = 6;

static APP_DATA_DIR_PATH: Lazy<PathBuf> = Lazy::new(|| match dirs::home_dir() {
    Some(d) => d.join(".tmgw-wifi-authorizer"),
    None => panic!("ホームディレクトリのパスを取得できません。"),
});
static LOG_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| APP_DATA_DIR_PATH.join("log.txt"));

fn main() {
    match create_dir_all(APP_DATA_DIR_PATH.clone()) {
        Ok(_) => (),
        Err(e) => panic!("データディレクトリの作成に失敗しました。詳細: {}", e),
    };

    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Off,
            simplelog::Config::default(),
            match File::create(LOG_FILE_PATH.clone()) {
                Ok(f) => f,
                Err(e) => panic!("ログファイルの作成に失敗しました。詳細: {}", e),
            },
        ),
    ]) {
        Ok(_) => (),
        Err(e) => panic!("ロガーの初期化に失敗しました。詳細: {}", e),
    }

    info!("ログインしています。");

    let user_id = match env::var("TMGW_ID") {
        Ok(s) => s,
        Err(_) => {
            error!("環境変数 TMGW_ID を定義してください。");
            exit(1)
        }
    };
    let user_password = match env::var("TMGW_PASSWORD") {
        Ok(s) => s,
        Err(_) => {
            error!("環境変数 TMGW_PASSWORD を定義してください。");
            exit(1)
        }
    };

    let post_form_data = [("STAT", "1"), ("USER", &user_id), ("PASS", &user_password)];

    let client = reqwest::blocking::Client::new();
    let res = match client
        .post(URL)
        .form(&post_form_data)
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            error!("ログインページへの接続に失敗しました。詳細: {}", e);
            exit(1)
        }
    };

    let res_body_str = match res.text_with_charset("EUC-JP") {
        Ok(s) => s,
        Err(_) => {
            error!("サーバからのメッセージに不適切なバイト列が含まれています。");
            exit(1)
        }
    };
    if res_body_str.contains("認証に成功しました。") {
        info!("ログインが完了しました。");
    } else if res_body_str.contains("ユーザ名かパスワードが間違っています。") {
        error!("IDまたはパスワードが異なります。");
        exit(1)
    } else {
        error!("非予期のエラーが発生しました。");
        exit(1)
    }
}
