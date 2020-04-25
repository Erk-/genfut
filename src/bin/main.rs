use genfut::{genfut, Opt};
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    genfut(opt);
}
