use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod setup_districts;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    districts: Vec<District>,
    tick: usize,
    panic_level: u8,
    is_game_over: bool,
}

impl Game {
    pub fn create_district(map: &mut Vec<District>, name: &str, neighbours: Vec<&str>) {
        map.push(District::new2(name, neighbours));
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    name: String,
    infected: f64,
    dead: f64,
    neighbours: Vec<String>,
    lambda: f64, //Wachstumsrate
}

impl District {
    pub fn new(name: impl Into<String>) -> District {
        District {
            name: name.into(),
            infected: 0.0,
            dead: 0.0,
            neighbours: Vec::new(),
            lambda: 0.0,
        }
    }

    pub fn new2(name: impl Into<String>, neighbours: Vec<impl Into<String>>) -> District {
        District {
            name: name.into(),
            infected: 0.0,
            dead: 0.0,
            neighbours: neighbours.into_iter().map(|w| w.into()).collect(),
            lambda: 0.0,
        }
    }

    pub fn set_random_lambda(&mut self) {
        use ::rand::prelude::*;
        let mut rng = rand::thread_rng();
        let l: f64 = rng.gen();

        self.lambda = l;
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Reactor {
    game: Game,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryDistrictDatasource;

impl MemoryDistrictDatasource {
    pub fn get_districts(&self) -> Vec<District> {
        setup_districts::setup_districts().into_iter().take(5).collect()
    }
}

impl Reactor {
    pub fn new(districts: Vec<District>) -> Self {
        Reactor {
            game: Game {
                districts: districts,
                tick: 0,
                panic_level: 0,
                is_game_over: false,
            },
        }
    }

    pub fn init(&mut self) {
        use ::rand::prelude::*;

        let number = 3;

        for _ in 0..number {
            let index: usize = rand::thread_rng().gen_range(0, self.game.districts.len());

            self.game.districts[index].infected = rand::thread_rng().gen_range(1, 20) as f64;
        }
    }
}

#[wasm_bindgen]
impl Reactor {
    pub fn tick(&mut self) {
        use ::rand::prelude::*;

        let mut d : Vec<&mut District>= self
            .game
            .districts
            .iter_mut()
            .filter(|w| w.infected > 0.0)
            .collect();

        //Increase
        let mut rng = rand::thread_rng();

        for mut di in d {
            //Lambda
            let k : f64 = rng.gen();
            di.lambda = k / 12.0;
            di.lambda = (di.lambda * 100.0).round() / 100.0;

            //Infected
            di.infected = (di.infected * std::f64::consts::E.powf(self.game.tick as f64)).round();

            //Dead
            let l: f64 = rng.gen();
            di.dead = (di.infected * l / 1000.0).round();
        }

        self.game.tick += 1;
    }

    pub fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
