use std::path::PathBuf;

use structopt::StructOpt;

use genfut::genfut;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "genfut",
    about = "Generates rust code to interface with generated futhark code."
)]
struct Opt {
    /// Output dir
    #[structopt(name = "NAME")]
    name: String,

    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    genfut(opt.name, opt.file);
}
