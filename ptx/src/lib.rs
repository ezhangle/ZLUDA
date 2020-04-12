#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate lalrpop_util;
extern crate rspirv;
extern crate spirv_headers as spirv;

lalrpop_mod!(ptx);

mod test;
mod translate;
pub mod ast;

pub use ast::Module as Module;
pub use translate::to_spirv as to_spirv;

pub(crate) fn without_none<T>(x: Vec<Option<T>>) -> Vec<T> {
    x.into_iter().filter_map(|x| x).collect()
}