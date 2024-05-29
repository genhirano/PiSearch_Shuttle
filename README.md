[公開中！] → https://pisearch.shuttleapp.rs

# PiSearch_Shuttle（円周率検索アプリ）
**プログラミング言語「Rust」を使って** 簡単なWebアプリ（簡単な円周率検索）を作成し、 **[Rustを完全に理解](https://note.com/teched/n/n555f4f5f9344)** する
## このプロジェクトの概要
* Rust言語を[「完全に理解した」](https://note.com/teched/n/n555f4f5f9344)まで到達する
* これを実践を通して達成するため、Rust with Rocket でWebアプリを作ってみる（テーマ：「円周率の検索」）
*  **作るだけではなく、** 公開する
## 対象
* Rust（プログラミング言語）
* Rocket /  rocket_dyn_templates （RustのWEBフレームワーク）
* Suttle（Rustアプリケーションのホスティング）
## 手順
### Rust環境構築/Hello World!
* [Rust Install](https://www.rust-lang.org/ja/learn/get-started)
* Rustのいろいろ管理ツール「Cargo」が入る
  ```
  $>cargo --version
  cargo 1.78.0 (54d8815d0 2024-03-26)
  ```
  * 虎の巻
    * プロジェクトのビルドにはcargo build
    * プロジェクトの実行にはcargo run
    * プロジェクトのテストにはcargo test
    * プロジェクトのドキュメントのビルドにはcargo doc
  * new して Runしてみる
    ```
    $ >cargo new myFirstRust
        Creating binary (application) `myFirstRust` package
      warning: the name `myFirstRust` is not snake_case or kebab-case which is recommended for package names, consider `myfirstrust`
      note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    $>cd myFirstRust

    $>cargo run
     Compiling myFirstRust v0.1.0 (C:\dev\myFirstRust)
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.01s
       Running `target\debug\myFirstRust.exe`
    Hello, world!
    ```
  * "Hello, Wold!" 表示されました！
  * このプログラムは、Cargo new の時にCargoが自動的に作っている。ソースコードはこちら
    * src/main.rs
    ```rust
    fn main() {
     println!("Hello, world!");
    }
    ```
### Suttle環境構築、スケルトンプロジェクト作成、そのデプロイ
#### Suttleとは？ →[Suttle](https://www.shuttle.rs/)
```
What is Shuttle?
Shuttle is a Rust-native cloud development platform that lets you deploy your app while also taking care of all of your infrastructure.

[意訳]※現時点での私の理解で
Shuttleとは何ですか？
Rustアプリのデプロイを受け付けて、アプリ実行インフラとを提供するクラウド開発プラットフォーム。アプリ実行に必要なDBなども準備できる。
```
#### Suttleスケルトン（Rustプロジェクト）作成
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
  * cargo-shuttleで、CLIクライアントによるSuttleアカウントログイン
    ```
    $ > cargo shuttle login
    ```
    * Webブラウザが開いて、ブラウザがログイン状態になる
    * CLI画面は、「API Key」を求めるので、WEB画面のそれをコピーしてセット

  * cargo-shuttleで、プロジェクトスケルトンを作成
    ```
    $ > cargo shuttle init
    What do you want to name your project?
    It will be hosted at ${project_name}.shuttleapp.rs, so choose something unique!
    ✔ Project name · pisearch

    Where should we create this project?
    ✔ Directory · ***\pisearch

    What type of project template would you like to start from?
    ❯ A Hello World app in a supported framework
      Browse our full library of templates

    ✔ Select template · Rocket - Simple and easy-to-use web framework

    Creating project "pisearch" in "***\pisearch"

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
      Building ***\pisearch
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
#### Suttleスケルトンデプロイ
* Steps
  * git commit
    * デプロイする前にcommitしていないとダメみたい
    ```
    $ >cargo shuttle deploy
    Error: 4 files in the working directory contain changes that were not yet committed into git:
    To proceed despite this and include the uncommitted changes, pass the `--allow-dirty` or `--ad` flag
    ```
  * デプロイ
    ```
    $ > cargo shuttle deploy
      ```
  * Shuttle 管理画面でデプロイ済みを確認！
  * アプリにアクセスしてHelloWorldを確認！ Ok!
    * [デプロイ先にアクセス！](https://pisearch.shuttleapp.rs)
  * おわり！
### Let'sプログラミング
* 設計
  * 全体概要
    * 円周率データ(複数のYCDファイル)から、WEB画面からの入力による検索ターゲットを検索して画面に表示する。
  * 構成
    * main
      * 概要：WEBアクセス処理、円周率検索
      * 機能：WEBから検索文字列を受け取り、ヒットするまでYCDが提供するブロック単位で検索し、ヒットした場所（円周率少数点以下桁数）をWEB画面に返す。
      * 画面：index.html（テンプレートエンジンを使って生成してクライアントへ帰す）
    * ycd(YCDクレート)
      * 概要：複数のYCDFileを順番に管理する
      * 機能：指定フォルダにある***.ycdファイルを、ファイル名順に複数管理する。最小のYCDFileから順次1000ブロック単位で読み、そのYCDFileの全てのブロックが読み終わったら次のYCDFileへシームレスに移行する。
      * その他：ブロック跨り、又はファイルまたがりの部分に検索文字列が跨る場合があるので工夫が必要。
    * ycdfile(YCDFileクレート)
      * 概要：YCDファイル１つに対応し、I/O管理する（ReadOnly）
      * 機能：YCDヘッダの取得と保持、円周率データの1ブロック(8バイトの19桁)毎に提供
* 実装
  * プログラミング言語：Rust、WEBフレームワーク：Rocket、TemplateEngine：rocket_dyn_templates
### 公開
#### Shuttleデプロイ
* $ >cargo shuttle deploy → 失敗！
* テストがfailするとDeployが失敗する。
  * デプロイ時にテストをスキップする方法
    ```
    $ > cargo shuttle deploy --no-test  
    ```
#### アクセスしてみる
* [デプロイ先にアクセス！](https://pisearch.shuttleapp.rs)
* 表示された！

---
# [Rustプログラミング、完全に理解しました！](https://note.com/teched/n/n555f4f5f9344)


---
#### 代表的な円周率検索サイト
* PiSearch
  * https://pisearch.joshkeegan.co.uk/
* Irrational Numbers Search Engine
  * https://www.subidiom.com/pi/
* The Pi-Search Page
  * https://www.angio.net/pi/
* Pi Search
  * https://katiesteckles.co.uk/pisearch/



