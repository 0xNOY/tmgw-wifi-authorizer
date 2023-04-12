# TMGW WiFi Authorizer

## 概要

大学のWiFi接続時に外部ネットワークへアクセスするには認証が必要です。通常、ブラウザを介して認証を行いますが、このソフトウェアを用いて、ブラウザ無しで認証を行えます。WiFi接続時のイベント登録等すれば自動で認証を行うこともできます。自動化の詳細は [WiFi接続時に認証を自動化](#Windowsでの手順) をご覧ください。


## 使い方

1. [リリースページ](https://github.com/0xNOY/tmgw-wifi-authorizer/releases)から実行ファイルをダウンロードする。

1. 環境変数 `TMGW_ID` にMyPCアカウントのIDを、環境変数 `TMGW_PASSWORD` にMyPCアカウントのパスワードを設定する。

1. ダウンロードした実行ファイルを実行する。

> **Note**  
> 出力を確認するには、ターミナルから起動します。

> **Note**  
> リリースページからダウンロードできる実行ファイルは有効な署名がなされていないため、実行時に警告が表示されます。気になる方はこのリポジトリをクローンしてローカルでビルドしてください。`cargo build --release`でビルドできます。


### WiFi接続時に認証を自動化

#### Windowsでの手順

1. [使い方](#使い方) のセクションに記載されている通りに環境変数を設定する。

1. タスクスケジューラを起動する。以下の場所にあります。  
    `コントロールパネル` > `システムとセキュリティ` > `管理ツール` > `タスク スケジューラ`

1. メニューバーの `操作` > `タスクの作成` をクリックする。

1. 名前を分かり易いものに変更する。  
    例) `TMGW WiFi Authorizer`

1. 同ウィンドウのタブ `トリガー` > `新規` をクリックする。

1. `タスクの開始` を `イベント時` に変更する。

1. `ログ` を `Microsoft-Windows-NetworkProfile/Operational` に変更する。  
    `ソース` に `NetworkProfile` と入力する。  
    `イベント ID` に `10000` と入力する。

1. `遅延時間を指定する` にチェックを入れ、`1 秒間` と入力する。  
    `繰り返し間隔を指定する` にチェックを入れ、`5 分間`と入力する。  
    `継続時間` に `12 時間` と入力し `OK` をクリックする。

1. タブ `操作` > `新規` をクリックする。

1. `参照` をクリックし、ダウンロードした実行ファイルを選択し `OK` をクリックする。

1. タブ `条件` > `コンピュータをAC電源で...` のチェックを外し `OK` をクリックする。


#### Linuxでの手順

`NetworkManager`の`dispatcher`利用します。それについての詳細は [NetworkManager dispatcher を使用したネットワークサービス](https://wiki.archlinux.jp/index.php/NetworkManager#NetworkManager_dispatcher_.E3.82.92.E4.BD.BF.E7.94.A8.E3.81.97.E3.81.9F.E3.83.8D.E3.83.83.E3.83.88.E3.83.AF.E3.83.BC.E3.82.AF.E3.82.B5.E3.83.BC.E3.83.93.E3.82.B9) を参照してください。


## コンタクト

質問・要望は、お気軽に [Issues](https://github.com/0xNOY/tmgw-wifi-authorizer/issues) へどうぞ。


## ライセンス

TMGW WiFi Authorizer は [Apache License, Version 2.0](https://github.com/0xNOY/tmgw-wifi-authorizer/blob/main/LICENSE) に基づいて公開されています。
