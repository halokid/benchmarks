extern crate serde;
extern crate serde_json;
extern crate json_rs;

use json_rs::Skip;
use std::fs::File;
use std::io::Read;
use std::path::Path;


include!(concat!(env!("OUT_DIR"), "/serde_types_struct.rs"));


fn main() {
    let path = Path::new("./1.json");
    let mut s = Vec::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_end(&mut s).unwrap();

    let jobj: TestStruct = serde_json::de::from_slice(&s).unwrap();

    let len = jobj.coordinates.len() as f64;
    let mut x = 0_f64;
    let mut y = 0_f64;
    let mut z = 0_f64;

    for coord in jobj.coordinates.iter() {
        x += coord.x;
        y += coord.y;
        z += coord.z;
    }

    println!("{}", x / len);
    println!("{}", y / len);
    println!("{}", z / len);
}
