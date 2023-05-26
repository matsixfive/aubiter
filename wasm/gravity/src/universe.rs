use vector3::Vector3;
use wasm_bindgen::prelude::*;

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
    // width: u32,
    // height: u32,
    planets: Vec<Planet>,
    edges: Vec<Edge>,
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
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for i in 0..self.edges.len() {
            let a = &self.planets[self.edges[i].a];
            let b = &self.planets[self.edges[i].b];

            let a_to_b = b.position - a.position;
            let b_to_a = a.position - b.position;

            let distance_sqr = a_to_b.x * a_to_b.x + a_to_b.y * a_to_b.y + a_to_b.z * a_to_b.z;

            // let force = (a_to_b / distance_sqr.sqrt()) * ((a.mass * b.mass * 6e-11) / distance_sqr); // F = G * (m1 * m2) / r^2
            let a_accel = a_to_b.normalize() * ((b.mass * 6e-11) / distance_sqr); // F = G * (m1 * m2) / r^2
            let b_accel = b_to_a.normalize() * ((a.mass * 6e-11) / distance_sqr); // F = G * (m1 * m2) / r^2

            // let a_accel = force / a.mass;
            // let b_accel = force / -b.mass;

            self.planets[self.edges[i].a].acceleration =
                self.planets[self.edges[i].a].acceleration + a_accel * 1000.0;
            self.planets[self.edges[i].b].acceleration =
                self.planets[self.edges[i].b].acceleration + b_accel * 1000.0;
        }

        for i in 0..self.planets.len() {
            let planet = &mut self.planets[i];
            planet.velocity = planet.velocity + planet.acceleration ;
            planet.position = planet.position + planet.velocity;
            planet.acceleration = Vector3::new(0.0, 0.0, 0.0);
        }
    }

    pub fn new() -> Universe {
        Universe {
            planets: vec![],
            edges: vec![],
        }
    }

    pub fn add_planet(&mut self, name: String, pos_x: f64, pos_y: f64, pos_z: f64, vel_x: f64, vel_y: f64, vel_z: f64, mass: f64, radius: f64) {
        let planet = Planet {
            name,
            mass,
            radius,
            position: Vector3::new(pos_x, pos_y, pos_z),
            velocity: Vector3::new(vel_x, vel_y, vel_z),
            acceleration: Vector3::new(0.0, 0.0, 0.0),
        };
        self.planets.push(planet);
        self.build_edges();
    }

    pub fn render(&self) -> Output {
        let mut output = Output { planets: vec![] };

        for i in 0..self.planets.len() {
            let planet = &self.planets[i];
            let out_planet = OutPlanet {
                name: planet.name.clone(),
                radius: planet.radius,
                x: planet.position.x,
                y: planet.position.y,
                z: planet.position.z,
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
    x: f64,
    y: f64,
    z: f64,
}

#[wasm_bindgen]
impl OutPlanet {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}
