mod geometry;
mod scene;
mod utils;

use std::{
    env::{self, args},
    fs::File,
    io::Read,
};

use scene::Scene;
use utils::InputData;

fn main() {
    let vars = env::vars().collect::<Vec<_>>();
    if vars.iter().any(|s| s.0 == "SERIALIZE") {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .unwrap();
    }
    let args = args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("You should provide 1st argument to specify the path of input file");
        eprintln!("You should provide 2nd argument to specify the path of output file");
        eprintln!("You can optionally provide 3rd argument to specify the antialiasing level");
        return;
    }
    let mut in_file = File::open(args[1].as_str()).unwrap();
    let mut out_file = File::create(args[2].as_str()).unwrap();
    let mut input_buf = String::new();
    in_file.read_to_string(&mut input_buf).unwrap();
    let data = InputData::parse(&input_buf).unwrap();
    let mut scene = Scene::new_with_antialiasing(
        data,
        if args.len() < 4 {
            1
        } else {
            args[3]
                .parse::<u8>()
                .expect("fail to parse given antialiasing level")
        },
    );
    scene.render(&mut out_file).unwrap();
}
