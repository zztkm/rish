# rish

interactive shell written in rust

## Features

- basic shell 🤔
- Run WASI
- Run editor (vim, emacs, nano, etc...)

## 仕様

### 起動時

以下の優先順位で設定ファイルを探して読み込む
1. 引数に指定されたファイル
2. シェルを起動したカレントディレクトリの `.rishrc.toml`
3. ホームディレクトリの `.rishrc.toml`

### Wasi 実行

参考
- https://github.com/wasmerio/wasmer/blob/master/examples/wasi.rs
