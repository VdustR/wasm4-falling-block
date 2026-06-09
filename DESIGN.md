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

The page is a full-height app shell. A compact command panel sits beside or above the 160x160 WASM-4 viewport depending on available width. No landing-page hero or nested cards. On small screens, Start and Focus enter a play view that hides the web shell chrome, places the game viewport in the upper play area, and reserves a bottom control dock for touch input.

## Components

- Brand system: the primary mark is the selected `Slanted Drop Cart` direction from `docs/logo-showcase.html`: a pure tilted retro cartridge icon with a lime body, black play window, and orange active block. It intentionally has no outer tile background, while reusing the Falling Block palette so the app icon, main visual, and cartridge feel like one system.
- Game viewport: square, pixel-rendered, framed by restrained product chrome.
- Blocks: each tetromino receives a random single-color visual style independent of piece kind and gameplay state. The readable variants are solid, inner-frame, and dither; the hollow cutout variant is intentionally removed. Cell fills do not draw per-cell borders; blocks render as connected shapes with black/background-colored outer contours, and cleared rows recalculate contours as cells disappear.
- Hi score: prominent numeric readout on the cartridge title screen. The web shell avoids duplicate score or mode cards.
- Title demo: before player start, the cartridge runs low-key CPU demo play in the viewport while keeping the real game unstarted. The title overlay stays on the board, showing hi score and an always-visible blinking `PRESS X/Z START` prompt.
- Game-over overlay: restart instructions must stay inside a black panel so they never overlap unreadably with the board behind them.
- Start button: filled accent action, keyboard and pointer accessible.
- Mobile fullscreen: installed PWA launch requests fullscreen display mode. Browser play view uses the Fullscreen API when available and still works as a chrome-free mobile layout when the API is unavailable.
- Mobile controls: the WASM-4 runtime's built-in virtual gamepad is an overlay, so the PWA shell hides it on narrow viewports and provides a bottom control dock that dispatches the same keyboard inputs. This keeps the board readable while preserving standard WASM-4 cartridge input.
- Game feel: horizontal movement uses deterministic DAS/ARR instead of frame-modulo repeats, line-clear transitions buffer recent rotate/shift input, grounded pieces use a short lock delay, and Arrow Up hard-drops. These choices keep the GB-inspired feel but remove avoidable missed-input frustration on keyboard and mobile.
- Update prompts move to the top edge during play view to avoid competing with thumb controls.
- Update prompt: small fixed toast with a direct reload action.
- Footer: compact MIT License, 2026 VdustR (ViPro), and GitHub link.
- Rights note: short, explicit, and linked to `docs/music-rights.md` in the repository.

## Motion

Only small state transitions are used in the web shell. The game animation is rendered by WASM-4 at 60 Hz. Line clears pause input briefly, wipe from the center, then collapse the board. Reduced motion disables shell transitions.

## Audio

Music sits below the sound effects in the mix, but both should be audible on phone speakers. Move, rotate, lock, line-clear, and game-over cues should remain easier to hear than the background loop.
