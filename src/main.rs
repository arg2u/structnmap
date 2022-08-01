use std::path::PathBuf;

use structnmap::{Data, Error};
use structopt;
use structopt::StructOpt;

#[allow(unused)]
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Structnmap",
    about = "This is a tool for parsing nmap xml and structing it to files by service name."
)]
struct Opt {
    /// Nmap xml file path
    #[structopt(parse(from_os_str))]
    xml: PathBuf,
    /// Output directory
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}
fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let output = opt.output;
    let xml = opt.xml;
    let test = Data::build(xml.to_str().unwrap())?;
    test.generate(output.to_str().unwrap())?;
    Ok(())
}
