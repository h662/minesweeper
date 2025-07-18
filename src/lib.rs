// src/lib.rs

mod game;
mod input;
mod render;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

#[wasm_bindgen]
pub struct Minesweeper {
    game: game::Game,
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Minesweeper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Minesweeper, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().dyn_into()?;
        let context: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;

        let input = input::InputHandler::new(&canvas);
        let width = canvas.width();
        let height = canvas.height();
        let game = game::Game::new(width, height, input);

        Ok(Minesweeper { game, context })
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.game.update();
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        render::clear(&self.context, self.game.width, self.game.height);
        render::draw_grid(&self.context, &self.game.board, self.game.cell_size);
    }

    #[wasm_bindgen(js_name = set_mode)]
    pub fn set_mode_js(&mut self, mode: &str) {
        self.game.mode = match mode {
            "flag" => game::Mode::Flag,
            "question" => game::Mode::Question,
            _ => game::Mode::Reveal,
        }
    }

    #[wasm_bindgen(js_name = current_mode)]
    pub fn current_mode_js(&mut self) -> String {
        match self.game.mode {
            game::Mode::Reveal => "reveal",
            game::Mode::Flag => "flag",
            game::Mode::Question => "question",
        }
        .into()
    }

    #[wasm_bindgen]
    pub fn restart(&mut self) {
        self.game.restart();
    }

    #[wasm_bindgen(js_name = is_game_over)]
    pub fn is_game_over(&self) -> bool {
        self.game.game_over
    }
    
    #[wasm_bindgen(js_name = is_game_won)]
    pub fn is_game_won(&self) -> bool {
        self.game.game_won
    }
}
