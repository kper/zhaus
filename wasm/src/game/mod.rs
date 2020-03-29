use crate::console_log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod setup_districts;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Game {
    districts: Vec<District>,
    tick: usize,
    panic_level: u8,
    pub is_game_over: bool,
}

impl Game {
    pub fn create_district(map: &mut Vec<District>, name: &str, neighbours: Vec<&str>) {
        map.push(District::new2(name, neighbours));
    }

    pub fn get_districts(&self) -> &Vec<District> {
        &self.districts
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct District {
    name: String,
    pub infected: f64,
    pub dead: f64,
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

    pub fn get_name(&self) -> &String {
        &self.name
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
    map: geojson::GeoJson,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryDistrictDatasource;

impl MemoryDistrictDatasource {
    pub fn get_districts(&self) -> Vec<District> {
        setup_districts::setup_districts().into_iter().collect()
    }
}

impl Reactor {
    pub fn new(districts: Vec<District>) -> Self {
        use geojson::GeoJson;

        let geojson_str = include_str!("./../../bezirke.json");
        let mut geojson = geojson_str.parse::<GeoJson>().unwrap();

        Reactor {
            game: Game {
                districts: districts,
                tick: 0,
                panic_level: 0,
                is_game_over: false,
            },
            map: geojson
        }
    }

    pub fn get_game(&self) -> &Game {
        &self.game
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
        crate::utils::set_panic_hook();

        {
            let mut d: Vec<&mut District> = self
                .game
                .districts
                .iter_mut()
                .filter(|w| w.infected > 0.0)
                .collect();

            //Increase
            let mut rng = rand::thread_rng();

            for mut di in d.iter_mut() {
                //Lambda
                let k: f64 = rng.gen();
                di.lambda = k / 12.0;
                di.lambda = (di.lambda * 100.0).round() / 100.0;

                //Infected
                di.infected =
                    (di.infected * std::f64::consts::E.powf(self.game.tick as f64)).round();

                //Dead
                let l: f64 = rng.gen();
                di.dead = (di.infected * l / 1000.0).round(); //Sterblichkeit
            }
        }

        {
            let copy: Vec<_> = self
                .game
                .districts
                .iter()
                .filter(|w| w.infected > 0.0)
                .cloned()
                .collect::<Vec<_>>();

            let mut d: Vec<&mut District> = self.game.districts.iter_mut().collect();

            //console_log!("{:?}", d);

            //Spread to other areas
            for di in copy.iter() {
                //Iterating over copy while modifying the array
                console_log!("Checking district {}", di.name);

                let mut rng = thread_rng();
                let b: f64 = rng.gen();
                if rng.gen_bool(b) {
                    //10% change to spread for every district
                    let numbers: usize = rand::thread_rng().gen_range(0, di.neighbours.len());

                    let mut indices: Vec<_> = (0..di.neighbours.len()).map(|w| w).collect();
                    indices.shuffle(&mut rng);

                    for n in indices.into_iter().take(numbers) {
                        console_log!("Spreading to {:?}", di.neighbours[n]);

                        //TODO hashmap
                        let mut name_of_neighbour = d
                            .iter_mut()
                            .filter(|w| *w.get_name() == di.neighbours[n])
                            .collect::<Vec<_>>();

                        //console_log!("Spreading to {:?}", name_of_neighbour);

                        let mut w = name_of_neighbour
                            .get_mut(0)
                            .expect("Neighbour should exist"); //Reference by name

                        if w.infected > 0.0 {
                            //already infected
                            continue;
                        }

                        //Lambda
                        let k: f64 = rng.gen();
                        w.lambda = k / 12.0;
                        w.lambda = (w.lambda * 100.0).round() / 100.0;
                        w.infected = 1.0; //init

                        //Infected
                        w.infected =
                            (w.infected * std::f64::consts::E.powf(self.game.tick as f64)).round();
                    }
                }
            }
        }

        self.game.tick += 1;
    }

    pub fn get_overlay_infected(&self) -> Vec<JsValue> {
        crate::utils::set_panic_hook();

        let infected_districts: Vec<_> = self
            .get_game()
            .get_districts()
            .iter()
            //.filter(|w| w.infected > 0.0 || w.dead > 0.0)
            .map(|w| {
                //let js = serde_wasm_bindgen::to_value(w.get_name()).unwrap();
                let feature = self.get_feature_by_name(&w.name, w.infected, w.dead);

                feature
            })
            .map(|w| {
                return serde_wasm_bindgen::to_value(&w).unwrap();
            })
            .collect();

        infected_districts
    }

    pub fn action_quarantine(&mut self, _name: String) {
        crate::utils::set_panic_hook();

        console_log!("ahahhahaha");
    }

    fn get_feature_by_name(&self, name: &String, infected: f64, dead: f64) -> geojson::Feature {
        use geojson::GeoJson;
        use serde_json::to_value;

        //let name: String = serde_wasm_bindgen::from_value(name).unwrap();

        match self.map {
            GeoJson::FeatureCollection(ref ctn) => {
                for feature in &ctn.features {
                    let mut feature = feature.clone();
                    if let Some(ref mut props) = feature.properties {
                        let fname = match props.get("name").unwrap() {
                            serde_json::value::Value::String(ref s) => s,
                            _ => panic!("not ok"),
                        };

                        if fname == name {
                            props.insert(String::from("infected"), to_value(infected).unwrap());
                            props.insert(String::from("dead"), to_value(dead).unwrap());
                            return feature.clone();
                            //return serde_wasm_bindgen::to_value(&feature.clone()).unwrap();
                        }
                    }
                }
            }
            _ => panic!("not ok"),
        };

        panic!("feature not found {}", name);
    }

    pub fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
