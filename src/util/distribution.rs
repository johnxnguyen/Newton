use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use geometry::types::Point;
use geometry::types::Vector;
use geometry::util::Transformation;
use physics::types::Mass;
use util::gens::Generator;
use util::gens::MassGen;
use util::gens::RadialGen;
use util::gens::Repeater;
use util::gens::RotationGen;
use util::gens::UniformGen;
use util::gens::VelocityGen;

pub struct Loader {
    mass_gens:      HashMap<String, MassGen>,
    distance_gens:  HashMap<String, UniformGen>,
    velocity_gens:  HashMap<String, VelocityGen>,
    rotation_gens:  HashMap<String, RotationGen>,
    radials_gens:   HashMap<String, RadialGen>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            mass_gens:      HashMap::new(),
            distance_gens:  HashMap::new(),
            velocity_gens:  HashMap::new(),
            rotation_gens:  HashMap::new(),
            radials_gens:   HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
        let docs = Loader::docs(path);
        let doc = &docs[0];

        // should define error type for useful feedback

        // need to give back errors instead of unwrapping

        // this could be refactored into a method
        let gens = doc["gens"].as_vec().unwrap();

        for gen in gens {
            let name = gen["name"].as_str().unwrap().to_owned();
            let gen_type = gen["type"].as_str().unwrap();

            match gen_type {
                "mass" => {
                    self.mass_gens.insert(name, Loader::parse_mass_gen(gen));
                },
                "distance" => {
                    self.distance_gens.insert(name, Loader::parse_distance_gen(gen));
                },
                "velocity" => {
                    self.velocity_gens.insert(name, Loader::parse_velocity_gen(gen));
                },
                "rotation" => {
                    self.rotation_gens.insert(name, Loader::parse_rotation_gen(gen));
                },
                "radial" => {
                    self.radials_gens.insert(name, Loader::parse_radial_gen(gen));
                },
                _ => panic!("Unknown generator type: {:?}", gen_type),
            };
        }

        println!("mass gens: {:?}", self.mass_gens.len());
        println!("dist gens: {:?}", self.distance_gens.len());
        println!("vel gens: {:?}", self.velocity_gens.len());
        println!("rot gens: {:?}", self.rotation_gens.len());
        println!("radials gens: {:?}", self.radials_gens.len());

        // now we create body nodes
        let bods = doc["bodies"].as_vec().unwrap();

        for bod in bods {
            let name = bod["name"].as_str().unwrap().to_owned();
            // this should be positive
            let num = bod["num"].as_i64().unwrap();
        }
    }

    fn docs(path: &str) -> Vec<Yaml> {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        YamlLoader::load_from_str(&contents).unwrap()
    }

    fn parse_mass_gen(gen: &Yaml) -> MassGen {
        let low = gen["low"].as_f64().unwrap() as f32;
        let high = gen["high"].as_f64().unwrap() as f32;
        MassGen::new(low, high)
    }

    fn parse_distance_gen(gen: &Yaml) -> UniformGen {
        let dist_min = gen["dist"]["min"].as_i64().unwrap() as f32;
        let dist_max = gen["dist"]["max"].as_i64().unwrap() as f32;
        UniformGen::new(dist_min, dist_max)
    }

    fn parse_rotation_gen(gen: &Yaml) -> RotationGen {
        let low = gen["low"].as_f64().unwrap() as f32;
        let high = gen["high"].as_f64().unwrap() as f32;
        RotationGen::new_degrees(low, high)
    }

    fn parse_velocity_gen(gen: &Yaml) -> VelocityGen {
        let vel_min = gen["vel"]["min"].as_f64().unwrap() as f32;
        let vel_max = gen["vel"]["max"].as_f64().unwrap() as f32;
        VelocityGen::new(0.0, 0.0, vel_min, vel_max)
    }

    fn parse_radial_gen(gen: &Yaml) -> RadialGen {
        let distance = Loader::parse_distance_gen(gen);
        let rotation = RotationGen::new_degrees(0.0, 360.0);
        let velocity = Loader::parse_velocity_gen(gen);
        RadialGen::new(distance, rotation, velocity)
    }

//    fn parse_bod(bod: &Yaml) -> (String, Vec<Node>) {
//        let name = bod["name"].as_str().unwrap();
//        let num = bod["num"].as_i64().unwrap_or(1);
//
//        let mut nodes: Vec<Node> = vec![];
//
//        // actually, it makes sense to make all of these gens, because
//        // we don't want to parse this body over and over. If mass is a
//        // hard value, make that a repetitive gen.
//
//        // I would need to make sure that I could use the gens on
//        // separate threads.
//
//        // Also, how would the gen look like as a trait? It would be
//        // generic surely, meaning it would have it's own associated
//        // type. But the gen isn't a generic type.
//
//        // this can also be a gen
//        let mass = bod["mass"].as_f64().unwrap();
//
//        // how to handle missing keys and default values?
//
//        let trans = match bod["trans"].as_str() {
//            Some(gen_name) => {
//                // lookup gen here
//                Point::new(0.0, 0.0)
//            },
//            None => {
//                let x = bod["trans"]["x"].as_i64().unwrap() as f32;
//                let y  = bod["trans"]["y"].as_i64().unwrap() as f32;
//                Point::new(x, y)
//            },
//        };
//
//        let vel = match bod["vel"].as_str() {
//            Some(gen_name) => {
//                // lookup gen here
//                Vector::new(0.0, 0.0)
//            },
//            None => {
//                let dx = bod["vel"]["dx"].as_f64().unwrap() as f32;
//                let dy = bod["vel"]["dy"].as_f64().unwrap() as f32;
//                Vector::new(dx, dy)
//            },
//        };
//
//        let rot = match bod["rot"].as_str() {
//            Some(gen_name) => {
//                // lookup gen here
//                0.0
//            },
//            None => {
//                bod["rot"].as_f64().unwrap()
//            },
//        };
//
//        // make the nodes here
//    }
}

//////////////////////////////////////////////////////////////////////////////

//enum Node {
//    // translation, velocity, subsystems
//    System(Point, Vector, Vec<Index>),
//    // position, velocity, mass
//    Body(Point, Vector, f32),
//}
//
//type Index = u32;
//
//struct DistributionTree {
//    nodes: Vec<Index>
//}
//
//impl DistributionTree {
//    fn new() -> DistributionTree {
//        DistributionTree { nodes: vec![] }
//    }
//}
