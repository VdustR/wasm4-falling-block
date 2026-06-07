#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std)]

mod game;

#[cfg(all(target_arch = "wasm32", not(test)))]
mod audio;
#[cfg(all(target_arch = "wasm32", not(test)))]
mod render;
#[cfg(all(target_arch = "wasm32", not(test)))]
mod wasm4;

#[cfg(all(target_arch = "wasm32", not(test)))]
use core::mem::MaybeUninit;
#[cfg(all(target_arch = "wasm32", not(test)))]
use core::ptr::addr_of_mut;

#[cfg(all(target_arch = "wasm32", not(test)))]
use game::{Game, SoundEvent};

#[cfg(all(target_arch = "wasm32", not(test)))]
const DISK_MAGIC: [u8; 4] = *b"SL01";
#[cfg(all(target_arch = "wasm32", not(test)))]
const DISK_SIZE: u32 = 8;

#[cfg(all(target_arch = "wasm32", not(test)))]
static mut GAME: MaybeUninit<Game> = MaybeUninit::uninit();
#[cfg(all(target_arch = "wasm32", not(test)))]
static mut GAME_READY: bool = false;

#[cfg(all(target_arch = "wasm32", not(test)))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(target_arch = "wasm32", not(test)))]
#[no_mangle]
fn start() {
    unsafe {
        (*wasm4::PALETTE)[0] = 0x0b0f09;
        (*wasm4::PALETTE)[1] = 0xbbe06b;
        (*wasm4::PALETTE)[2] = 0x6fa84f;
        (*wasm4::PALETTE)[3] = 0xe05d2e;
        *wasm4::SYSTEM_FLAGS = wasm4::SYSTEM_HIDE_GAMEPAD_OVERLAY;
        addr_of_mut!(GAME).write(MaybeUninit::new(Game::new(load_hi_score())));
        GAME_READY = true;
    }
}

#[cfg(all(target_arch = "wasm32", not(test)))]
#[no_mangle]
fn update() {
    unsafe {
        if !GAME_READY {
            start();
        }

        let game = &mut *(addr_of_mut!(GAME) as *mut Game);
        let input = *wasm4::GAMEPAD1;
        let result = game.tick(input);

        if game.take_save_request() {
            save_hi_score(game.hi_score());
        }

        render::draw(game);
        audio::play_music(game);
        if result.sound != SoundEvent::None {
            audio::play_sound(result.sound);
        }
    }
}

#[cfg(all(target_arch = "wasm32", not(test)))]
fn load_hi_score() -> u32 {
    let mut data = [0u8; DISK_SIZE as usize];
    let read = unsafe { wasm4::diskr(data.as_mut_ptr(), DISK_SIZE) };

    if read == DISK_SIZE && data[0..4] == DISK_MAGIC {
        u32::from_le_bytes([data[4], data[5], data[6], data[7]])
    } else {
        0
    }
}

#[cfg(all(target_arch = "wasm32", not(test)))]
fn save_hi_score(score: u32) {
    let mut data = [0u8; DISK_SIZE as usize];
    data[0..4].copy_from_slice(&DISK_MAGIC);
    data[4..8].copy_from_slice(&score.to_le_bytes());
    unsafe {
        wasm4::diskw(data.as_ptr(), DISK_SIZE);
    }
}
