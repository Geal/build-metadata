#![feature(plugin_registrar, rustc_private)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::tokenstream::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, DummyResult, MacResult};
use syntax::print::pprust::tt_to_string;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("commit", commit);
}

fn commit<'a>(cx: &'a mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'a> {
    base::check_zero_tts(cx, sp, tts, "commit!");

    let topmost = cx.expansion_cause().unwrap_or(sp);
    let loc = cx.codemap().lookup_char_pos(topmost.lo);
    base::MacEager::expr(cx.expr_str(topmost, Symbol::intern(&"pouet")))
}
