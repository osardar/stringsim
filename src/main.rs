use std::env;
use stringsim;
use stringsim::TwoWayCmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Insufficient args");
        return;
    }

    let x = stringsim::Hamming::new(&args[1][..], &args[2][..]);
    println!("Ham: {}", x.cmp());

    let x = stringsim::Levenshtein::new(&args[1][..], &args[2][..]);
    println!("Lev: {}", x.cmp());

    let x = stringsim::Jaccard::new(&args[1][..], &args[2][..]);
    println!("Jac: {}", x.cmp());
}