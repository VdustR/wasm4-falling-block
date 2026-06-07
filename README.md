# Stackline

Stackline is an original falling-block puzzle game for the [WASM-4](https://wasm4.org) fantasy console. It is inspired by compact handheld puzzlers and uses original code, visuals, music, and sound effects.

Repository metadata targets `vdustr/wasm4-tetris`.

## Features

- WASM-4 cartridge written in Rust.
- Title screen with persistent hi score and heuristic CPU demo play before the player starts.
- Classic 10x20 falling-block board, next preview, score, lines, and level.
- Original chiptune loop and sound effects using WASM-4 `tone()`.
- PWA shell with offline caching and an update prompt.
- MIT license, copyright 2026 VdustR (ViPro).

## Controls

- Arrow Left / Arrow Right: move.
- Arrow Down: soft drop.
- X, Space, Arrow Up: rotate clockwise or start.
- Z: rotate counterclockwise or start.
- Enter: WASM-4 runtime menu.

## Build

Install the WASM-4 CLI through your preferred Node package manager, or set `W4_CLI` to a local CLI path.

```sh
pnpm install
pnpm build
```

Without local dependencies:

```sh
cargo build --release
W4_CLI=/path/to/w4 node scripts/build.mjs
```

The PWA bundle is written to `dist/index.html`.

## Test

```sh
cargo test --target aarch64-apple-darwin
pnpm smoke
```

Use the matching host target for non-macOS systems.

## Music Rights

The project uses an original composition named "Falling Olive Study" and original sound effects. See [docs/music-rights.md](docs/music-rights.md) for the reasoning.
