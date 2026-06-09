#![cfg_attr(test, allow(dead_code))]

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const BOARD_CELLS: usize = BOARD_WIDTH * BOARD_HEIGHT;
pub const LINE_CLEAR_FRAMES: u8 = 14;

pub const BTN_1: u8 = 1;
pub const BTN_2: u8 = 2;
pub const BTN_LEFT: u8 = 16;
pub const BTN_RIGHT: u8 = 32;
pub const BTN_UP: u8 = 64;
pub const BTN_DOWN: u8 = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Title,
    Playing,
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundEvent {
    None,
    Start,
    Move,
    Rotate,
    SoftDrop,
    Lock,
    Line(u8),
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TickResult {
    pub sound: SoundEvent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: u8,
    pub rot: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockPattern {
    Solid,
    InnerFrame,
    Dither,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockStyle {
    pub color: u8,
    pub pattern: BlockPattern,
}

#[derive(Clone, Copy)]
pub struct Game {
    board: [u8; BOARD_CELLS],
    board_ids: [u16; BOARD_CELLS],
    board_styles: [BlockStyle; BOARD_CELLS],
    current: Piece,
    next: Piece,
    current_style: BlockStyle,
    next_style: BlockStyle,
    x: i8,
    y: i8,
    demo_target_x: i8,
    demo_target_rot: u8,
    rng: u32,
    next_piece_id: u16,
    clear_mask: u32,
    clear_timer: u8,
    frame: u32,
    last_input: u8,
    left_hold_frames: u8,
    right_hold_frames: u8,
    lock_timer: u8,
    lock_resets: u8,
    buffered_input: u8,
    input_buffer_timer: u8,
    soft_drop_sound_timer: u8,
    score: u32,
    lines: u16,
    level: u8,
    hi_score: u32,
    mode: Mode,
    save_requested: bool,
}

const START_X: i8 = 3;
const START_Y: i8 = 0;
const DAS_FRAMES: u8 = 12;
const ARR_FRAMES: u8 = 4;
const LOCK_DELAY_FRAMES: u8 = 20;
const LOCK_RESET_LIMIT: u8 = 15;
const INPUT_BUFFER_FRAMES: u8 = 18;
const SOFT_DROP_FRAMES: u32 = 2;
const SOFT_DROP_SOUND_FRAMES: u8 = 5;
const BUFFERABLE_INPUT: u8 = BTN_LEFT | BTN_RIGHT | BTN_1 | BTN_2;
const EMPTY_STYLE: BlockStyle = BlockStyle {
    color: 0,
    pattern: BlockPattern::Solid,
};

#[rustfmt::skip]
const PIECES: [[[(i8, i8); 4]; 4]; 7] = [
    [
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(1, 0), (1, 1), (1, 2), (1, 3)],
    ],
    [
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
    ],
    [
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (1, 2)],
        [(1, 0), (0, 1), (1, 1), (1, 2)],
    ],
    [
        [(1, 0), (2, 0), (0, 1), (1, 1)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(1, 1), (2, 1), (0, 2), (1, 2)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
    ],
    [
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(2, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(1, 0), (0, 1), (1, 1), (0, 2)],
    ],
    [
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(1, 0), (1, 1), (0, 2), (1, 2)],
    ],
    [
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 1), (0, 2)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ],
];

impl Game {
    pub fn new(hi_score: u32) -> Self {
        let mut game = Self {
            board: [0; BOARD_CELLS],
            board_ids: [0; BOARD_CELLS],
            board_styles: [EMPTY_STYLE; BOARD_CELLS],
            current: Piece { kind: 0, rot: 0 },
            next: Piece { kind: 1, rot: 0 },
            current_style: EMPTY_STYLE,
            next_style: EMPTY_STYLE,
            x: START_X,
            y: START_Y,
            demo_target_x: START_X,
            demo_target_rot: 0,
            rng: 0x51_4c_30_31,
            next_piece_id: 1,
            clear_mask: 0,
            clear_timer: 0,
            frame: 0,
            last_input: 0,
            left_hold_frames: 0,
            right_hold_frames: 0,
            lock_timer: 0,
            lock_resets: 0,
            buffered_input: 0,
            input_buffer_timer: 0,
            soft_drop_sound_timer: 0,
            score: 0,
            lines: 0,
            level: 0,
            hi_score,
            mode: Mode::Title,
            save_requested: false,
        };
        game.current.kind = game.random_piece();
        game.current_style = game.random_style();
        game.next.kind = game.random_piece();
        game.next_style = game.random_style();
        game.set_demo_target();
        game
    }

    pub fn tick(&mut self, input: u8) -> TickResult {
        self.frame = self.frame.wrapping_add(1);
        let pressed = input & !self.last_input;
        let mut sound = SoundEvent::None;

        match self.mode {
            Mode::Title => {
                if pressed & (BTN_1 | BTN_2) != 0 {
                    self.start_game();
                    sound = SoundEvent::Start;
                } else {
                    self.update_cpu_demo();
                }
            }
            Mode::GameOver => {
                if pressed & (BTN_1 | BTN_2) != 0 {
                    self.start_game();
                    sound = SoundEvent::Start;
                }
            }
            Mode::Playing => {
                sound = self.update_playing(input, pressed);
            }
        }

        self.last_input = input;
        self.age_input_buffer();
        self.tick_soft_drop_sound_timer();
        TickResult { sound }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn lines(&self) -> u16 {
        self.lines
    }

    pub fn hi_score(&self) -> u32 {
        self.hi_score
    }

    pub fn frame(&self) -> u32 {
        self.frame
    }

    pub fn current(&self) -> Piece {
        self.current
    }

    pub fn next(&self) -> Piece {
        self.next
    }

    pub fn position(&self) -> (i8, i8) {
        (self.x, self.y)
    }

    pub fn cell(&self, x: usize, y: usize) -> u8 {
        self.board[y * BOARD_WIDTH + x]
    }

    pub fn cell_id(&self, x: usize, y: usize) -> u16 {
        self.board_ids[y * BOARD_WIDTH + x]
    }

    pub fn cell_style(&self, x: usize, y: usize) -> BlockStyle {
        self.board_styles[y * BOARD_WIDTH + x]
    }

    pub fn current_style(&self) -> BlockStyle {
        self.current_style
    }

    pub fn next_style(&self) -> BlockStyle {
        self.next_style
    }

    pub fn is_clearing(&self) -> bool {
        self.clear_timer > 0
    }

    pub fn clearing_row(&self, y: usize) -> bool {
        self.clear_mask & row_mask(y) != 0
    }

    pub fn clear_timer(&self) -> u8 {
        self.clear_timer
    }

    pub fn piece_cells(piece: Piece) -> [(i8, i8); 4] {
        PIECES[piece.kind as usize][piece.rot as usize]
    }

    pub fn take_save_request(&mut self) -> bool {
        let requested = self.save_requested;
        self.save_requested = false;
        requested
    }

    fn update_playing(&mut self, input: u8, pressed: u8) -> SoundEvent {
        if self.clear_timer > 0 {
            self.buffer_input(pressed);
            self.clear_timer -= 1;
            if self.clear_timer == 0 {
                self.apply_clear_mask(self.clear_mask);
                self.clear_mask = 0;
                self.spawn_next();
                if self.mode == Mode::GameOver {
                    return SoundEvent::GameOver;
                }
                self.apply_buffered_input();
            }
            return SoundEvent::None;
        }

        let mut sound = self.update_horizontal_movement(input, pressed);

        if pressed & BTN_1 != 0 {
            if self.try_rotate(1) {
                sound = SoundEvent::Rotate;
            }
        }

        if pressed & BTN_2 != 0 {
            if self.try_rotate(-1) {
                sound = SoundEvent::Rotate;
            }
        }

        if pressed & BTN_UP != 0 {
            return self.hard_drop();
        }

        if input & BTN_DOWN != 0 && self.frame % SOFT_DROP_FRAMES == 0 {
            if self.try_move(0, 1) {
                self.reset_lock_state_after_descent();
                self.score = self.score.saturating_add(1);
                self.touch_hi_score();
                if self.soft_drop_sound_timer == 0 {
                    self.soft_drop_sound_timer = SOFT_DROP_SOUND_FRAMES;
                    sound = SoundEvent::SoftDrop;
                }
            }
        }

        if self.frame % self.gravity_frames() == 0 {
            if self.try_move(0, 1) {
                self.reset_lock_state_after_descent();
            }
        }

        if let Some(lock_sound) = self.update_lock_delay() {
            lock_sound
        } else {
            sound
        }
    }

    fn update_cpu_demo(&mut self) {
        if self.clear_timer > 0 {
            self.clear_timer -= 1;
            if self.clear_timer == 0 {
                self.apply_clear_mask(self.clear_mask);
                self.clear_mask = 0;
                self.spawn_demo_next();
            }
            return;
        }

        if self.current.rot != self.demo_target_rot {
            if !self.try_rotate(1) {
                self.demo_target_rot = self.current.rot;
            }
            return;
        }

        if self.x < self.demo_target_x {
            if self.try_move(1, 0) {
                return;
            }
            self.demo_target_x = self.x;
        } else if self.x > self.demo_target_x {
            if self.try_move(-1, 0) {
                return;
            }
            self.demo_target_x = self.x;
        }

        if self.frame % 3 == 0 && !self.try_move(0, 1) {
            self.lock_demo_piece();
        }
    }

    fn start_game(&mut self) {
        self.board = [0; BOARD_CELLS];
        self.board_ids = [0; BOARD_CELLS];
        self.board_styles = [EMPTY_STYLE; BOARD_CELLS];
        self.next_piece_id = 1;
        self.clear_mask = 0;
        self.clear_timer = 0;
        self.score = 0;
        self.lines = 0;
        self.level = 0;
        self.current = Piece {
            kind: self.random_piece(),
            rot: 0,
        };
        self.current_style = self.random_style();
        self.next = Piece {
            kind: self.random_piece(),
            rot: 0,
        };
        self.next_style = self.random_style();
        self.x = START_X;
        self.y = START_Y;
        self.demo_target_x = START_X;
        self.demo_target_rot = 0;
        self.mode = Mode::Playing;
        self.last_input = 0;
        self.reset_timing_state();
    }

    fn try_move(&mut self, dx: i8, dy: i8) -> bool {
        let x = self.x + dx;
        let y = self.y + dy;
        if self.fits(self.current, x, y) {
            self.x = x;
            self.y = y;
            true
        } else {
            false
        }
    }

    fn try_rotate(&mut self, direction: i8) -> bool {
        let rot = ((self.current.rot as i8 + direction + 4) % 4) as u8;
        let rotated = Piece {
            kind: self.current.kind,
            rot,
        };
        let was_grounded = self.grounded();
        for (kick_x, kick_y) in [
            (0, 0),
            (-1, 0),
            (1, 0),
            (-2, 0),
            (2, 0),
            (0, -1),
            (-1, -1),
            (1, -1),
            (0, -2),
        ] {
            if self.fits(rotated, self.x + kick_x, self.y + kick_y) {
                self.current = rotated;
                self.x += kick_x;
                self.y += kick_y;
                self.reset_lock_delay_after_action(was_grounded);
                return true;
            }
        }
        false
    }

    fn lock_piece(&mut self) -> SoundEvent {
        self.reset_lock_state_after_descent();
        let piece_id = self.take_next_piece_id();
        for (cx, cy) in Self::piece_cells(self.current) {
            let bx = self.x + cx;
            let by = self.y + cy;
            if bx >= 0 && bx < BOARD_WIDTH as i8 && by >= 0 && by < BOARD_HEIGHT as i8 {
                let index = by as usize * BOARD_WIDTH + bx as usize;
                self.board[index] = self.current.kind + 1;
                self.board_ids[index] = piece_id;
                self.board_styles[index] = self.current_style;
            }
        }

        let mask = self.full_rows_mask();
        let cleared = count_rows(mask);
        if cleared > 0 {
            self.clear_mask = mask;
            self.clear_timer = LINE_CLEAR_FRAMES;
            self.add_line_score(cleared);
            SoundEvent::Line(cleared)
        } else {
            self.spawn_next();
            if self.mode == Mode::GameOver {
                SoundEvent::GameOver
            } else {
                SoundEvent::Lock
            }
        }
    }

    fn spawn_next(&mut self) {
        self.current = self.next;
        self.current.rot = 0;
        self.current_style = self.next_style;
        self.next = Piece {
            kind: self.random_piece(),
            rot: 0,
        };
        self.next_style = self.random_style();
        self.x = START_X;
        self.y = START_Y;
        self.reset_lock_state_after_descent();
        if !self.fits(self.current, self.x, self.y) {
            self.mode = Mode::GameOver;
            self.touch_hi_score();
        }
    }

    fn lock_demo_piece(&mut self) {
        let piece_id = self.take_next_piece_id();
        for (cx, cy) in Self::piece_cells(self.current) {
            let bx = self.x + cx;
            let by = self.y + cy;
            if bx >= 0 && bx < BOARD_WIDTH as i8 && by >= 0 && by < BOARD_HEIGHT as i8 {
                let index = by as usize * BOARD_WIDTH + bx as usize;
                self.board[index] = self.current.kind + 1;
                self.board_ids[index] = piece_id;
                self.board_styles[index] = self.current_style;
            }
        }

        let mask = self.full_rows_mask();
        if mask != 0 {
            self.clear_mask = mask;
            self.clear_timer = LINE_CLEAR_FRAMES;
        } else {
            self.spawn_demo_next();
        }
    }

    fn spawn_demo_next(&mut self) {
        self.current = self.next;
        self.current.rot = 0;
        self.current_style = self.next_style;
        self.next = Piece {
            kind: self.random_piece(),
            rot: 0,
        };
        self.next_style = self.random_style();
        self.x = START_X;
        self.y = START_Y;
        self.reset_lock_state_after_descent();
        self.set_demo_target();
        if !self.fits(self.current, self.x, self.y) {
            self.reset_cpu_demo();
        }
    }

    fn reset_cpu_demo(&mut self) {
        self.board = [0; BOARD_CELLS];
        self.board_ids = [0; BOARD_CELLS];
        self.board_styles = [EMPTY_STYLE; BOARD_CELLS];
        self.clear_mask = 0;
        self.clear_timer = 0;
        self.current = Piece {
            kind: self.random_piece(),
            rot: 0,
        };
        self.current_style = self.random_style();
        self.next = Piece {
            kind: self.random_piece(),
            rot: 0,
        };
        self.next_style = self.random_style();
        self.x = START_X;
        self.y = START_Y;
        self.reset_timing_state();
        self.set_demo_target();
    }

    fn update_horizontal_movement(&mut self, input: u8, pressed: u8) -> SoundEvent {
        let left = input & BTN_LEFT != 0;
        let right = input & BTN_RIGHT != 0;
        let was_overlapping = self.last_input & BTN_LEFT != 0 && self.last_input & BTN_RIGHT != 0;

        if left == right {
            self.left_hold_frames = 0;
            self.right_hold_frames = 0;
            return SoundEvent::None;
        }

        if left {
            self.right_hold_frames = 0;
            if pressed & BTN_LEFT != 0 || was_overlapping {
                self.left_hold_frames = 0;
                return self.try_horizontal_move(-1);
            }

            self.left_hold_frames = self.left_hold_frames.saturating_add(1);
            if should_repeat_horizontal(self.left_hold_frames) {
                return self.try_horizontal_move(-1);
            }
        } else {
            self.left_hold_frames = 0;
            if pressed & BTN_RIGHT != 0 || was_overlapping {
                self.right_hold_frames = 0;
                return self.try_horizontal_move(1);
            }

            self.right_hold_frames = self.right_hold_frames.saturating_add(1);
            if should_repeat_horizontal(self.right_hold_frames) {
                return self.try_horizontal_move(1);
            }
        }

        SoundEvent::None
    }

    fn try_horizontal_move(&mut self, dx: i8) -> SoundEvent {
        let was_grounded = self.grounded();
        if self.try_move(dx, 0) {
            self.reset_lock_delay_after_action(was_grounded);
            SoundEvent::Move
        } else {
            SoundEvent::None
        }
    }

    fn update_lock_delay(&mut self) -> Option<SoundEvent> {
        if !self.grounded() {
            self.lock_timer = 0;
            return None;
        }

        self.lock_timer = self.lock_timer.saturating_add(1);
        if self.lock_timer > LOCK_DELAY_FRAMES {
            Some(self.lock_piece())
        } else {
            None
        }
    }

    fn hard_drop(&mut self) -> SoundEvent {
        let mut dropped = 0u32;
        while self.try_move(0, 1) {
            dropped += 1;
        }

        if dropped > 0 {
            self.score = self.score.saturating_add(dropped * 2);
            self.touch_hi_score();
        }
        self.lock_piece()
    }

    fn grounded(&self) -> bool {
        !self.fits(self.current, self.x, self.y + 1)
    }

    fn buffer_input(&mut self, pressed: u8) {
        let input = pressed & BUFFERABLE_INPUT;
        if input != 0 {
            self.buffered_input |= input;
            self.input_buffer_timer = INPUT_BUFFER_FRAMES;
        }
    }

    fn apply_buffered_input(&mut self) {
        if self.input_buffer_timer == 0 || self.mode != Mode::Playing {
            return;
        }

        let input = self.buffered_input;
        self.buffered_input = 0;
        self.input_buffer_timer = 0;

        if input & BTN_1 != 0 {
            self.try_rotate(1);
        } else if input & BTN_2 != 0 {
            self.try_rotate(-1);
        }

        if input & BTN_LEFT != 0 && input & BTN_RIGHT == 0 {
            self.try_move(-1, 0);
        } else if input & BTN_RIGHT != 0 && input & BTN_LEFT == 0 {
            self.try_move(1, 0);
        }
    }

    fn age_input_buffer(&mut self) {
        if self.input_buffer_timer == 0 {
            return;
        }

        self.input_buffer_timer -= 1;
        if self.input_buffer_timer == 0 {
            self.buffered_input = 0;
        }
    }

    fn tick_soft_drop_sound_timer(&mut self) {
        if self.soft_drop_sound_timer > 0 {
            self.soft_drop_sound_timer -= 1;
        }
    }

    fn reset_timing_state(&mut self) {
        self.left_hold_frames = 0;
        self.right_hold_frames = 0;
        self.lock_timer = 0;
        self.lock_resets = 0;
        self.buffered_input = 0;
        self.input_buffer_timer = 0;
        self.soft_drop_sound_timer = 0;
    }

    fn reset_lock_state_after_descent(&mut self) {
        self.lock_timer = 0;
        self.lock_resets = 0;
    }

    fn reset_lock_delay_after_action(&mut self, was_grounded: bool) {
        if !was_grounded {
            self.lock_timer = 0;
            return;
        }

        if self.lock_resets < LOCK_RESET_LIMIT {
            self.lock_timer = 0;
            self.lock_resets += 1;
        }
    }

    #[cfg(test)]
    fn clear_lines(&mut self) -> u8 {
        let mask = self.full_rows_mask();
        let cleared = count_rows(mask);
        if cleared > 0 {
            self.apply_clear_mask(mask);
        }

        cleared
    }

    fn full_rows_mask(&self) -> u32 {
        let mut mask = 0u32;
        for y in 0..BOARD_HEIGHT {
            if self.row_full(y) {
                mask |= row_mask(y);
            }
        }
        mask
    }

    fn apply_clear_mask(&mut self, mask: u32) {
        let mut write_y = BOARD_HEIGHT as i16 - 1;

        for read_y in (0..BOARD_HEIGHT).rev() {
            if mask & row_mask(read_y) == 0 {
                for x in 0..BOARD_WIDTH {
                    let source = read_y * BOARD_WIDTH + x;
                    let target = write_y as usize * BOARD_WIDTH + x;
                    self.board[target] = self.board[source];
                    self.board_ids[target] = self.board_ids[source];
                    self.board_styles[target] = self.board_styles[source];
                }
                write_y -= 1;
            }
        }

        while write_y >= 0 {
            for x in 0..BOARD_WIDTH {
                let index = write_y as usize * BOARD_WIDTH + x;
                self.board[index] = 0;
                self.board_ids[index] = 0;
                self.board_styles[index] = EMPTY_STYLE;
            }
            write_y -= 1;
        }
    }

    fn row_full(&self, y: usize) -> bool {
        for x in 0..BOARD_WIDTH {
            if self.board[y * BOARD_WIDTH + x] == 0 {
                return false;
            }
        }
        true
    }

    fn add_line_score(&mut self, cleared: u8) {
        let base = match cleared {
            1 => 40,
            2 => 100,
            3 => 300,
            _ => 1200,
        };
        self.score = self
            .score
            .saturating_add(base * (u32::from(self.level) + 1));
        self.lines = self.lines.saturating_add(cleared as u16);
        self.level = (self.lines / 10).min(19) as u8;
        self.touch_hi_score();
    }

    fn touch_hi_score(&mut self) {
        if self.score > self.hi_score {
            self.hi_score = self.score;
            self.save_requested = true;
        }
    }

    fn fits(&self, piece: Piece, x: i8, y: i8) -> bool {
        for (cx, cy) in Self::piece_cells(piece) {
            let bx = x + cx;
            let by = y + cy;
            if bx < 0 || bx >= BOARD_WIDTH as i8 || by >= BOARD_HEIGHT as i8 {
                return false;
            }
            if by >= 0 && self.board[by as usize * BOARD_WIDTH + bx as usize] != 0 {
                return false;
            }
        }
        true
    }

    fn gravity_frames(&self) -> u32 {
        let base = 48u32.saturating_sub(u32::from(self.level) * 3);
        base.max(6)
    }

    fn random_piece(&mut self) -> u8 {
        ((self.next_random() >> 16) % 7) as u8
    }

    fn random_style(&mut self) -> BlockStyle {
        let color = 2 + ((self.next_random() >> 16) % 3) as u8;
        let pattern = match (self.next_random() >> 16) % 3 {
            0 => BlockPattern::Solid,
            1 => BlockPattern::InnerFrame,
            _ => BlockPattern::Dither,
        };
        BlockStyle { color, pattern }
    }

    fn set_demo_target(&mut self) {
        let mut best_score = i32::MIN;
        let mut best_x = START_X;
        let mut best_rot = 0u8;

        for rot in 0..4 {
            let piece = Piece {
                kind: self.current.kind,
                rot,
            };
            let (min_target, max_target) = target_range(piece);
            for x in min_target..=max_target {
                if !self.fits(piece, x, START_Y) {
                    continue;
                }

                let mut y = START_Y;
                while self.fits(piece, x, y + 1) {
                    y += 1;
                }

                let score = self.demo_placement_score(piece, x, y);
                if score > best_score {
                    best_score = score;
                    best_x = x;
                    best_rot = rot;
                }
            }
        }

        self.demo_target_x = best_x;
        self.demo_target_rot = best_rot;
    }

    fn demo_placement_score(&self, piece: Piece, piece_x: i8, piece_y: i8) -> i32 {
        let mut board = self.board;
        let cells = Self::piece_cells(piece);
        for (cx, cy) in cells {
            let x = piece_x + cx;
            let y = piece_y + cy;
            if x >= 0 && x < BOARD_WIDTH as i8 && y >= 0 && y < BOARD_HEIGHT as i8 {
                board[y as usize * BOARD_WIDTH + x as usize] = piece.kind + 1;
            }
        }

        let mut completed = 0u8;
        for y in 0..BOARD_HEIGHT {
            if row_full_in(&board, y) {
                completed += 1;
            }
        }

        let mut heights = [0u8; BOARD_WIDTH];
        let mut holes = 0i32;
        for x in 0..BOARD_WIDTH {
            let mut seen_block = false;
            for y in 0..BOARD_HEIGHT {
                let filled = board[y * BOARD_WIDTH + x] != 0;
                if filled && !seen_block {
                    heights[x] = (BOARD_HEIGHT - y) as u8;
                    seen_block = true;
                } else if !filled && seen_block {
                    holes += 1;
                }
            }
        }

        let mut aggregate_height = 0i32;
        for height in heights {
            aggregate_height += i32::from(height);
        }

        let mut bumpiness = 0i32;
        for x in 0..BOARD_WIDTH - 1 {
            bumpiness += (i32::from(heights[x]) - i32::from(heights[x + 1])).abs();
        }

        i32::from(completed) * 900 - holes * 380 - aggregate_height * 7 - bumpiness * 20
    }

    fn next_random(&mut self) -> u32 {
        self.rng = self.rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.rng
    }

    fn take_next_piece_id(&mut self) -> u16 {
        let id = self.next_piece_id;
        self.next_piece_id = self.next_piece_id.wrapping_add(1);
        if self.next_piece_id == 0 {
            self.next_piece_id = 1;
        }
        id
    }
}

fn row_mask(y: usize) -> u32 {
    1 << y
}

fn count_rows(mask: u32) -> u8 {
    mask.count_ones() as u8
}

fn row_full_in(board: &[u8; BOARD_CELLS], y: usize) -> bool {
    for x in 0..BOARD_WIDTH {
        if board[y * BOARD_WIDTH + x] == 0 {
            return false;
        }
    }
    true
}

fn should_repeat_horizontal(held_frames: u8) -> bool {
    held_frames >= DAS_FRAMES && (held_frames - DAS_FRAMES) % ARR_FRAMES == 0
}

fn target_range(piece: Piece) -> (i8, i8) {
    let cells = Game::piece_cells(piece);
    let mut min_x = 4i8;
    let mut max_x = -4i8;
    for (x, _) in cells {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
    }
    (-min_x, BOARD_WIDTH as i8 - 1 - max_x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn playing_game() -> Game {
        let mut game = Game::new(0);
        game.tick(BTN_1);
        game.tick(0);
        game
    }

    #[test]
    fn title_screen_requires_start_input() {
        let mut game = Game::new(0);

        for _ in 0..120 {
            game.tick(0);
        }

        assert_eq!(game.mode(), Mode::Title);
        game.tick(BTN_1);
        assert_eq!(game.mode(), Mode::Playing);
    }

    #[test]
    fn title_screen_runs_cpu_demo_without_starting_game() {
        let mut game = Game::new(0);

        for _ in 0..240 {
            game.tick(0);
        }

        assert_eq!(game.mode(), Mode::Title);
        assert_eq!(game.score(), 0);
        assert!(game.board.iter().any(|cell| *cell != 0));
    }

    #[test]
    fn starting_game_clears_cpu_demo_board() {
        let mut game = Game::new(0);
        for _ in 0..240 {
            game.tick(0);
        }
        assert!(game.board.iter().any(|cell| *cell != 0));

        game.tick(BTN_1);

        assert_eq!(game.mode(), Mode::Playing);
        assert!(game.board.iter().all(|cell| *cell == 0));
        assert!(game.board_styles.iter().all(|style| *style == EMPTY_STYLE));
    }

    #[test]
    fn cpu_demo_prefers_line_clear_placement() {
        let mut game = Game::new(0);
        game.current = Piece { kind: 0, rot: 0 };
        game.x = START_X;
        game.y = START_Y;
        for x in 0..BOARD_WIDTH {
            if !(3..=6).contains(&x) {
                game.board[19 * BOARD_WIDTH + x] = 1;
            }
        }

        game.set_demo_target();

        assert_eq!(game.demo_target_rot, 0);
        assert_eq!(game.demo_target_x, 3);
    }

    #[test]
    fn rotation_kicks_piece_away_from_left_wall() {
        let mut game = playing_game();
        game.current = Piece { kind: 0, rot: 1 };
        game.x = -1;
        game.y = 0;

        let result = game.tick(BTN_1);

        assert_eq!(result.sound, SoundEvent::Rotate);
        assert!(game.position().0 >= 0);
    }

    #[test]
    fn held_horizontal_movement_waits_for_das_before_repeating() {
        let mut game = playing_game();
        game.current = Piece { kind: 1, rot: 0 };
        game.x = 4;
        game.y = 0;

        game.tick(BTN_LEFT);
        assert_eq!(game.position().0, 3);

        for _ in 0..11 {
            game.tick(BTN_LEFT);
        }
        assert_eq!(game.position().0, 3);

        game.tick(BTN_LEFT);
        assert_eq!(game.position().0, 2);
    }

    #[test]
    fn releasing_one_of_two_horizontal_inputs_moves_remaining_direction() {
        let mut game = playing_game();
        game.current = Piece { kind: 1, rot: 0 };
        game.x = 4;
        game.y = 0;

        game.tick(BTN_LEFT);
        assert_eq!(game.position().0, 3);

        game.tick(BTN_LEFT | BTN_RIGHT);
        assert_eq!(game.position().0, 3);

        game.tick(BTN_RIGHT);
        assert_eq!(game.position().0, 4);
    }

    #[test]
    fn buffered_rotation_applies_after_line_clear_animation() {
        let mut game = playing_game();
        game.current = Piece { kind: 0, rot: 0 };
        game.next = Piece { kind: 2, rot: 0 };
        game.x = 3;
        game.y = 18;

        for x in 0..BOARD_WIDTH {
            if !(3..=6).contains(&x) {
                game.board[19 * BOARD_WIDTH + x] = 2;
            }
        }

        assert_eq!(game.lock_piece(), SoundEvent::Line(1));
        game.tick(BTN_1);
        game.tick(0);
        game.tick(BTN_LEFT);
        game.tick(0);
        while game.is_clearing() {
            game.tick(0);
        }

        assert_eq!(game.current.kind, 2);
        assert_eq!(game.current.rot, 1);
        assert_eq!(game.position().0, START_X - 1);
    }

    #[test]
    fn grounded_piece_uses_lock_delay_before_locking() {
        let mut game = playing_game();
        game.current = Piece { kind: 1, rot: 0 };
        game.x = 4;
        game.y = 18;
        game.frame = game.gravity_frames() - 1;

        let result = game.tick(0);

        assert_eq!(result.sound, SoundEvent::None);
        assert_eq!(game.current.kind, 1);
        assert!(game.board.iter().all(|cell| *cell == 0));

        for _ in 0..20 {
            game.tick(0);
        }

        assert!(game.board.iter().any(|cell| *cell != 0));
    }

    #[test]
    fn grounded_lock_delay_resets_are_limited() {
        let mut game = playing_game();
        game.current = Piece { kind: 1, rot: 0 };
        game.x = 4;
        game.y = 18;
        game.frame = game.gravity_frames() - 1;

        game.tick(0);

        for turn in 0..usize::from(LOCK_RESET_LIMIT) + usize::from(LOCK_DELAY_FRAMES) + 8 {
            let input = if turn % 2 == 0 { BTN_LEFT } else { BTN_RIGHT };
            game.tick(input);
            game.tick(0);
            if game.board.iter().any(|cell| *cell != 0) {
                break;
            }
        }

        assert!(game.board.iter().any(|cell| *cell != 0));
    }

    #[test]
    fn up_input_hard_drops_and_locks_the_piece() {
        let mut game = playing_game();
        game.current = Piece { kind: 1, rot: 0 };
        game.x = 4;
        game.y = 0;

        let result = game.tick(BTN_UP);

        assert_eq!(result.sound, SoundEvent::Lock);
        assert_eq!(game.cell(5, 18), 2);
        assert_eq!(game.cell(6, 19), 2);
        assert_eq!(game.score(), 36);
    }

    #[test]
    fn clearing_four_lines_scores_like_classic_tetris() {
        let mut game = playing_game();
        for y in 16..20 {
            for x in 0..BOARD_WIDTH {
                game.board[y * BOARD_WIDTH + x] = 2;
            }
        }

        let cleared = game.clear_lines();
        game.add_line_score(cleared);

        assert_eq!(cleared, 4);
        assert_eq!(game.lines(), 4);
        assert_eq!(game.score(), 1200);
        assert_eq!(game.hi_score(), 1200);
        assert!(game.take_save_request());
    }

    #[test]
    fn line_clear_waits_for_animation_before_shifting_board() {
        let mut game = playing_game();
        game.current = Piece { kind: 0, rot: 0 };
        game.x = 3;
        game.y = 18;

        for x in 0..BOARD_WIDTH {
            if !(3..=6).contains(&x) {
                game.board[19 * BOARD_WIDTH + x] = 2;
                game.board_ids[19 * BOARD_WIDTH + x] = 8;
            }
        }

        let sound = game.lock_piece();

        assert_eq!(sound, SoundEvent::Line(1));
        assert!(game.is_clearing());
        assert!(game.row_full(19));
        assert_eq!(game.lines(), 1);
        assert_eq!(game.score(), 40);

        for _ in 0..LINE_CLEAR_FRAMES - 1 {
            game.tick(0);
        }
        assert!(game.is_clearing());
        assert!(game.row_full(19));

        game.tick(0);

        assert!(!game.is_clearing());
        assert_eq!(game.clear_mask, 0);
        assert!(!game.row_full(19));
    }

    #[test]
    fn locked_cells_keep_the_current_piece_visual_style() {
        let mut game = playing_game();
        game.current = Piece { kind: 1, rot: 0 };
        let style = BlockStyle {
            color: 4,
            pattern: BlockPattern::InnerFrame,
        };
        game.current_style = style;
        game.x = 3;
        game.y = 16;

        let sound = game.lock_piece();

        assert_eq!(sound, SoundEvent::Lock);
        for (cx, cy) in Game::piece_cells(Piece { kind: 1, rot: 0 }) {
            let x = (3 + cx) as usize;
            let y = (16 + cy) as usize;
            assert_eq!(game.cell_style(x, y), style);
        }
    }

    #[test]
    fn line_clear_shifts_visual_style_with_surviving_cells() {
        let mut game = playing_game();
        let survivor_style = BlockStyle {
            color: 3,
            pattern: BlockPattern::Dither,
        };
        game.board[18 * BOARD_WIDTH + 2] = 1;
        game.board_ids[18 * BOARD_WIDTH + 2] = 12;
        game.board_styles[18 * BOARD_WIDTH + 2] = survivor_style;
        for x in 0..BOARD_WIDTH {
            game.board[19 * BOARD_WIDTH + x] = 2;
            game.board_ids[19 * BOARD_WIDTH + x] = 8;
            game.board_styles[19 * BOARD_WIDTH + x] = BlockStyle {
                color: 4,
                pattern: BlockPattern::InnerFrame,
            };
        }

        let cleared = game.clear_lines();

        assert_eq!(cleared, 1);
        assert_eq!(game.cell(2, 19), 1);
        assert_eq!(game.cell_id(2, 19), 12);
        assert_eq!(game.cell_style(2, 19), survivor_style);
        assert_eq!(game.cell_style(2, 18).color, 0);
    }

    #[test]
    fn random_styles_keep_only_readable_pattern_variants() {
        let mut game = playing_game();
        let mut seen_solid = false;
        let mut seen_inner_frame = false;
        let mut seen_dither = false;

        for _ in 0..64 {
            match game.random_style().pattern {
                BlockPattern::Solid => seen_solid = true,
                BlockPattern::InnerFrame => seen_inner_frame = true,
                BlockPattern::Dither => seen_dither = true,
            }
        }

        assert!(seen_solid);
        assert!(seen_inner_frame);
        assert!(seen_dither);
    }

    #[test]
    fn game_over_when_spawn_area_is_blocked() {
        let mut game = playing_game();
        for x in 3..7 {
            game.board[x] = 1;
        }

        game.spawn_next();

        assert_eq!(game.mode(), Mode::GameOver);
    }
}
