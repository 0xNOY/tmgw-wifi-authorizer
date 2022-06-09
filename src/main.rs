use reqwest;
use std::env;

mod log;

fn main() {
    let logger = log::Log::new();
    macro_rules! exit_with_record_error {
        ($($arg:tt)*) => {{
            logger.record_error(&std::format!($($arg)*));
            logger.save();
            std::process::exit(1);
        }};
    }

    logger.record_info("ログインしています。");

    let user_id = match env::var("TMGW_ID") {
        Ok(s) => s,
        Err(_) => exit_with_record_error!("環境変数 TMGW_ID を定義してください。"),
    };
    let user_password = match env::var("TMGW_PASSWORD") {
        Ok(s) => s,
        Err(_) => exit_with_record_error!("環境変数 TMGW_PASSWORD を定義してください。"),
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
        Err(e) => exit_with_record_error!("ログインページへの接続に失敗しました。詳細: {}", e),
    };

    let res_body_str = match res.text_with_charset("EUC-JP") {
        Ok(s) => s,
        Err(_) => {
            exit_with_record_error!("サーバからのメッセージに不適切なバイト列が含まれています。")
        }
    };
    if res_body_str.contains("認証に成功しました。") {
        logger.record_info("ログインが完了しました。");
    } else if res_body_str.contains("ユーザ名かパスワードが間違っています。") {
        exit_with_record_error!("IDまたはパスワードが異なります。")
    } else {
        exit_with_record_error!("非予期のエラーが発生しました。")
    }

    logger.save();
}
