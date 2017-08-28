#![feature(plugin)]
#![plugin(build_metadata)]

fn main() {
    println!("build time: {}", time!());
    println!("head: {}", head!());
    println!("commit id: {}", commit!());
}
