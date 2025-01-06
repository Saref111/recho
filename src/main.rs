use std::env::args;

fn main() {
    let mut args = args();
    let (_, Some(to_print)) = (args.next(), args.next()) else {
        panic!("Must be at list one arg to echo it");
    };

    println!("echo: {to_print}");
}
