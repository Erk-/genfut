use clap::Parser;
use genfut::{genfut, Options};

fn main() {
    let opt = Options::parse();
    genfut(opt);
}
