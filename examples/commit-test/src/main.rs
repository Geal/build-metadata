#![feature(plugin)]
#![plugin(build_metadata)]

fn main() {
    println!("head: {}", head!());
    println!("commit id: {}", commit!());
}
