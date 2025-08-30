# Zed × MATLAB LSP セットアップ手順（再現性メモ）

このリポジトリの拡張で、Zed 上で MATLAB Language Server（MLS）を動かす最小構成の手順です。将来自分が忘れたときの備忘も兼ねています。

## 前提
- MATLAB: R2021b 以降（例: R2025a）
- MATLAB-language-server: 手元でビルド済み（`out/index.js` が存在）
- Node.js: 動作する 1 つ（例: `/opt/homebrew/opt/node@22/bin/node`）
- Zed: 0.201+（参考）

## 最短セットアップ（推奨）
- Node を絶対パスで指定し、PATH は最小限

settings.json 抜粋:
```
"lsp": {
  "matlab-language-server": {
    "binary": {
      "path": "/opt/homebrew/opt/node@22/bin/node",
      "arguments": [
        "/Users/sugu/personal/tools/MATLAB-language-server/out/index.js",
        "--stdio",
        "--matlabInstallPath", "/Applications/MATLAB_R2025a.app",
        "--matlabConnectionTiming", "onStart"
      ],
      "env": {
        "PATH": "/Applications/MATLAB_R2025a.app/bin:/usr/bin:/bin:/usr/sbin:/sbin"
      }
    }
  }
}
```
ポイント:
- `--matlabInstallPath` はオプション名と値を“別要素”で書く。
- `binary.env.PATH` は“ディレクトリの列”。`…/bin/matlab`（ファイル）を入れない。

## 代替（node をコマンド名で）
```
"binary": {
  "path": "node",
  "arguments": ["…/out/index.js", "--stdio", "--matlabInstallPath", "/Applications/MATLAB_R2025a.app"],
  "env": { "PATH": "/Applications/MATLAB_R2025a.app/bin:/opt/homebrew/opt/node@22/bin:/usr/bin:/bin:/usr/sbin:/sbin" }
}
```

## 検証手順
- Zed で .m を開く → Extension Logs に以下を確認
  - Binary path（node）と arguments に `--matlabInstallPath` がある
  - `matlabls: Launching MATLAB...` が出て、エラーなしで初期化
- ターミナル検証（任意）:
```
/Applications/MATLAB_R2025a.app/bin/matlab -batch "disp(version); exit"
node /Users/sugu/personal/tools/MATLAB-language-server/out/index.js --help
```

## よくあるエラーと対処
- spawn matlab ENOENT: PATH に `/Applications/…/bin` が無い → PATH 前置 or `--matlabInstallPath` 明示
- MATLAB 即終了: PATH を最小化しすぎ → `/usr/bin:/bin:/usr/sbin:/sbin` を追加、必要なら `--matlabConnectionTiming onStart`
- 引数解釈されない: `"--matlabInstallPath /path"` を 1 要素で書いている → 要素を分割

## 再現性メモ
- いま動作確認した組み合わせ:
  - MATLAB: R2025a
  - Node: `/opt/homebrew/opt/node@22/bin/node`
  - MLS: `/Users/sugu/personal/tools/MATLAB-language-server/out/index.js`
  - Zed: 0.201 系

## 拡張の更新手順（WASM 再生成）
```
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
cp target/wasm32-wasip1/release/*.wasm extension.wasm
```
- `extension.wasm` はコミット運用（即試せるため）。ソース変更時は必ず更新。

## オプション
- ラッパースクリプト方式（例: `/usr/local/bin/matlabls`）にし、Zed 側は `path: "matlabls"` にすると設定が短くなる。
