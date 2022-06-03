#![windows_subsystem = "windows"]

use reqwest;
use std::env;

static SHOULD_PRINT: bool = true;

macro_rules! print {
    ($($arg:tt)*) => {{
        if SHOULD_PRINT { std::print!($($arg)*) }
    }};
}

macro_rules! error {
    ($($arg:tt)*) => {{
        if SHOULD_PRINT { std::eprint!("\n[Error] {}", std::format!($($arg)*)) }
        std::process::exit(1);
    }};
}

fn main() {
    print!("ログインしています。\n");

    let user_id = match env::var("TMGW_ID") {
        Ok(s) => s,
        Err(_) => error!("環境変数 TMGW_ID を定義してください。"),
    };
    let user_password = match env::var("TMGW_PASSWORD") {
        Ok(s) => s,
        Err(_) => error!("環境変数 TMGW_PASSWORD を定義してください。\n"),
    };

    let url = "https://dhcp.tamagawa.ac.jp/index.cgi";
    let post_form_data = [("STAT", "1"), ("USER", &user_id), ("PASS", &user_password)];

    let client = reqwest::blocking::Client::new();
    let res = match client
        .post(url)
        .form(&post_form_data)
        .timeout(std::time::Duration::from_secs(6))
        .send()
    {
        Ok(r) => r,
        Err(e) => error!("ログインページへの接続に失敗しました。\n詳細: {}\n", e),
    };

    let res_body_str = match res.text_with_charset("EUC-JP") {
        Ok(s) => s,
        Err(_) => error!("サーバからのメッセージに不適切なバイト列が含まれています。"),
    };
    if res_body_str.contains("認証に成功しました。") {
        print!("ログインが完了しました。\n");
    } else if res_body_str.contains("ユーザ名かパスワードが間違っています。") {
        error!("IDまたはパスワードが異なります。\n")
    } else {
        error!("非予期のエラーが発生しました。\n")
    }
}
