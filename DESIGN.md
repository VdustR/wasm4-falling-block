# Design

## Color Palette

- `--bg`: `oklch(0.090 0.000 0)`
- `--surface`: `oklch(0.165 0.030 110)`
- `--panel`: `oklch(0.230 0.050 110)`
- `--ink`: `oklch(0.935 0.012 110)`
- `--muted`: `oklch(0.735 0.035 110)`
- `--primary`: `oklch(0.430 0.090 110)`
- `--screen`: `oklch(0.810 0.130 135)`
- `--accent`: `oklch(0.620 0.170 32)`
- `--danger`: `oklch(0.560 0.180 28)`

## Typography

Use `system-ui` for the PWA shell and the WASM-4 built-in 8px bitmap font inside the cartridge. Shell text uses compact product sizing, not hero-scale typography.

## Layout

The page is a full-height app shell. A compact command/score rail sits beside or above the 160x160 WASM-4 viewport depending on available width. No landing-page hero or nested cards.

## Components

- Game viewport: square, pixel-rendered, framed by restrained product chrome.
- Blocks: each tetromino receives a random single-color visual style independent of piece kind and gameplay state. The four style variants are solid, cutout, inner-frame, and dither. Locked pieces render as connected shapes with outer contours, not isolated cells. Cleared rows recalculate contours as cells disappear.
- Hi score: prominent numeric readout on the cartridge title screen. The web shell avoids duplicate score or mode cards.
- Title demo: before player start, the cartridge runs low-key CPU demo play in the viewport while keeping the real game unstarted. The title overlay stays on the board, showing hi score and an always-visible blinking `PRESS X/Z START` prompt.
- Game-over overlay: restart instructions must stay inside a black panel so they never overlap unreadably with the board behind them.
- Start button: filled accent action, keyboard and pointer accessible.
- Update prompt: small fixed toast with a direct reload action.
- Footer: compact MIT License, 2026 VdustR (ViPro), and GitHub link.
- Rights note: short, explicit, and linked to `docs/music-rights.md` in the repository.

## Motion

Only small state transitions are used in the web shell. The game animation is rendered by WASM-4 at 60 Hz. Line clears pause input briefly, wipe from the center, then collapse the board. Reduced motion disables shell transitions.

## Audio

Music sits below the sound effects in the mix. Move, rotate, lock, line-clear, and game-over cues should remain easier to hear than the background loop.
