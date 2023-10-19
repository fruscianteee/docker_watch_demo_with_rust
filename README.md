# docker_watch_demo_with_rust

docker compose watchの動作確認を行う。
(さぶで、docker initでrustがサポートされたので合わせて確認)

## 使い方

1. compose watch実行
    ```
    docker compose watch
    ```
2. ブラウザやcurlなどで`localhost:8080`にアクセスする

### 動作確認
`/src`,`/templates`ディレクトリ配下のファイルを修正してみる

## その他
初回セットアップ

```
cargo init
docker init
```

dockerの設定内容

```
? What application platform does your project use? `Rust`

? What version of Rust do you want to use? `1.73.0`

? What port does your server listen on? `8080`
```
