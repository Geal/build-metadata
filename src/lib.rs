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
    base::MacEager::expr(cx.expr_str(topmost, Symbol::intern(&METADATA.commit_oid)))
}

struct Metadata {
    pub commit_oid: String,
}

impl Metadata {
    pub fn new() -> Metadata {
        let commit_oid = Repository::discover(".")
            .and_then(|repo| {
                repo.describe(&DescribeOptions::new()
                        .describe_tags()
                        .show_commit_oid_as_fallback(true))
                    .and_then(|desc| desc.format(None))
            })
            .unwrap_or(String::from("error"));

        Metadata { commit_oid: commit_oid }
    }
}
