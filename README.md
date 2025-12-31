# Monitor Switcher

DDC/CI経由でモニターの入力を切り替えるコマンドラインツール

## Setup

### Installation

```powershell
cargo build --release
```

### Create Config.file

`config.toml`ファイルを作成し、各モニターの識別値を設定：

```toml
[monitors]
main_maximum = 5
secondary_maximum = 17
```

メインモニターとセカンダリモニタの識別を `Input Select` の最大値で判別しています。（暫定）

## Usage

```powershell
# メインモニタを HDMI-1 に切り替え
monitor-switcher -m main -v 17

# セカンダリモニタを HDMI-2 に切り替え
monitor-switcher -m secondary -v 18

# セカンダリモニタを DisplayPort に切り替え
monitor-switcher -m secondary -v 15
```

## Option

- `-m, --monitor <TYPE>`: モニター種類（`main` または `secondary`）
- `-v, --value <VALUE>`: 入力値

## Environment

- Windows
- DDC/CI対応モニター

## Liscense

MIT
