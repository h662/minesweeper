// src/input.rs

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlCanvasElement, MouseEvent, TouchEvent};

#[derive(Clone)]
pub struct InputHandler {
    clicks: Rc<RefCell<Vec<(f64, f64)>>>,
}

impl InputHandler {
    pub fn new(canvas: &HtmlCanvasElement) -> InputHandler {
        let clicks = Rc::new(RefCell::new(Vec::new()));
        let click_ref = clicks.clone();
        let on_click = Closure::wrap(Box::new(move |event: MouseEvent| {
            let rect = event
                .target()
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap()
                .get_bounding_client_rect();
            click_ref.borrow_mut().push((
                event.client_x() as f64 - rect.left(),
                event.client_y() as f64 - rect.top(),
            ));
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())
            .unwrap();
        on_click.forget();

        let touch_ref = clicks.clone();
        let on_touch = Closure::wrap(Box::new(move |event: TouchEvent| {
            let touch = event.touches().get(0).unwrap();
            let rect = event
                .target()
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap()
                .get_bounding_client_rect();
            touch_ref.borrow_mut().push((
                touch.client_x() as f64 - rect.left(),
                touch.client_y() as f64 - rect.top(),
            ));
        }) as Box<dyn FnMut(_)>);
        on_touch.forget();

        InputHandler { clicks }
    }
    
    pub fn take_clicks(&self) -> Vec<(f64, f64)> {
        self.clicks.borrow_mut().drain(..).collect()
    }
}
