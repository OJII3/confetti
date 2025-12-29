# Confetti

画面全体に紙吹雪を飛ばす Linux Desktop コマンド。

## 実装

### 1. GNOME Shell Extension (推奨)

GNOME Wayland で確実に動作する。

#### Nixでのインストール

```bash
# ビルド
nix build .#gnome-extension

# Extensionのインストール
mkdir -p ~/.local/share/gnome-shell/extensions/
cp -r result/share/gnome-shell/extensions/confetti@ojii3.github.com ~/.local/share/gnome-shell/extensions/

# ログアウト/ログインしてから有効化
gnome-extensions enable confetti@ojii3.github.com

# CLIトリガーのインストール (オプション)
cp result/bin/confetti ~/.local/bin/confetti

# 紙吹雪を発射
confetti
```

#### 手動インストール

```bash
cd gnome-extension

# インストール
./install.sh

# ログアウト/ログインしてから有効化
gnome-extensions enable confetti@ojii3.github.com

# 紙吹雪を発射
./confetti
```

D-Bus経由でトリガー: `com.github.ojii3.Confetti.Fire()`

### 2. Rust + GTK4 版 (実験的)

gtk4-layer-shell を使用。GNOMEでは透過が機能しない可能性あり。

```bash
# Nixでビルド
nix build .#rust && ./result/bin/confetti

# または開発環境で実行
direnv allow
cargo run
```

## Nixパッケージ

flake.nixで以下のパッケージが提供されます：

- `packages.default` - GNOME Shell Extension (推奨)
- `packages.gnome-extension` - GNOME Shell Extension
- `packages.rust` - Rust + GTK4 版

```bash
# デフォルト（GNOME Extension）
nix build

# 特定のパッケージをビルド
nix build .#gnome-extension
nix build .#rust

# 利用可能なパッケージを確認
nix flake show
```

## 構造

```
.
├── gnome-extension/     # GNOME Shell Extension
│   ├── extension.js
│   ├── metadata.json
│   ├── confetti         # CLIトリガー
│   └── install.sh
├── src/main.rs          # Rust版
├── flake.nix           # Nixビルド設定
└── Cargo.toml
```
