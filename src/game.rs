// src/game.rs

use crate::input::InputHandler;
use crate::render::Cell;
use web_sys::js_sys::Math;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Reveal,
    Flag,
    Question,
}

pub struct Game {
    pub board: Vec<Vec<Cell>>,
    pub width: u32,
    pub height: u32,
    pub cell_size: u32,
    pub input_handler: InputHandler,
    pub mode: Mode,
    pub mine_count: usize,
    pub game_over: bool,
    pub game_won: bool,
}

impl Game {
    pub fn new(width: u32, height: u32, input_handler: InputHandler) -> Game {
        let cols = 10;
        let rows = 10;
        let cell_size = width / cols;
        let mut board = vec![vec![Cell::new(0, 0); cols as usize]; rows as usize];
        for y in 0..rows as usize {
            for x in 0..cols as usize {
                board[y][x] = Cell::new(x, y);
            }
        }

        let mine_count = 15;
        Game::place_mines(&mut board, mine_count);
        
        Game {
            board,
            width,
            height,
            cell_size,
            input_handler,
            mode: Mode::Reveal,
            mine_count,
            game_over: false,
            game_won: false,
        }
    }

    fn place_mines(board: &mut [Vec<Cell>], count: usize) {
        let rows = board.len();
        let cols = board[0].len();
        let mut placed = 0;

        while placed < count {
            let x = (Math::random() * cols as f64).floor() as usize;
            let y = (Math::random() * rows as f64).floor() as usize;
            if !board[y][x].is_mine {
                board[y][x].is_mine = true;
                placed += 1;
            }
        }

        for y in 0..rows {
            for x in 0..cols {
                board[y][x].adj = Game::count_adjacent(board, x, y);
            }
        }
    }

    fn count_adjacent(board: &[Vec<Cell>], x: usize, y: usize) -> u8 {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, 0),
            (1, -1),
            (1, 1),
        ];
        directions
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0
                    && ny >= 0
                    && (ny as usize) < board.len()
                    && (nx as usize) < board[0].len()
                    && board[ny as usize][nx as usize].is_mine
                {
                    Some(())
                } else {
                    None
                }
            })
            .count() as u8
    }

    fn flood_reveal(&mut self, cx: usize, cy: usize) {
        let rows = self.board.len();
        let cols = self.board[0].len();

        if cx >= rows || cy >= cols {
            return;
        }
        let cell = &mut self.board[cy][cx];
        if cell.revealed || cell.is_mine {
            return;
        }
        cell.revealed = true;
        if cell.adj == 0 {
            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = cx as isize + dx;
                    let ny = cy as isize + dy;
                    if nx >= 0 && ny >= 0 {
                        self.flood_reveal(nx as usize, ny as usize);
                    }
                }
            }
        }
    }
    
    pub fn restart(&mut self) {
        let cols = self.board[0].len();
        let rows = self.board.len();
        self.board = vec![vec![Cell::new(0, 0); cols]; rows];
        for y in 0..rows {
            for x in 0..cols {
                self.board[y][x] = Cell::new(x, y);
            }
        }
        Game::place_mines(&mut self.board, self.mine_count);
        self.mode = Mode::Reveal;
        self.game_over = false;
        self.game_won = false;
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }
        
        for (px, py) in self.input_handler.take_clicks() {
            let cx = (px as u32 / self.cell_size) as usize;
            let cy = (py as u32 / self.cell_size) as usize;
            if cy < self.board.len() && cx < self.board[0].len() {
                let cell = &mut self.board[cy][cx];
                match self.mode {
                    Mode::Reveal => {
                        if cell.is_mine {
                            self.game_over = true;
                        } else {
                            self.flood_reveal(cx, cy);
                            
                            let total = (self.board.len() - self.board[0].len()) as usize;
                            let safe_cells = total - self.board.len();
                            let revealed_safe = self.board
                                .iter()
                                .flat_map(|row| row.iter())
                                .filter(|cell| cell.revealed && !cell.is_mine)
                                .count();
                            if revealed_safe == safe_cells {
                                self.game_over = true;
                                self.game_won = true;
                            }
                        }
                    },
                    Mode::Flag => {
                        let current_flags = self.board
                            .iter()
                            .flat_map(|row| row.iter())
                            .filter(|cell| cell.flagged)
                            .count();
                        let cell = &mut self.board[cy][cx];
                        if cell.flagged {
                            cell.toggle_flag();
                        } else if current_flags < self.mine_count {
                            cell.toggle_flag();
                        }
                    },
                    Mode::Question => {
                        cell.toggle_question();
                    },
                }
            }
        }
    }
}
