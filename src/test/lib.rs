#![crate_name = "test"]
#![crate_type = "rlib"]

use std::process::{fork, wait_id, ChildStatus, WAIT_EXITED, set_resource_limit, self};
use std::process::resource::{CoreDumpSize};
use std::signal::{signals, SigHandler, Sigset, Signal, SigInfo, set_handler};
use std::signal::flags::{SA_NONE};

pub struct StaticTestName(pub &'static str);

pub struct StaticTestFn(pub fn());

pub enum ShouldPanic {
    No,
    Yes,
    YesWithMessage(&'static str)
}

impl ShouldPanic {
    fn yes(&self) -> bool {
        match *self {
            ShouldPanic::No => false,
            _ => true,
        }
    }
}

pub struct TestDesc {
    pub name: StaticTestName,
    pub ignore: bool,
    pub should_panic: ShouldPanic,
}

pub struct TestDescAndFn {
    pub desc: TestDesc,
    pub testfn: StaticTestFn,
}

pub fn test_main_static(tests: &[TestDescAndFn]) {
    // disable core-dumps
    set_resource_limit(0, CoreDumpSize, 0, 0).unwrap();

    for t in tests {
        print!("testing {} ... ", t.desc.name.0);

        let id = match fork(|| {
            extern fn abort_handler(_: Signal, _: &SigInfo, _: usize) {
                process::exit(1);
            }

            set_handler(signals::Illegal, Sigset::new(), SigHandler::Func(abort_handler),
                        SA_NONE);

            t.testfn.0()
        }) {
            Ok(id) => id,
            Err(e) => {
                println!("could not fork ({:?})", e);
                break;
            }
        };

        match wait_id(id, WAIT_EXITED) {
            Ok(ChildStatus::Exited(0)) if !t.desc.should_panic.yes() => println!("ok"),
            Ok(ChildStatus::Exited(1)) if t.desc.should_panic.yes() => println!("ok"),
            Ok(e) => println!("FAILURE ({:?})", e),
            Err(e) => println!("FAILURE ({:?})", e),
        };
    }
}
