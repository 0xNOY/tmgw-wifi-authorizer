use clap::Parser;
use dirs;
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use reqwest;
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    path::PathBuf,
    process::exit,
};

const TMGW_ID_ENV_KEY: &str = "TMGW_ID";
const TMGW_PASSWORD_ENV_KEY: &str = "TMGW_PASSWORD";

static APP_DATA_DIR_PATH: Lazy<PathBuf> = Lazy::new(|| match dirs::home_dir() {
    Some(d) => d.join(".tmgw-wifi-authorizer"),
    None => panic!("Failed to get home directory."),
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
    // ID of MyPCAccount
    #[arg(short = 'n', long, env = TMGW_ID_ENV_KEY)]
    tmgw_id: String,

    // password of MyPCAcount
    #[arg(short = 'p', long, env = TMGW_PASSWORD_ENV_KEY)]
    tmgw_password: String,

    // URL of authentication page
    #[arg(
        short = 'u',
        long,
        default_value = "https://dhcp.tamagawa.ac.jp/index.cgi"
    )]
    url: String,

    // Timeout seconds
    #[arg(short = 't', long, default_value = "6")]
    timeout_secs: u64,

    // app data directory path
    #[arg(short = 'd', long, default_value = APP_DATA_DIR_PATH.to_str().unwrap())]
    data_dir_path: PathBuf,

    // log file name
    #[arg(short = 'l', long, default_value = "log.txt")]
    log_file_name: String,
}

fn main() {
    let args = Args::parse();

    match create_dir_all(APP_DATA_DIR_PATH.clone()) {
        Ok(_) => (),
        Err(e) => panic!("Failed to create app data directory. Details: {}", e),
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
                Err(e) => panic!("Failed to open log file. Details: {}", e),
            },
        ),
    ]) {
        Ok(_) => (),
        Err(e) => panic!("Failed to initialize logger. Details: {}", e),
    }

    if args.tmgw_id.is_empty() {
        warn!(
            "{} is empty. Please set environment variable {} or use -n option.",
            TMGW_ID_ENV_KEY, TMGW_ID_ENV_KEY
        );
    } else {
        debug!("TMGW_ID: {}", args.tmgw_id);
    }
    if args.tmgw_password.is_empty() {
        warn!(
            "{} is empty. Please set environment variable {} or use -p option.",
            TMGW_PASSWORD_ENV_KEY, TMGW_PASSWORD_ENV_KEY
        );
    }
    if args.tmgw_id.is_empty() || args.tmgw_password.is_empty() {
        exit(1);
    }

    let form_data = [
        ("STAT", "1"),
        ("USER", &args.tmgw_id),
        ("PASS", &args.tmgw_password),
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
            error!("Failed to send request. Details: {}", e);
            exit(1)
        }
    };

    let res_body_str = match res.text_with_charset("EUC-JP") {
        Ok(s) => s,
        Err(_) => {
            error!("Find invalid character in response body.");
            exit(1)
        }
    };

    if res_body_str.contains("認証に成功しました。") {
        info!("Successfully authenticated.");
    } else if res_body_str.contains("ユーザ名かパスワードが間違っています。") {
        error!("User ID or password is incorrect.");
        exit(1)
    } else {
        error!("An unexpected error has occurred.");
        exit(1)
    }
}
