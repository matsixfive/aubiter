use crate::vector3::Vector3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    // #[wasm_bindgen(js_namespace = console)]
    // fn time(s: &str);
    // #[wasm_bindgen(js_namespace = console)]
    // fn timeEnd(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[derive(Clone, Debug)]
pub struct Planet {
    pub name: String,
    pub mass: f64,
    pub radius: f64,

    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    planets: Vec<Planet>,
    speed: u32,
}

impl Universe {
    const G: f64 = 6.67e-11;

    fn tick_once(&mut self) {
        // reset acceleration
        self.planets.iter_mut().for_each(|planet| planet.acceleration = Vector3::new(0.0, 0.0, 0.0));

        // calculate new acceleration
        for i in 0..self.planets.len() {
            for j in (i + 1)..self.planets.len() {
                let a = &self.planets[i];
                let b = &self.planets[j];

                let mut a_to_b = b.position - a.position;

                let distance_sqr = a_to_b.magnitude_sqr();
                if distance_sqr == 0.0 {
                    continue;
                }

                a_to_b.normalize();

                let accel_mag = a_to_b * (Self::G / distance_sqr);

                let a_accel = accel_mag * a.mass; // F = G * (m1 * m2) / r^2
                let b_accel = -accel_mag * b.mass; // F = G * (m1 * m2) / r^2

                self.planets[i].acceleration += a_accel;
                self.planets[j].acceleration += b_accel;
            }
        }

        // calculate new velocity and position
        self.planets.iter_mut().for_each(|planet| {
            planet.velocity += planet.acceleration;
            planet.position += planet.velocity;
        });

        // check for collisions
        let mut to_remove: Vec<usize> = vec![];
        for i in 0..self.planets.len() {
            for j in (i + 1)..self.planets.len() {
                let a = &self.planets[i];
                let b = &self.planets[j];

                let distance_sqr = (b.position - a.position).magnitude_sqr();
                let radius = a.radius + b.radius;

                // console_log!("{} {} {} {}", a.name, b.name, distance_sqr, radius * radius);

                if distance_sqr < radius * radius {
                    // add to `remove` list in sorted order (no duplicates)
                    match to_remove.binary_search(&i) {
                        Ok(_) => {}
                        Err(pos) => to_remove.insert(pos, i),
                    }
                    match to_remove.binary_search(&j) {
                        Ok(_) => {}
                        Err(pos) => to_remove.insert(pos, j),
                    }
                }
            }
        }

        // remove collided planets
        to_remove.sort(); // should already be sorted, but just in case
        to_remove.iter().rev().for_each(|&i| {
            self.planets.remove(i);
        });
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for _ in 0..self.speed {
            self.tick_once();
        }
    }

    pub fn new() -> Universe {
        Universe {
            planets: vec![],
            speed: 1,
        }
    }

    pub fn add_planet(&mut self, name: String, pos: Vector3, vel: Vector3, mass: f64, radius: f64) {
        let planet = Planet {
            name,
            mass,
            radius,
            position: pos,
            velocity: vel,
            acceleration: Vector3::new(0.0, 0.0, 0.0),
        };
        self.planets.push(planet);
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn render(&self) -> Output {
        let mut output = Output { planets: vec![] };

        for i in 0..self.planets.len() {
            let planet = &self.planets[i];
            let out_planet = OutPlanet {
                name: planet.name.clone(),
                radius: planet.radius,
                mass: planet.mass,
                velocity: planet.velocity,
                position: planet.position,
            };
            output.planets.push(out_planet);
        }

        output
    }

    pub fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[wasm_bindgen]
#[repr(C)]
pub struct Output {
    planets: Vec<OutPlanet>,
}

#[wasm_bindgen]
impl Output {
    pub fn get_planet(&self, index: usize) -> OutPlanet {
        self.planets[index].clone()
    }

    pub fn length(&self) -> usize {
        self.planets.len()
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
#[derive(Clone)]
pub struct OutPlanet {
    name: String,
    radius: f64,
    mass: f64,
    velocity: Vector3,
    position: Vector3,
}

#[wasm_bindgen]
impl OutPlanet {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn velocity(&self) -> Vector3 {
        self.velocity
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }
}
