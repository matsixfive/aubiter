// use vector3::Vector3;
use crate::vector3::Vector3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
    edges: Vec<Edge>,
    speed: u32,
}

impl Universe {
    fn build_edges(&mut self) {
        for i in 0..self.planets.len() {
            for j in 0..self.planets.len() {
                if i > j {
                    self.edges.push(Edge { a: i, b: j });
                }
            }
        }
    }

    fn tick_once(&mut self) {
        for i in 0..self.planets.len() {
            self.planets[i].acceleration = Vector3::new(0.0, 0.0, 0.0);
        }

        for i in 0..self.edges.len() {
            let a = &self.planets[self.edges[i].a];
            let b = &self.planets[self.edges[i].b];

            let mut a_to_b = b.position - a.position;
            // let b_to_a = a.position - b.position;

            let distance_sqr = a_to_b.magnitude_sqr();

            a_to_b.normalize();

            // log(&format!("{:?} {:?}", a_to_b, a_to_b.magnitude()));

            let a_accel = a_to_b * (b.mass * (6.67e-11 / distance_sqr)); // F = G * (m1 * m2) / r^2
            let b_accel = -a_to_b * (a.mass * (6.67e-11 / distance_sqr)); // F = G * (m1 * m2) / r^2

            self.planets[self.edges[i].a].acceleration += a_accel;
            self.planets[self.edges[i].b].acceleration += b_accel;
        }

        for i in 0..self.planets.len() {
            let planet = &mut self.planets[i];
            planet.velocity += planet.acceleration;
            planet.position += planet.velocity;
        }
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
            edges: vec![],
            speed: 1,
        }
    }

    pub fn add_planet(
        &mut self,
        name: String,
        pos: Vector3,
        vel: Vector3,
        mass: f64,
        radius: f64,
    ) {
        let planet = Planet {
            name,
            mass,
            radius,
            position: pos,
            velocity: vel,
            acceleration: Vector3::new(0.0, 0.0, 0.0),
        };
        self.planets.push(planet);
        self.build_edges();
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

#[derive(Debug)]
struct Edge {
    a: usize,
    b: usize,
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
