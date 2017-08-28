#![feature(plugin_registrar, rustc_private)]

#[macro_use]
extern crate lazy_static;
extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;
extern crate git2;

use rustc_plugin::Registry;
use syntax::tokenstream::TokenTree;
use syntax::codemap::Span;
use syntax::symbol::Symbol;
use syntax::ext::base::{self, ExtCtxt, DummyResult, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::print::pprust::tt_to_string;
use git2::{Repository, DescribeOptions};

lazy_static! {
    static ref METADATA: Metadata = Metadata::new();
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("commit", commit);
}

fn commit<'a>(cx: &'a mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'a> {
    base::check_zero_tts(cx, sp, tts, "commit!");

    let topmost = cx.expansion_cause().unwrap_or(sp);
    let loc = cx.codemap().lookup_char_pos(topmost.lo);
    base::MacEager::expr(cx.expr_str(topmost,
                                     Symbol::intern(&format!("{}-{}",
                                                             METADATA.head,
                                                             METADATA.commit_short))))
}

struct Metadata {
    pub commit_short: String,
    pub head: String,
}

impl Metadata {
    pub fn new() -> Metadata {
        let repo = Repository::discover(".").expect("should find a repository");
        let head = repo.head()
            .expect("should find head")
            .shorthand()
            .expect("head name should be valid utf-8")
            .to_string();

        let desc =
            repo.describe(&DescribeOptions::new()
                    .describe_tags()
                    .show_commit_oid_as_fallback(true))
                .expect("should get repository description");
        let commit_oid = desc.format(None).unwrap_or(String::from("error"));
        Metadata {
            commit_short: commit_oid,
            head: head,
        }
    }
}
