use number_namer::parse;
use std::io::stdin;

fn main() {
    let mut buf = String::new();

    while let Ok(_) = stdin().read_line(&mut buf) {
        println!("{:?}", parse(&buf.to_lowercase()));
        buf.clear();
    }
}
