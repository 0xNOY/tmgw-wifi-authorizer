# TMGW WiFi Autholizer

## 概要

大学のWiFi接続時に外部ネットワークへアクセスするには認証が必要です。通常、ブラウザを介して認証を行いますが、このソフトウェアを用いて、ブラウザ無しで認証を行えます。WiFi接続時のイベント登録等すれば自動で認証を行うこともできます。自動化の詳細は [Windowsで認証を自動化する](###Windowsで認証を自動化する)


## 使い方

1. [リリースページ](https://github.com/0xNOY/tmgw-wifi-autholizer/releases)から実行ファイルをダウンロードする。

1. 環境変数 `TMGW_ID` にMyPCアカウントのIDを、環境変数 `TMGW_PASSWORD` にMyPCアカウントのパスワードを設定する。

1. ダウンロードした実行ファイルを実行する。

> **Note**  
このソフトウェアは、処理が完了するとすぐに終了します。出力を確認するには、ターミナルから起動するといいです。


### Windowsで認証を自動化する

Windowsでは、タスクスケジューラを用いてWiFi接続時に自動で認証を行えます。


#### 手順

1. ダウンロードした実行可能ファイルのショートカットを作成する。

1. 作成したショートカットファイルを右クリックし `プロパティ` をクリックする。

1. `実行時の大きさ` を `最小化` に変更し `OK` をクリック。

1. タスクスケジューラを起動する。以下の場所にあります。  
    `コントロールパネル` > `システムとセキュリティ` > `管理ツール` > `タスク スケジューラ`

1. メニューバーの `操作` > `タスクの作成` をクリックする。

1. 名前を分かり易いものに変更する。  
    例) `TMGW WiFi Autholizer`

1. 同ウィンドウのタブ `トリガー` > `新規` をクリックする。

1. `タスクの開始` を `イベント時` に変更する。

1. `ログ` を `Microsoft-Windows-NetworkProfile/Operational` に変更する。  
    `ソース` に `NetworkProfile` と入力する。  
    `イベント ID` に `10000` と入力する。

1. `遅延時間を指定する` にチェックを入れ、`1 秒間` と入力する。  
    `繰り返し間隔を指定する` にチェックを入れ、`30 分間`と入力する。  
    `継続時間` に `8 時間` と入力し `OK` をクリックする。

1. タブ `操作` > `新規` をクリックする。

1. `参照` をクリックし、`1.` で作成したショートカットファイルを選択し `OK` をクリックする。

1. タブ `条件` > `コンピュータをAC電源で...` のチェックを外し `OK` をクリックする。


## ライセンス

TMGW WiFi Autholizer は [Apache License, Version 2.0](https://github.com/0xNOY/tmgw-wifi-autholizer/blob/main/LICENSE) に基づいて公開されています。