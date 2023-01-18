use clap::Parser;
use genfut::{genfut, Opt};

fn main() {
    let opt = Opt::parse();
    genfut(opt);
}
