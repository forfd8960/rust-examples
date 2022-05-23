mod check;

use std::collections::HashMap;
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
    
    let q = Query::new(opt.pattern.into(), opt.file_path.into());
    println!("q: {:?}", q);

    print_result(q);
}

fn print_result<T: Match>(q: T) {
    let result = match q.get_matched_lines() {
        Ok(v) => v,
        Err(e) => {
            println!("has err: {:?}", e);
            let res = HashMap::new();
            res
        }
    };

    for (file, lines) in result.into_iter() {
        println!("> {:?}", file);
        for line in lines {
            println!(">>>> {:?}", line);
        }
    }
}