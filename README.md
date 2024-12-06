# Flutter X Terraform (AWS) 開発者支援ツール

## Flutter

- 初期に必要なファイルの生成
- 設定ファイルからDDD構成のファイル群を生成
- 画面遷移・頻出する画面(認証画面など)を生成
- UseCaseの生成
- Infrastructureは実装しない
    - API通信部分はswagger.yamlを記述して以下コマンドを実行する
        - `openapi-generator-cli generate -i swagger.yaml -g dart-dio -o openapi`

## Terraform

- 初期に必要なフォルダ構成・ファイル群を生成
- よくあるテンプレートファイルの生成
- sam との連携も視野に入れつつどのような構成にするかは要検討
    - Serverless構成以外は実装する予定なし

## Python (暫定)

- AWS Lambdaの実行のためのサンプルコード生成
    - 実行速度、型定義の不安定さから使ってられないため、言語は変える
    - 有力候補はRust
- API 通信部分はGraphQLのようなもので良い
    - それ以降の具体的な実装の自動生成は現実的ではなく、AI支援による実装が簡単にできる
    - また、自動生成では複雑化が進み、使用者が理解できない構造になる


## TODO

- flutter
    - `route_path`に変数を保持できるようにする
    - `fromRoutePath`のデフォルト初期構築を実装
    - shellのindexのタイプを切り替えられるように実装
    - `flutter pub add`を実行させる
    - `dart fix --apply`を実行させる
    - `flutter pub run import_sorter:main`を実行させる
    - `package:{my_app}`のimportを相対に変換する
    - slangなどでl10nのファイルをわかりやすくする
    - providerの自動生成の実装
    - RepositoryとServiceの自動生成実装
    - 既存のファイルをコピーしてくるような実装をしたい。特にwidgetとやl10n
