use wasm_bindgen::prelude::*;
use rand;

mod utils;
mod game;

use game::{MemoryDistrictDatasource, Game};

pub use game::Reactor;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

#[wasm_bindgen]
pub fn get_reactor() -> Reactor {
    utils::set_panic_hook();

    let mem = MemoryDistrictDatasource;

    let districts = mem.get_districts();

    let mut reactor = Reactor::new(districts);

    reactor.init();

    reactor
}
