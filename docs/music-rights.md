# Music Rights Notes

This is an implementation note, not legal advice.

## What was checked

- The well-known handheld block-puzzle A-Type melody is associated with `Korobeiniki`, a Russian folk tune.
- The specific 1989 handheld game arrangement, sound programming, and recordings are separate protected works from the underlying folk tune.
- Tetris Holding has a registered and renewed U.S. sound mark for an electronic sine-wave tune based on `Korobeiniki` in computer and video game software, hand-held electronic game devices, and online games.

## Decision

Falling Block does not use `Korobeiniki`, the Game Boy A-Type arrangement, any Nintendo sound recording, or the registered sound-mark note sequence.

The in-game loop, `Falling Olive Study`, is original. It uses an eight-bar D minor phrase with a short motive, a response, a contrasting departure, and a closing descent. The arrangement is intentionally sparse: pulse lead, quiet off-beat pulse arpeggio, and triangle bass roots. It avoids the `Korobeiniki` opening contour, the Game Boy A-Type phrase cadence, and the registered sound-mark note sequence. It is implemented directly through WASM-4 `tone()` calls in `src/audio.rs`.

## Sound Effects

All sound effects are original procedural tones:

- Start: short pulse confirmation.
- Move: low pulse tick.
- Rotate: right-panned pulse chirp.
- Soft drop: quiet triangle pulse.
- Lock: short noise hit.
- Line clear: pulse and triangle flourish based on the number of lines cleared.
- Game over: descending triangle plus noise tail.
