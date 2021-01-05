#[macro_use]
extern crate gluon_codegen;
#[macro_use]
extern crate gluon;
#[macro_use]
extern crate serde_derive;

use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use gluon::import::add_extern_module;
use gluon::{vm::ExternModule, Thread, ThreadExt};

// mod repl;

#[tokio::main]
async fn main() {
    let vm = gluon::new_vm();
    vm.get_database_mut().set_optimize(false);
    vm.run_io(true);
    add_extern_module(&vm, "test", load);
    let res = vm.run_expr::<()>("test", r#"
    let { E1, E2 } = import! test
    let e1 : E1 = E11 (Num 3)
    ()
    "#);
    // let res = repl::run(&vm, "> ").await;
    if let Err(e) = res {
        print_gluon_err(e);
    }
}

#[derive(Clone, Debug, VmType, Pushable, Getable)]
enum E1 {
    E11(E2),
}

#[derive(Clone, Debug, VmType, Pushable, Getable)]
enum E2 {
    Num(i32),
}

pub fn load(thread: &Thread) -> Result<ExternModule, gluon::vm::Error> {
    ExternModule::new(
        thread,
        record! {
            type E1 => E1,
            type E2 => E2,
        },
    )
}

pub fn print_gluon_err(e: gluon::Error) {
    e.emit(&mut StandardStream::stderr(ColorChoice::Always))
        .unwrap();
}
