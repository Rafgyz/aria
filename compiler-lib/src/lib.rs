// SPDX-License-Identifier: Apache-2.0
use aria_parser::ast::{ParsedModule, SourceBuffer};
use do_compile::{CompilationError, CompilationResult};
use module::CompiledModule;

pub mod bc_reader;
pub mod bc_writer;
pub mod constant_value;
pub mod do_compile;
pub mod dump;
pub mod func_builder;
pub mod line_table;
pub mod module;
pub mod scope;

pub struct CompilationOptions {
    pub optimize: bool,
}

impl Default for CompilationOptions {
    fn default() -> Self {
        Self { optimize: true }
    }
}

pub fn compile_from_source(
    src: &SourceBuffer,
    options: &CompilationOptions,
) -> CompilationResult<CompiledModule, Vec<CompilationError>> {
    do_compile::compile_from_source(src, options)
}

pub fn compile_from_ast(
    ast: &ParsedModule,
    options: &CompilationOptions,
) -> CompilationResult<CompiledModule, Vec<CompilationError>> {
    do_compile::compile_from_ast(ast, options)
}
