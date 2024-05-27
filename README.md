# PiSearch_Shuttle
Rustで簡単なWebアプリを作成して、Suttleで公開する。
## このプロジェクトの概要
* Rust言語を学んで「完全に理解した」まで到達する
* Rust with Rocket でWebアプリを作ってみる。テーマ：「円周率の検索」
* SuttleでWebアプリを公開する
## 必要な知識
### Rust（プログラミング言語）
### Rocket（RustのWEBフレームワーク）
### Suttle（Rustのホスティング）
## 手順
### Rust環境構築
* [RUST Install](https://www.rust-lang.org/ja/learn/get-started)
* Rustのいろいろ管理ツール「Cargo」が入る
  ```
  $>cargo --version
  cargo 1.78.0 (54d8815d0 2024-03-26)
  ```
  * プロジェクトのビルドにはcargo build
  * プロジェクトの実行にはcargo run
  * プロジェクトのテストにはcargo test
  * プロジェクトのドキュメントのビルドにはcargo doc
### Suttle基本知識習得
* [Suttle](https://www.shuttle.rs/)
```
What is Shuttle?
Shuttle is a Rust-native cloud development platform that lets you deploy your app while also taking care of all of your infrastructure.

[意訳]※現時点での私の理解で
Shuttleとは何ですか？
Rustアプリのデプロイを受け付けて、アプリ実行インフラとを提供するクラウド開発プラットフォーム。アプリ実行に必要なDBなども準備できる。
```

### Suttleスケルトン（Rustプロジェクト）作成
* Steps
  * アカウント作成
    * [Shuttle Login](https://console.shuttle.rs/login)
    * GitHubアカウント連携なので「Login with GitHub」押すだけ
  * トップ画面
    * API Key が自動作成されてマスキング表示されている。（いつでも再参照可）
  * cargo-shuttleクライアントツール(CLI)をローカルにインストール
    ```
    $> cargo install cargo-shuttle
    ```
    * めっちゃいろいろ入る。400個以上、、、5分くらいかかる。
    ```
    $ >cargo shuttle --version
    cargo-shuttle 0.45.0
    ```
  * cargo-shuttleで、Suttleアカウントログイン
    ```
    $ > cargo shuttle login
    ```
    * Webブラウザが開いて、ブラウザがログイン状態になる
    * CLI画面は、「API Key」を求めるので、WEB画面のそれをコピーしてセット

  * cargo-shuttleで、プロジェクトスケルトンを作成
    ```
    C:\dev\ > cargo shuttle init
    What do you want to name your project?
    It will be hosted at ${project_name}.shuttleapp.rs, so choose something unique!
    ✔ Project name · pisearch

    Where should we create this project?
    ✔ Directory · C:\dev\pisearch

    What type of project template would you like to start from?
    ❯ A Hello World app in a supported framework
      Browse our full library of templates

    ✔ Select template · Rocket - Simple and easy-to-use web framework

    Creating project "pisearch" in "C:\dev\pisearch"

    ✔ Claim the project name "pisearch" by starting a project container on Shuttle? · yes

    Project "pisearch" is ready
    Your project will sleep if it is idle for 30 minutes.
    To change the idle time refer to the docs: https://docs.shuttle.rs/getting-started/idle-projects

    Run `cargo shuttle deploy --allow-dirty` to deploy your Shuttle service.
    You can `cd` to the directory, then:
    Run `cargo shuttle run` to run the app locally.
    ```
    * 出来上がり。Git管理下であり、.gitignoreも作ってくれている。
    ```
    /project
    │  .gitignore
    │  Cargo.lock
    │  Cargo.toml
    │
    ├─.git
    │    ***
    └─src
          main.rs
    ```
  * ローカル実行
    ```
    $ > cargo shuttle run
      Building C:\dev\pisearch
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
      [Runtime] shuttle-runtime 0.45.0 executable started
      [Runtime] Shuttle's default tracing subscriber is initialized!
      [Runtime] To disable it and use your own, check the docs: https://docs.shuttle.rs/configuration/logs
      No resources are linked to this service

      St arting pisearch on http://127.0.0.1:8000
    ```
  * ローカルアクセス
    * ブラウザで http://127.0.0.1:8000
    * ”Hello, world!” が表示される
  * ローカル停止
    * Ctrl + C
### Suttleスケルトンデプロイ
* Steps
  * git commit
    * デプロイする前にcommitしていないとダメみたい
    ```
    C:\dev\pisearch>cargo shuttle deploy
    Error: 4 files in the working directory contain changes that were not yet committed into git:
    .gitignore
    Cargo.lock
    Cargo.toml
    src/

    To proceed despite this and include the uncommitted changes, pass the `--allow-dirty` or `--ad` flag
    ```
  * デプロイ
    ```
    $ > cargo shuttle deploy
      ```
  * Shuttle 管理画面でデプロイ済みを確認！
  * アプリにアクセスしてHelloWorldを確認！
  * おわり！
### Let'sプログラミング
### Suttleデプロイ
### TEST
### リリース！

