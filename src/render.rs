// src/render.rs

use web_sys::CanvasRenderingContext2d;

#[derive(Clone)]
pub struct Cell {
    pub is_mine: bool,
    pub adj: u8,
    pub revealed: bool,
    pub flagged: bool,
    pub questioned: bool,
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
        Cell {
            is_mine: false,
            adj: 0,
            revealed: false,
            flagged: false,
            questioned: false,
            x,
            y,
        }
    }
    pub fn toggle_flag(&mut self) {
        if !self.revealed {
            self.flagged = !self.flagged;
            if self.flagged {
                self.questioned = false;
            }
        }
    }
    pub fn toggle_question(&mut self) {
        if !self.revealed {
            self.questioned = !self.questioned;
            if self.questioned {
                self.flagged = false;
            }
        }
    }
}

pub fn clear(context: &CanvasRenderingContext2d, width: u32, height: u32) {
    context.set_fill_style_str("#eee");
    context.fill_rect(0.0, 0.0, width as f64, height as f64);
}

pub fn draw_grid(context: &CanvasRenderingContext2d, board: &[Vec<Cell>], cell_size: u32) {
    context.set_stroke_style_str("#999");
    for row in board {
        for cell in row {
            let x = (cell.x as u32 * cell_size) as f64;
            let y = (cell.y as u32 * cell_size) as f64;

            if cell.revealed {
                if cell.is_mine {
                    context.set_fill_style_str("red");
                } else {
                    context.set_fill_style_str("white");
                }
            } else {
                context.set_fill_style_str("#666");
            }
            context.fill_rect(x, y, cell_size as f64, cell_size as f64);
            
            context.set_fill_style_str("black");
            context.set_text_align("left");
            context.set_text_baseline("alphabetic");

            if !cell.revealed {
                if cell.flagged {
                    context.set_font(&format!("{}px sans-serif", cell_size / 2));
                    context.set_text_align("center");
                    context.set_text_baseline("middle");
                    context
                        .fill_text("🚩", x + cell_size as f64 / 2.0, y + cell_size as f64 / 2.0)
                        .unwrap();
                } else if cell.questioned {
                    context.set_font(&format!("{}px sans-serif", cell_size / 2));
                    context.set_text_align("center");
                    context.set_text_baseline("middle");
                    context
                        .fill_text("?", x + cell_size as f64 / 2.0, y + cell_size as f64 / 2.0)
                        .unwrap();
                }
            } else if cell.revealed && !cell.is_mine {
                if cell.adj > 0 {
                    context.set_fill_style_str("black");
                    context.set_font(&format!("{}px sans-serif", cell_size / 2));
                    context
                        .fill_text(&cell.adj.to_string(), x + 4.0, y + (cell_size / 2) as f64)
                        .unwrap();
                }
            }

            context.set_stroke_style_str("#222");
            context.stroke_rect(x, y, cell_size as f64, cell_size as f64);
        }
    }
}
