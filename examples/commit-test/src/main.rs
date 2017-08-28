#![feature(plugin)]
#![plugin(build_metadata)]

fn main() {
    println!("commit id: {}", commit!());
}
