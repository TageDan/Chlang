use std::{env::args, fs, io::Read};

use chlang::compile;

fn main() {
    let mut args = args();
    args.next();
    let file_path = args.next().unwrap();
    let mut buf = String::new();
    fs::File::open(file_path).unwrap().read_to_string(&mut buf);
    println!("{}", compile::compile(buf));
}
