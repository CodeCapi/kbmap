use itertools::Itertools;
use std::collections::HashMap;

use rand::prelude::*;

use crate::{vec2::Vec2, KeyPress};

#[derive(Debug)]
pub struct Body {
    pub name: String,
    pub position: Vec2,
    speed: Vec2,
    force: Vec2,
}

const DT: f32 = 0.01;

fn spring_force(body1: &Body, body2: &Body) -> Vec2 {
    let dx = body2.position - body1.position;
    let size = dx.size();

    if size == 0. {
        Vec2 {
            x: rand::random::<f32>() * 10.,
            y: random::<f32>() * 10.,
        }
    } else {
        dx.unit() * 1. * (size - 50.) * (size - 50.) * if size < 50. { -1. } else { 1. }
    }
}

fn repulse_force(body1: &Body, body2: &Body) -> Vec2 {
    let dx = body1.position - body2.position;
    let size = dx.size();

    if size == 0. {
        Vec2 {
            x: rand::random::<f32>() * 10.,
            y: random::<f32>() * 10.,
        }
    } else {
        1e7 * dx.unit() * (1. / (size * size))
    }
}

#[derive(Debug)]
pub struct Layout {
    pub bodies: Vec<Body>,
    name_map: HashMap<String, usize>,
}

impl Layout {
    pub fn new() -> Self {
        Layout {
            bodies: vec![],
            name_map: HashMap::new(),
        }
    }

    pub fn update(&mut self, keys: &Vec<KeyPress>) {
        self.update_forces(keys);
        self.update_speeds();
        self.update_positions();
    }

    fn try_insert(&mut self, key: String) -> Option<usize> {
        if !self.name_map.contains_key(key.as_str()) {
            let body = Body {
                name: key,
                force: Vec2::new(),
                speed: Vec2::new(),
                position: Vec2 {
                    x: 50. * rand::random::<f32>(),
                    y: 50. * rand::random::<f32>(),
                },
            };
            self.insert_body(body);
            Some(self.bodies.len() - 1)
        } else {
            None
        }
    }

    fn insert_body(&mut self, body: Body) {
        self.name_map.insert(body.name.clone(), self.bodies.len());
        self.bodies.push(body);
    }

    fn _update_forces_spring(&mut self) {
        let combinations = (0..self.bodies.len()).combinations(2);
        for index_pair in combinations {
            let (index1, index2) = (*index_pair.first().unwrap(), *index_pair.last().unwrap());
            let force1 = spring_force(
                self.bodies.get(index1).unwrap(),
                self.bodies.get(index2).unwrap(),
            );
            {
                let body1 = self.bodies.get_mut(index1).unwrap();
                body1.force = body1.force + force1;
            }
            {
                let body2 = self.bodies.get_mut(index2).unwrap();
                body2.force = body2.force - force1;
            }
        }
    }

    fn update_forces_repulse(&mut self) {
        let combinations = (0..self.bodies.len()).combinations(2);
        for index_pair in combinations {
            let (index1, index2) = (*index_pair.first().unwrap(), *index_pair.last().unwrap());
            let force1 = repulse_force(
                self.bodies.get(index1).unwrap(),
                self.bodies.get(index2).unwrap(),
            );
            {
                let body1 = self.bodies.get_mut(index1).unwrap();
                body1.force = body1.force + force1;
            }
            {
                let body2 = self.bodies.get_mut(index2).unwrap();
                body2.force = body2.force - force1;
            }
        }
    }

    fn update_forces_key_attract(&mut self, keys: &Vec<KeyPress>) {
        if keys.len() > 1 {
            for i in 0..(keys.len() - 1) {
                let key1 = keys.get(i).unwrap();
                let key2 = keys.get(i + 1).unwrap();
                let diff_time = key2.time - key1.time;
                let index1 = *self.name_map.get(&key1.key).unwrap();
                let index2 = *self.name_map.get(&key2.key).unwrap();
                if diff_time < 200000 {
                    let force1 = spring_force(
                        self.bodies.get(index1).unwrap(),
                        self.bodies.get(index2).unwrap(),
                    );
                    {
                        let body1 = self.bodies.get_mut(index1).unwrap();
                        body1.force = body1.force + force1;
                    }
                    {
                        let body2 = self.bodies.get_mut(index2).unwrap();
                        body2.force = body2.force - force1;
                    }
                }
            }
        }
    }

    fn update_forces(&mut self, keys: &Vec<KeyPress>) {
        for key_press in keys {
            self.try_insert(key_press.key.clone());
        }
        for body in self.bodies.iter_mut() {
            body.force = Vec2::new();
        }
        self.update_forces_repulse();
        self.update_forces_key_attract(keys);
    }
    fn update_speeds(&mut self) {
        for body in self.bodies.iter_mut() {
            body.speed.update(&((body.speed + body.force * DT) * 0.9));
        }
    }
    fn update_positions(&mut self) {
        for body in self.bodies.iter_mut() {
            body.position = body.position + body.speed * DT;
        }
    }
}
