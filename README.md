# Confetti 🎉

Linux デスクトップで画面全体に紙吹雪を飛ばすコマンドです。

## 特徴

- GNOME Shell Extension による確実な動作 (Wayland対応)
- D-Bus 経由でのトリガー
- シンプルなCLIインターフェース
- Nix Flakes によるビルドとパッケージング

## インストール方法

### 1. GNOME Shell Extension (推奨)

GNOME Wayland で確実に動作します。

#### Nix を使用する場合

```bash
# ビルド
nix build .#gnome-extension

# Extension のインストール
mkdir -p ~/.local/share/gnome-shell/extensions/
cp -r result/share/gnome-shell/extensions/confetti@ojii3.github.com ~/.local/share/gnome-shell/extensions/

# ログアウト/ログインしてから Extension を有効化
gnome-extensions enable confetti@ojii3.github.com

# CLI トリガーのインストール (オプション)
cp result/bin/confetti ~/.local/bin/confetti
```

#### 手動インストール

```bash
cd gnome-extension

# インストールスクリプトを実行
./install.sh

# ログアウト/ログインしてから Extension を有効化
gnome-extensions enable confetti@ojii3.github.com
```

### 2. Rust + GTK4 版 (実験的)

`gtk4-layer-shell` を使用した実験的な実装です。GNOME では透過が機能しない可能性があります。

```bash
# Nix でビルド
nix build .#rust && ./result/bin/confetti

# または開発環境で実行
direnv allow
cargo run
```

## 使い方

### CLI から実行

```bash
confetti
```

### D-Bus 経由で実行

```bash
gdbus call --session \
  --dest org.gnome.Shell \
  --object-path /org/gnome/Shell/Extensions/Confetti \
  --method com.github.ojii3.Confetti.Fire
```

## Nix パッケージ

このプロジェクトは Nix Flakes で以下のパッケージを提供します:

- `packages.default` - GNOME Shell Extension (推奨)
- `packages.gnome-extension` - GNOME Shell Extension
- `packages.rust` - Rust + GTK4 版

```bash
# デフォルト (GNOME Extension)
nix build

# 特定のパッケージをビルド
nix build .#gnome-extension
nix build .#rust

# 利用可能なパッケージを確認
nix flake show
```

## 開発

```bash
# 開発環境に入る
direnv allow

# Rust 版を実行
cargo run

# Rust 版をビルド
cargo build --release
```

## プロジェクト構造

```
.
├── gnome-extension/     # GNOME Shell Extension
│   ├── extension.js     # Extension 本体
│   ├── metadata.json    # Extension メタデータ
│   ├── confetti         # CLI トリガースクリプト
│   └── install.sh       # インストールスクリプト
├── src/                 # Rust 版ソースコード
│   └── main.rs
├── flake.nix           # Nix ビルド設定
├── Cargo.toml          # Rust プロジェクト設定
└── README.md           # このファイル
```

## 動作環境

- **GNOME Extension 版**: GNOME Shell 45-49
- **Rust 版**: GTK4, gtk4-layer-shell

## ライセンス

MIT License

## リポジトリ

https://github.com/ojii3/confetti
