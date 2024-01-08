use raylib::prelude::*;
use std::{default::Default, ops::Deref};

use State::{Alive, Dead};
type Board = [[Cell; WIDTH]; HEIGHT];

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const GOL1: [[State; 9]; 2] = [
    [Dead, Dead, Dead, Alive, Dead, Dead, Alive, Alive, Dead],
    [Dead, Dead, Alive, Alive, Dead, Dead, Dead, Dead, Alive],
];

const DN: [[State; 9]; 2] = [
    [Dead, Dead, Dead, Alive, Dead, Dead, Alive, Alive, Alive],
    [Dead, Dead, Dead, Alive, Alive, Dead, Alive, Alive, Alive],
];

const DN2: [[State; 9]; 2] = [
    [Dead, Dead, Alive, Alive, Alive, Dead, Alive, Alive, Dead],
    [Alive, Dead, Dead, Alive, Alive, Dead, Alive, Alive, Alive],
];
const GOL: [[State; 9]; 2] = [
    [Dead, Dead, Dead, Alive, Dead, Dead, Alive, Alive, Dead],
    [Dead, Dead, Alive, Alive, Alive, Dead, Dead, Alive, Alive],
];

const GOL2: [[State; 9]; 2] = [
    [Dead, Alive, Dead, Alive, Dead, Dead, Dead, Dead, Dead],
    [Alive, Alive, Dead, Alive, Alive, Dead, Dead, Dead, Dead],
];
const GOL3: [[State; 9]; 2] = [
    [Dead, Alive, Dead, Dead, Dead, Dead, Alive, Dead, Dead],
    [Alive, Alive, Dead, Dead, Dead, Alive, Alive, Alive, Alive],
];

const GOL4: [[State; 9]; 2] = [
    [Dead, Alive, Dead, Dead, Dead, Dead, Alive, Dead, Dead],
    [Alive, Alive, Dead, Alive, Alive, Dead, Alive, Alive, Alive],
];

const GOL5: [[State; 9]; 2] = [
    [Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead],
    [Dead, Dead, Alive, Alive, Dead, Dead, Dead, Dead, Alive],
];
const AUTOMATA: [[State; 9]; 2] = GOL1;
#[derive(PartialEq, Eq, Clone, Copy)]
struct Cell {
    color: Color,
    state: State,
}

impl Cell {
    fn new(color: Color, state: State) -> Self {
        Self { color, state }
    }
    fn default() -> Self {
        Self {
            color: Color::new(18, 18, 18, 255),
            state: Dead,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Alive,
    Dead,
}

impl State {
    fn as_usize(&self) -> usize {
        match self {
            State::Dead => 0,
            State::Alive => 1,
        }
    }
}

fn count_neighbors(x: i32, y: i32, cells: &Board) -> i32 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let x = x + i;
            let y = y + j;
            if x < 0 || x >= WIDTH as i32 || y < 0 || y >= HEIGHT as i32 {
                continue;
            }
            if i == 0 && j == 0 {
                continue;
            }
            if cells[x as usize][y as usize].state == Alive {
                count += 1;
            }
        }
    }
    count
}

fn next_board(cells: &Board) -> Board {
    let mut next = [[Cell::default(); WIDTH]; HEIGHT];
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let cell = cells[x][y];
            let count = count_neighbors(x as i32, y as i32, cells);
            let next_state = AUTOMATA[cell.state.as_usize()][count as usize];
            next[x][y] = Cell::new(cell.color, next_state);
        }
    }
    next
}

fn image_to_board(image: Image) -> Board {
    let mut board = [[Cell::default(); WIDTH]; HEIGHT];
    let data = image.get_image_data();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let cell = data[x + y * image.width() as usize];
            board[x][y] = Cell::new(cell, Alive);
        }
    }
    board
}

fn display_board(board: &Board, d: &mut RaylibDrawHandle) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let cell = board[x][y];
            let color = if cell.state == Alive {
                cell.color
            } else {
                Color::new(18, 18, 18, 255)
            };
            d.draw_rectangle(x as i32 * 2, y as i32 * 2, 3, 3, color);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(256 * 2, 256 * 2)
        .title("pngatoma")
        .build();

    let nex = Image::load_image("nexfools.png").unwrap();
    let mut nex_board = image_to_board(nex);

    let bg = Color::new(18, 18, 18, 255);
    let mut isplaying = false;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg);
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            isplaying = !isplaying;
        }
        if isplaying {
            display_board(&nex_board, &mut d);
            nex_board = next_board(&nex_board);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
