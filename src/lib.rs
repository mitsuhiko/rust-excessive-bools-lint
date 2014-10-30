#![feature(macro_rules, plugin_registrar, quote, phase)]

#[phase(plugin, link)]
extern crate syntax;
#[phase(plugin, link)]
extern crate rustc;

use syntax::ast;
use rustc::lint::LintPassObject;
use rustc::plugin::Registry;
use rustc::lint::{Context, LintPass, LintArray};
use rustc::middle::def;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::visit::FnKind;

declare_lint!(EXCESSIVE_BOOL_USAGE_STRUCTS, Warn,
              "Warn about exessive use of boolean members in structs.")
declare_lint!(EXCESSIVE_BOOL_USAGE_FUNCS, Warn,
              "Warn about exessive use of boolean arguments in functions.")

struct Pass;

fn node_is_bool(cx: &Context, ty: &ast::Ty) -> bool {
    match ty.node {
        ast::TyPath(_, _, id) => {
            match cx.tcx.def_map.borrow().get_copy(&id) {
                def::DefPrimTy(ast::TyBool) => true,
                _ => false,
            }
        },
        _ => false,
    }
}

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(EXCESSIVE_BOOL_USAGE_STRUCTS, EXCESSIVE_BOOL_USAGE_FUNCS)
    }

    fn check_struct_def(&mut self, cx: &Context, def: &ast::StructDef,
            _i: ast::Ident, _gen: &ast::Generics, id: ast::NodeId) {
        let mut bools = vec![];
        let mut spans = vec![];

        for field in def.fields.iter() {
            if node_is_bool(cx, &*field.node.ty) {
                match field.node {
                    ast::StructField_ { kind: ast::NamedField(ident, _), .. } => {
                        spans.push(field.span);
                        bools.push(token::get_ident(ident).get().to_string())
                    },
                    _ => {},
                }
            }
        }

        if bools.len() >= 3 {
            cx.span_lint(EXCESSIVE_BOOL_USAGE_STRUCTS, cx.tcx.map.span(id),
                format!("Struct contains an excessive number ({}) of bools ({}).  \
                    Did you want to create a state machine?",
                    bools.len(),
                    bools.connect(", ")).as_slice());
            for span in spans.iter() {
                cx.tcx.sess.span_note(*span, "boolean field defined here");
            }
        }
    }

    fn check_fn(&mut self, cx: &Context, _: FnKind, decl: &ast::FnDecl, _: &ast::Block, sp: Span, _: ast::NodeId) {
        let mut spans = vec![];
        for arg in decl.inputs.iter() {
            if node_is_bool(cx, &*arg.ty) {
                spans.push(cx.tcx.map.span(arg.id));
            }
        }

        if spans.len() >= 3 {
            cx.span_lint(EXCESSIVE_BOOL_USAGE_FUNCS, sp,
                format!("Funtion contains an excessive number ({}) of bools.  \
                    Perhaps you should use enumerated arguments?",
                    spans.len()).as_slice());
            for span in spans.iter() {
                cx.tcx.sess.span_note(*span, "boolean argument defined here");
            }
        }
    }
}


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box Pass as LintPassObject);
}
