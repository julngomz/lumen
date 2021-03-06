// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

use native_implemented_function::native_implemented_function;

use crate::erlang::list_to_string::list_to_string;
use crate::erlang::string_to_integer::base_string_to_integer;

#[native_implemented_function(list_to_integer/2)]
pub fn native(process: &Process, list: Term, base: Term) -> exception::Result<Term> {
    let string: String = list_to_string(list)?;

    base_string_to_integer(process, base, "list", '\'', &string).map_err(From::from)
}
