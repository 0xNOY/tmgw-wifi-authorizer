# TMGW WiFi Authorizer

## 概要

大学のWiFi接続時に外部ネットワークへアクセスするには認証が必要です。通常、ブラウザを介して認証を行いますが、このソフトウェアを用いて、ブラウザ無しで認証を行えます。WiFi接続時のイベント登録等すれば自動で認証を行うこともできます。自動化の詳細は [WiFi接続時に認証を自動化](#wifi接続時の認証を自動でする) をご覧ください。


## 使い方

1. [リリースページ](https://github.com/0xNOY/tmgw-wifi-authorizer/releases)から実行ファイルをダウンロードする。

1. 環境変数 `TMGW_ID` にMyPCアカウントのIDを、環境変数 `TMGW_PASSWORD` にMyPCアカウントのパスワードを設定する。

1. ダウンロードした実行ファイルを実行する。

> **Note**  
> リリースページからダウンロードできる実行ファイルは有効な署名がなされていないため、実行時に警告が表示されます。気になる方はこのリポジトリをクローンしてローカルでビルドしてください。`cargo build --release`でビルドできます。


### WiFi接続時の認証を自動でする

#### Windowsでの手順

[Windowsで認証を自動化する](docs/automation-win.md) を参照してください。

#### Linuxでの手順

`NetworkManager`の`dispatcher`を利用します。それについての詳細は [NetworkManager dispatcher を使用したネットワークサービス](https://wiki.archlinux.jp/index.php/NetworkManager#NetworkManager_dispatcher_.E3.82.92.E4.BD.BF.E7.94.A8.E3.81.97.E3.81.9F.E3.83.8D.E3.83.83.E3.83.88.E3.83.AF.E3.83.BC.E3.82.AF.E3.82.B5.E3.83.BC.E3.83.93.E3.82.B9) を参照してください。


## コンタクト

質問・要望は、お気軽に [Issues](https://github.com/0xNOY/tmgw-wifi-authorizer/issues) へどうぞ。


## ライセンス

TMGW WiFi Authorizer は [Apache License, Version 2.0](https://github.com/0xNOY/tmgw-wifi-authorizer/blob/main/LICENSE) に基づいて公開されています。
