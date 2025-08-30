# Repository Guidelines

## Project Structure & Module Organization
- `extension.toml`: Zed extension manifest (declares the MATLAB language server).
- `extension.wasm`: Built Rust→WASI artifact loaded by Zed.
- `src/`: Rust source for the extension (entry at `src/lib.rs`).
- `languages/matlab/config.toml`: Language config (comments, brackets, file suffixes).
- `Cargo.toml`, `rust-toolchain.toml`: Build configuration (Rust 2021, `wasm32-wasip1`).
- `simple.m`, `test.m`: Sample MATLAB files for manual checks.

## Build, Test, and Development Commands
- `rustup target add wasm32-wasip1`: Ensure the WASI target is available.
- `cargo build --target wasm32-wasip1 --release`: Build the extension.
- `cp target/wasm32-wasip1/release/*.wasm extension.wasm`: Update the artifact Zed loads.
- `cargo fmt && cargo clippy --no-deps`: Format and lint for basic hygiene.
- Run locally: Open this folder in Zed; Zed loads `extension.wasm` automatically.

## Coding Style & Naming Conventions
- Rust 2021; prefer `cargo fmt` defaults, 4‑space indentation.
- Use snake_case for files/functions, UpperCamelCase for types.
- Keep functions small; favor explicit returns and early error handling (`Result<_, String>` here).
- TOML: 2 spaces, trailing commas avoided.

## Testing Guidelines
- No automated tests; validate behavior manually:
  - Open `simple.m` in Zed; verify hover, completion, and diagnostics.
  - Check Zed’s Extension Logs for server startup lines.
- Useful checks: Server path detection, MATLAB install detection, and env overrides (below).

## Commit & Pull Request Guidelines
- Commits: Imperative, concise subject (50–72 chars). Example: `Detect MATLAB on Linux via /usr/local`.
- PRs: Clear description, rationale, test notes (files opened, expected LSP behavior), and screenshots/log snippets when relevant. Link issues if any.

## Security & Configuration Tips
- The server can be guided via env vars:
  - `MATLAB_LSP_SERVER=/path/to/out/index.js`
  - `MATLAB_LSP_NODE=/path/to/node` (else `node` on PATH)
  - `MATLAB_LSP_INSTALL_PATH` or `MATLAB_INSTALL_PATH=/path/to/MATLAB`
  - `MATLAB_LSP_USE_NPX=1` to run via `npx matlab-language-server --stdio`
- Example (macOS): `export MATLAB_INSTALL_PATH="/Applications/MATLAB_R2024a.app"`
