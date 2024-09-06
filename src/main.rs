mod geometry;
mod scene;
mod utils;

use std::{env::args, fs::File, io::Read};

use scene::Scene;
use utils::InputData;

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("You should provide 1 argument to specify the path of input file");
        eprintln!("You should provide 2 argument to specify the path of output file");
        return;
    }
    let mut in_file = File::open(args[1].as_str()).unwrap();
    let mut out_file = File::create(args[2].as_str()).unwrap();
    let mut input_buf = String::new();
    in_file.read_to_string(&mut input_buf).unwrap();
    let data = InputData::parse(&input_buf).unwrap();
    let mut scene = Scene::new(data);
    scene.render(&mut out_file).unwrap();
}
