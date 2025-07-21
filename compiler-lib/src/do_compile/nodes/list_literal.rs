// SPDX-License-Identifier: Apache-2.0
use crate::{
    do_compile::{
        CompilationError, CompilationErrorReason, CompilationResult, CompileNode, CompileParams,
    },
    func_builder::BasicBlockOpcode,
};

impl<'a> CompileNode<'a> for aria_parser::ast::ListLiteral {
    fn do_compile(&self, params: &'a mut CompileParams) -> CompilationResult {
        let count = self
            .items
            .expressions
            .iter()
            .map(|arg| arg.do_compile(params))
            .count();

        if count > u32::MAX as usize {
            Err(CompilationError {
                loc: self.loc.clone(),
                reason: CompilationErrorReason::ListTooLarge,
            })
        } else {
            params
                .writer
                .get_current_block()
                .write_opcode_and_source_info(
                    BasicBlockOpcode::BuildList(count as u32),
                    self.loc.clone(),
                );
            Ok(())
        }
    }
}
