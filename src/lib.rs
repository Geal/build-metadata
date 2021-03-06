//! use this crate to embed repository and build metadata at compile time
//!
//! This project uses procedural macros to provide you with this metadata,
//! exposing three macros:
//!
//! * `head!()`: the name of the current git branch or tag you are in
//! * `commit!()`: short git commit id
//! * `time!()`: UTC build time. This value is cached, so it will stay the same for every call in one crate. Note that using this makes the build non reproducible (since the resulting binary will change depending on build time)
//!
//! # Example
//!
//! ```rust
//! #![feature(plugin)]
//! #![plugin(build_metadata)]
//!
//! fn main() {
//!     println!("build time: {}", time!());
//!     println!("head: {}", head!());
//!     println!("commit id: {}", commit!());
//! }
//! ```
//!
#![feature(plugin_registrar, rustc_private)]

#[macro_use]
extern crate lazy_static;
extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;
extern crate git2;
extern crate time;

use rustc_plugin::Registry;
use syntax::tokenstream::TokenTree;
use syntax::codemap::Span;
use syntax::symbol::Symbol;
use syntax::ext::base::{self, ExtCtxt, MacResult};
use syntax::ext::build::AstBuilder;
use git2::{Repository, DescribeOptions};

lazy_static! {
    static ref METADATA: Metadata = Metadata::new();
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("commit", commit);
    reg.register_macro("head", head);
    reg.register_macro("time", current_time);
}

fn commit<'a>(cx: &'a mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'a> {
    base::check_zero_tts(cx, sp, tts, "commit!");

    let topmost = cx.expansion_cause().unwrap_or(sp);
    base::MacEager::expr(cx.expr_str(topmost, Symbol::intern(&METADATA.commit_short)))
}

fn head<'a>(cx: &'a mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'a> {
    base::check_zero_tts(cx, sp, tts, "head!");

    let topmost = cx.expansion_cause().unwrap_or(sp);
    base::MacEager::expr(cx.expr_str(topmost, Symbol::intern(&METADATA.head)))
}

fn current_time<'a>(cx: &'a mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'a> {
    base::check_zero_tts(cx, sp, tts, "time!");

    let topmost = cx.expansion_cause().unwrap_or(sp);
    base::MacEager::expr(cx.expr_str(topmost, Symbol::intern(&METADATA.time)))
}

struct Metadata {
    pub commit_short: String,
    pub head: String,
    pub time: String,
}

impl Metadata {
    pub fn new() -> Metadata {
        let repo = Repository::discover(".").expect("should find a repository");
        let head = repo.head()
            .expect("should find head")
            .shorthand()
            .expect("head name should be valid utf-8")
            .to_string();

        let desc = repo.describe(&DescribeOptions::new()
                .describe_tags()
                .show_commit_oid_as_fallback(true))
            .expect("should get repository description");

        let commit_oid = desc.format(None).unwrap_or(String::from("error"));

        Metadata {
            commit_short: commit_oid,
            head: head,
            time: time::now_utc().rfc3339().to_string(),
        }
    }
}
