mod with_local_reference;

use super::*;

#[test]
fn without_local_reference_errors_badarg() {
    without_info_without_local_reference_errors_badarg(file!(), options);
}

fn options(process: &Process) -> Term {
    process
        .cons(info_option(true, process), super::options(process))
        .unwrap()
}
