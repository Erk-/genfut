use genfut::{genfut, Opt};
use clap::Parser;

fn main() {
    let opt = Opt::parse();
    genfut(opt);
}
