use crate::game::{Game, Mode, SoundEvent};
use crate::wasm4::*;

const REST: u8 = 0;
const STEP_FRAMES: u32 = 12;

#[rustfmt::skip]
const LEAD: [u8; 64] = [
    62, REST, 65, 69, 67, 65, 64, REST,
    62, REST, 65, 70, 69, REST, 65, 64,
    67, REST, 69, 72, 70, 69, 67, REST,
    64, REST, 67, 70, 69, 67, 64, REST,
    62, 65, 69, 72, 69, 65, 62, REST,
    58, 62, 65, 69, 67, 65, 62, REST,
    60, 64, 67, 70, 69, 67, 64, REST,
    57, 61, 64, 67, 65, 64, 62, REST,
];

const BASS: [u8; 16] = [
    38, 38, 34, 34, 36, 36, 33, 33, 38, 38, 34, 34, 36, 36, 33, 38,
];

#[rustfmt::skip]
const ARP: [u8; 32] = [
    REST, 69, REST, 65, REST, 70, REST, 65,
    REST, 72, REST, 67, REST, 69, REST, 64,
    REST, 69, REST, 65, REST, 70, REST, 65,
    REST, 72, REST, 67, REST, 64, REST, 61,
];

pub fn play_music(game: &Game) {
    if game.mode() != Mode::Playing {
        return;
    }

    let frame = game.frame();
    let step = (frame / STEP_FRAMES) as usize;

    if frame % STEP_FRAMES == 0 {
        let note = LEAD[step % LEAD.len()];
        if note != REST {
            note_on(note, 4, 13, TONE_PULSE1 | TONE_MODE2);
        }
    }

    if frame % (STEP_FRAMES * 2) == STEP_FRAMES {
        let note = ARP[(step / 2) % ARP.len()];
        if note != REST {
            note_on(note, 2, 7, TONE_PULSE2 | TONE_MODE1 | TONE_PAN_RIGHT);
        }
    }

    if frame % (STEP_FRAMES * 4) == 0 {
        let note = BASS[(step / 4) % BASS.len()];
        note_on(note, 20, 10, TONE_TRIANGLE | TONE_PAN_LEFT);
    }
}

pub fn play_sound(event: SoundEvent) {
    match event {
        SoundEvent::None => {}
        SoundEvent::Start => note_on(62, 7, 58, TONE_PULSE1 | TONE_MODE3),
        SoundEvent::Move => note_on(47, 2, 24, TONE_PULSE2 | TONE_MODE1 | TONE_PAN_LEFT),
        SoundEvent::Rotate => note_on(67, 4, 38, TONE_PULSE2 | TONE_MODE2 | TONE_PAN_RIGHT),
        SoundEvent::SoftDrop => note_on(38, 2, 20, TONE_TRIANGLE),
        SoundEvent::Lock => noise(4, 26),
        SoundEvent::Line(lines) => {
            note_on(72 + lines * 2, 12, 68, TONE_PULSE1 | TONE_MODE3);
            note_on(50 + lines, 14, 44, TONE_TRIANGLE | TONE_PAN_LEFT);
        }
        SoundEvent::GameOver => {
            slide(50, 38, 42, TONE_TRIANGLE);
            noise(16, 46);
        }
    }
}

fn note_on(note: u8, sustain: u8, volume: u8, flags: u32) {
    tone(
        note as u32,
        envelope(1, 2, sustain, 5),
        volume as u32 | ((volume as u32 + 12).min(100) << 8),
        flags | TONE_NOTE_MODE,
    );
}

fn noise(sustain: u8, volume: u8) {
    tone(
        120,
        envelope(0, 1, sustain, 4),
        volume as u32 | ((volume as u32 + 24).min(100) << 8),
        TONE_NOISE | TONE_MODE4,
    );
}

fn slide(from: u16, to: u16, volume: u8, flags: u32) {
    tone(
        from as u32 | ((to as u32) << 16),
        envelope(0, 4, 22, 10),
        volume as u32 | ((volume as u32 + 12).min(100) << 8),
        flags | TONE_NOTE_MODE,
    );
}

fn envelope(attack: u8, decay: u8, sustain: u8, release: u8) -> u32 {
    sustain as u32 | ((release as u32) << 8) | ((decay as u32) << 16) | ((attack as u32) << 24)
}
