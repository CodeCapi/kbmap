extern crate wasm_bindgen;
extern crate web_sys;
extern crate itertools;
extern crate rand;

#[macro_use] extern crate impl_ops;

use wasm_bindgen::prelude::{Closure, JsValue, wasm_bindgen};
use wasm_bindgen::{JsCast};

use web_sys::{console, KeyboardEvent};
use std::{cell::RefCell, rc::Rc, };

mod vec2;
mod layout;
use layout::Layout;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
pub struct KeyPress {
    key: String,
    time: u32
}


pub type SharedVec = Rc<RefCell<Box<Vec<KeyPress>>>>;

fn get_time() -> u32 {
    let window = web_sys::window().unwrap();
    (window.performance().unwrap().now() * 1000.) as u32
}

fn _log_string(s: &String) {
    console::log_1(&JsValue::from_str(&s));
}


pub fn draw_layout(layout: &mut Layout) {
    let mut html_str: String = String::from("<div style=\"position: relative\">");
    {
        for body in layout.bodies.iter() {
            html_str += format!("<div style=\"position: absolute; left: {}px; top: {}px\">{}</div>", body.position.x, body.position.y, body.name).as_str();
        }
        html_str += "</div>";
    }

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    document.body().unwrap().set_inner_html(html_str.as_str());
}

pub fn gen_interval_closure(vec: SharedVec) -> Closure<dyn std::ops::FnMut()> {
    // let window = web_sys::window().unwrap();
    // let document = window.document().unwrap();
    let bound_vec = vec.clone();
    let mut layout = Layout::new();

    Closure::wrap(Box::new(move || {
        layout.update(&bound_vec.borrow());
        draw_layout(&mut layout);
    }))
}

pub fn gen_keypress_closure(vec: SharedVec) -> Closure<dyn std::ops::FnMut(KeyboardEvent)> {
    Closure::wrap(Box::new(move |_event: KeyboardEvent| {
        console::log_1(&JsValue::from_str("KB Event"));
        vec.borrow_mut().push(KeyPress{
            key: _event.key(),
            time: get_time()
        });
    }))
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();

    {
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.style().set_property("display", "flex")?;
        body.style().set_property("justify-content", "center")?;
        body.style().set_property("align-items", "center")?;
        body.style().set_property("align-items", "center")?;
        body.style().set_property("height", "100vh")?;
        body.style().set_property("width", "100vw")?;
    }

    let shared_v: SharedVec = Rc::new(RefCell::new(Box::new(vec![])));

    // document.body().unwrap().append_child(&document.create_element("div").unwrap());
    
    let interval_closure = gen_interval_closure(shared_v.clone());
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        interval_closure.as_ref().unchecked_ref(),
        100
    )?;
    interval_closure.forget();


    let keydown_closure = gen_keypress_closure(shared_v.clone());
    window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())?;
    keydown_closure.forget();

    Ok(())
}
