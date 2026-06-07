use crate::game::{
    BlockPattern, BlockStyle, Game, Mode, Piece, BOARD_HEIGHT, BOARD_WIDTH, LINE_CLEAR_FRAMES,
};
use crate::wasm4::*;

const CELL: i32 = 6;
const BOARD_X: i32 = 18;
const BOARD_Y: i32 = 20;
const PANEL_X: i32 = 88;

pub fn draw(game: &Game) {
    clear();

    match game.mode() {
        Mode::Title => draw_title(game),
        Mode::Playing => draw_playing(game),
        Mode::GameOver => {
            draw_playing(game);
            draw_game_over(game);
        }
    }
}

fn clear() {
    unsafe {
        *DRAW_COLORS = 0x0001;
    }
    rect(0, 0, 160, 160);
}

fn draw_title(game: &Game) {
    draw_board_frame();
    draw_locked_blocks(game);
    if !game.is_clearing() {
        draw_current_piece(game);
    }
    draw_title_overlay(game);
}

fn draw_playing(game: &Game) {
    draw_board_frame();
    draw_locked_blocks(game);
    if !game.is_clearing() {
        draw_current_piece(game);
    }
    draw_panel(game);
}

fn draw_board_frame() {
    unsafe {
        *DRAW_COLORS = 0x0033;
    }
    rect(
        BOARD_X - 3,
        BOARD_Y - 3,
        (BOARD_WIDTH as i32 * CELL + 6) as u32,
        (BOARD_HEIGHT as i32 * CELL + 6) as u32,
    );

    unsafe {
        *DRAW_COLORS = 0x0001;
    }
    rect(
        BOARD_X,
        BOARD_Y,
        (BOARD_WIDTH as i32 * CELL) as u32,
        (BOARD_HEIGHT as i32 * CELL) as u32,
    );
}

fn draw_locked_blocks(game: &Game) {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let kind = game.cell(x, y);
            if kind != 0 && locked_cell_visible(game, x, y) {
                draw_styled_cell(
                    BOARD_X + x as i32 * CELL,
                    BOARD_Y + y as i32 * CELL,
                    CELL,
                    game.cell_style(x, y),
                );
            }
        }
    }

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if locked_cell_visible(game, x, y) {
                let id = game.cell_id(x, y);
                let style = game.cell_style(x, y);
                draw_cell_outline(
                    BOARD_X + x as i32 * CELL,
                    BOARD_Y + y as i32 * CELL,
                    CELL,
                    style.color,
                    !same_locked_piece_visible(game, x as i32, y as i32 - 1, id),
                    !same_locked_piece_visible(game, x as i32 + 1, y as i32, id),
                    !same_locked_piece_visible(game, x as i32, y as i32 + 1, id),
                    !same_locked_piece_visible(game, x as i32 - 1, y as i32, id),
                );
            }
        }
    }
}

fn draw_current_piece(game: &Game) {
    let piece = game.current();
    let (px, py) = game.position();
    let cells = Game::piece_cells(piece);
    let style = game.current_style();

    for (cx, cy) in Game::piece_cells(piece) {
        let x = px + cx;
        let y = py + cy;
        if y >= 0 {
            draw_styled_cell(
                BOARD_X + x as i32 * CELL,
                BOARD_Y + y as i32 * CELL,
                CELL,
                style,
            );
        }
    }

    for (cx, cy) in cells {
        let x = px + cx;
        let y = py + cy;
        if y >= 0 {
            draw_cell_outline(
                BOARD_X + x as i32 * CELL,
                BOARD_Y + y as i32 * CELL,
                CELL,
                style.color,
                !piece_has_board_cell(cells, px, py, x, y - 1),
                !piece_has_board_cell(cells, px, py, x + 1, y),
                !piece_has_board_cell(cells, px, py, x, y + 1),
                !piece_has_board_cell(cells, px, py, x - 1, y),
            );
        }
    }
}

fn draw_panel(game: &Game) {
    set_text_color(2);
    text("NEXT", PANEL_X, 20);
    draw_next_piece(game);

    set_text_color(3);
    text("SCORE", PANEL_X, 58);
    draw_u32(game.score(), PANEL_X, 70, 4);

    text("LINES", PANEL_X, 90);
    draw_u32(game.lines() as u32, PANEL_X, 102, 3);

    text("LEVEL", PANEL_X, 122);
    draw_u32(game.level() as u32, PANEL_X, 134, 2);
}

fn draw_next_piece(game: &Game) {
    let next = game.next();
    draw_piece_preview(next, game.next_style(), PANEL_X, 32, 5);
}

fn draw_game_over(game: &Game) {
    draw_overlay_panel(18, 48, 124, 78);
    set_text_color(4);
    text("GAME OVER", 43, 57);
    set_text_color(2);
    text("HI SCORE", 48, 74);
    draw_u32(game.hi_score(), 56, 86, 4);
    set_text_color(blink_text_color(game));
    text("PRESS X/Z", 44, 102);
    text("RESTART", 52, 114);
}

fn draw_title_overlay(game: &Game) {
    draw_overlay_panel(PANEL_X - 4, 54, 68, 74);
    set_text_color(3);
    text("HI", PANEL_X + 18, 62);
    draw_u32(game.hi_score(), PANEL_X + 10, 74, 4);
    set_text_color(blink_text_color(game));
    text("PRESS", PANEL_X + 6, 94);
    text("X/Z", PANEL_X + 14, 106);
    text("START", PANEL_X + 6, 118);
}

fn draw_overlay_panel(x: i32, y: i32, width: u32, height: u32) {
    unsafe {
        *DRAW_COLORS = 0x0011;
    }
    rect(x, y, width, height);
    set_line_color(2);
    hline(x, y, width);
    hline(x, y + height as i32 - 1, width);
    vline(x, y, height);
    vline(x + width as i32 - 1, y, height);
}

fn locked_cell_visible(game: &Game, x: usize, y: usize) -> bool {
    game.cell(x, y) != 0 && !clear_animation_hides_cell(game, x, y)
}

fn same_locked_piece_visible(game: &Game, x: i32, y: i32, id: u16) -> bool {
    if id == 0 || x < 0 || y < 0 || x >= BOARD_WIDTH as i32 || y >= BOARD_HEIGHT as i32 {
        return false;
    }

    let x = x as usize;
    let y = y as usize;
    locked_cell_visible(game, x, y) && game.cell_id(x, y) == id
}

fn clear_animation_hides_cell(game: &Game, x: usize, y: usize) -> bool {
    if !game.clearing_row(y) {
        return false;
    }

    let wipe = ((clear_animation_elapsed(game) as usize * (BOARD_WIDTH + 2))
        / LINE_CLEAR_FRAMES as usize)
        .min(BOARD_WIDTH);
    if wipe == 0 {
        return false;
    }

    let left = (BOARD_WIDTH - wipe) / 2;
    x >= left && x < left + wipe
}

fn clear_animation_elapsed(game: &Game) -> u8 {
    LINE_CLEAR_FRAMES.saturating_sub(game.clear_timer())
}

fn draw_piece_preview(piece: Piece, style: BlockStyle, x: i32, y: i32, size: i32) {
    let cells = Game::piece_cells(piece);

    for (cx, cy) in cells {
        draw_styled_cell(x + cx as i32 * size, y + cy as i32 * size, size, style);
    }

    for (cx, cy) in cells {
        draw_cell_outline(
            x + cx as i32 * size,
            y + cy as i32 * size,
            size,
            style.color,
            !piece_has_local_cell(cells, cx, cy - 1),
            !piece_has_local_cell(cells, cx + 1, cy),
            !piece_has_local_cell(cells, cx, cy + 1),
            !piece_has_local_cell(cells, cx - 1, cy),
        );
    }
}

fn piece_has_board_cell(
    cells: [(i8, i8); 4],
    piece_x: i8,
    piece_y: i8,
    board_x: i8,
    board_y: i8,
) -> bool {
    if board_y < 0 {
        return false;
    }

    for (cx, cy) in cells {
        if piece_x + cx == board_x && piece_y + cy == board_y {
            return true;
        }
    }
    false
}

fn piece_has_local_cell(cells: [(i8, i8); 4], x: i8, y: i8) -> bool {
    for (cx, cy) in cells {
        if cx == x && cy == y {
            return true;
        }
    }
    false
}

fn draw_styled_cell(x: i32, y: i32, size: i32, style: BlockStyle) {
    if style.color == 0 {
        return;
    }

    match style.pattern {
        BlockPattern::Solid => draw_cell_fill(x, y, size, style.color),
        BlockPattern::Cutout => {
            draw_cell_fill(x, y, size, style.color);
            if size > 4 {
                draw_cell_cutout(x, y, size, 2);
            }
        }
        BlockPattern::InnerFrame => {
            draw_cell_fill(x, y, size, style.color);
            if size > 4 {
                draw_inner_frame_cut(x, y, size);
            }
        }
        BlockPattern::Dither => draw_dither_cell(x, y, size, style.color),
    }
}

fn draw_cell_fill(x: i32, y: i32, size: i32, color: u8) {
    unsafe {
        *DRAW_COLORS = color as u16 | 0x0010;
    }
    rect(x, y, size as u32, size as u32);
}

fn draw_cell_cutout(x: i32, y: i32, size: i32, inset: i32) {
    unsafe {
        *DRAW_COLORS = 0x0001;
    }
    rect(
        x + inset,
        y + inset,
        (size - inset * 2) as u32,
        (size - inset * 2) as u32,
    );
}

fn draw_inner_frame_cut(x: i32, y: i32, size: i32) {
    set_line_color(1);
    hline(x + 2, y + 2, (size - 4) as u32);
    hline(x + 2, y + size - 3, (size - 4) as u32);
    vline(x + 2, y + 2, (size - 4) as u32);
    vline(x + size - 3, y + 2, (size - 4) as u32);
}

fn draw_dither_cell(x: i32, y: i32, size: i32, color: u8) {
    unsafe {
        *DRAW_COLORS = color as u16 | 0x0010;
    }
    for yy in 0..size {
        for xx in 0..size {
            if (xx + yy) % 2 == 0 {
                rect(x + xx, y + yy, 1, 1);
            }
        }
    }
}

fn draw_cell_outline(
    x: i32,
    y: i32,
    size: i32,
    color: u8,
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
) {
    set_line_color(color);
    if top {
        hline(x, y, size as u32);
    }
    if right {
        vline(x + size - 1, y, size as u32);
    }
    if bottom {
        hline(x, y + size - 1, size as u32);
    }
    if left {
        vline(x, y, size as u32);
    }
}

fn set_text_color(color: u8) {
    unsafe {
        *DRAW_COLORS = color as u16;
    }
}

fn blink_text_color(game: &Game) -> u8 {
    if game.frame() / 28 % 2 == 0 {
        2
    } else {
        3
    }
}

fn set_line_color(color: u8) {
    unsafe {
        *DRAW_COLORS = color as u16;
    }
}

fn draw_u32(mut value: u32, x: i32, y: i32, min_width: usize) {
    let mut buf = [b'0'; 10];
    let mut index = buf.len();
    let mut written = 0usize;

    loop {
        index -= 1;
        buf[index] = b'0' + (value % 10) as u8;
        value /= 10;
        written += 1;
        if value == 0 && written >= min_width {
            break;
        }
    }

    set_text_color(2);
    text(&buf[index..], x, y);
}
