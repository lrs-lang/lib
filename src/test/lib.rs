#![crate_name = "test"]
#![crate_type = "rlib"]

use std::process::{fork, wait_id, ChildStatus, WAIT_EXITED, set_resource_limit};
use std::process::resource::{CoreDumpSize};

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
        let id = match fork(|| t.testfn.0()) {
            Ok(id) => id,
            Err(e) => {
                println!("could not fork ({:?})", e);
                break;
            }
        };
        match wait_id(id, WAIT_EXITED) {
            Ok(ChildStatus::Exited(0)) if !t.desc.should_panic.yes() => println!("ok"),
            Ok(ChildStatus::Dumped(4)) if t.desc.should_panic.yes() => println!("ok"),
            Ok(e) => println!("FAILURE ({:?})", e),
            Err(e) => println!("FAILURE ({:?})", e),
        };
    }
}
