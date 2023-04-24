use clap::Parser;
use dirs;
use log::{error, info};
use once_cell::sync::Lazy;
use reqwest;
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    path::PathBuf,
    process::exit,
};

static APP_DATA_DIR_PATH: Lazy<PathBuf> = Lazy::new(|| match dirs::home_dir() {
    Some(d) => d.join(".tmgw-wifi-authorizer"),
    None => panic!("ホームディレクトリのパスを取得できません。"),
});

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true
)]
struct Args {
    // MyPCアカウントのID
    #[arg(short = 'n', long, env = "TMGW_ID")]
    user_id: String,

    // MyPCアカウントのパスワード
    #[arg(short = 'p', long, env = "TMGW_PASSWORD")]
    user_password: String,

    // 認証サーバのURL
    #[arg(
        short = 'u',
        long,
        default_value = "https://dhcp.tamagawa.ac.jp/index.cgi"
    )]
    url: String,

    // タイムアウト時間(秒)
    #[arg(short = 't', long, default_value = "6")]
    timeout_secs: u64,

    // データディレクトリのパス
    #[arg(short = 'd', long, default_value = APP_DATA_DIR_PATH.to_str().unwrap())]
    data_dir_path: PathBuf,

    // ログファイルの名前
    #[arg(short = 'l', long, default_value = "log.txt")]
    log_file_name: String,
}

fn main() {
    let args = Args::parse();

    match create_dir_all(APP_DATA_DIR_PATH.clone()) {
        Ok(_) => (),
        Err(e) => panic!("データディレクトリの作成に失敗しました。詳細: {}", e),
    }

    let log_file_path = args.data_dir_path.join(args.log_file_name);

    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            simplelog::Config::default(),
            match OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_file_path)
            {
                Ok(f) => f,
                Err(e) => panic!("ログファイルの作成・書き込みに失敗しました。詳細: {}", e),
            },
        ),
    ]) {
        Ok(_) => (),
        Err(e) => panic!("ロガーの初期化に失敗しました。詳細: {}", e),
    }

    info!("ログインしています。");

    if args.user_id.is_empty() {
        error!("環境変数 TMGW_ID を定義してください。");
    }
    if args.user_password.is_empty() {
        error!("環境変数 TMGW_PASSWORD を定義してください。");
    }
    if args.user_id.is_empty() || args.user_password.is_empty() {
        exit(1);
    }

    let form_data = [
        ("STAT", "1"),
        ("USER", &args.user_id),
        ("PASS", &args.user_password),
    ];

    let client = reqwest::blocking::Client::new();
    let res = match client
        .post(args.url)
        .form(&form_data)
        .timeout(std::time::Duration::from_secs(args.timeout_secs))
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
        error!("IDまたはパスワードが間違っています。");
        exit(1)
    } else {
        error!("非予期のエラーが発生しました。");
        exit(1)
    }
}
