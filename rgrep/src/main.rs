mod check;

use structopt::StructOpt;
use check::*;

#[derive(Debug, StructOpt)]
#[structopt(name="line_match", about="get the text matched with the pattern")]
pub struct Opt {
    #[structopt(short="p", long="pattern")]
    pattern: String,

    #[structopt(short="f", long="file")]
    file_path: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    
    let q = Query::new("abc".into(), "test.txt".into());
    println!("q: {:?}", q);
}
