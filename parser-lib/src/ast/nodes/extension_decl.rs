// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Expression, ExtensionDecl, SourceBuffer, StructEntry,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ExtensionDecl {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::extension_decl);
        let loc = From::from(&p.as_span());
        let mut inner = p.into_inner();
        let target = Expression::from_parse_tree(inner.next().expect("need identifier"), source);
        let mut body = vec![];
        for next in inner {
            let next = StructEntry::from_parse_tree(next, source);
            body.push(next);
        }
        Self {
            loc: source.pointer(loc),
            target,
            body,
        }
    }
}

impl PrettyPrintable for ExtensionDecl {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        (buffer << "extension " << &self.target).write_indented_list(&self.body, "{\n", "\n", "\n}")
    }
}
