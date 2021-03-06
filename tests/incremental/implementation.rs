use crate::counter::Counter;
use crate::log::Log;
use crate::memoized_dep_inputs;
use crate::memoized_inputs;
use crate::memoized_volatile;

crate trait TestContext: salsa::QueryContext {
    fn clock(&self) -> &Counter;
    fn log(&self) -> &Log;
}

#[derive(Default)]
crate struct TestContextImpl {
    runtime: salsa::runtime::Runtime<TestContextImpl>,
    clock: Counter,
    log: Log,
}

impl TestContextImpl {
    crate fn assert_log(&self, expected_log: &[&str]) {
        use difference::{Changeset, Difference};

        let expected_text = &format!("{:#?}", expected_log);
        let actual_text = &format!("{:#?}", self.log().take());

        if expected_text == actual_text {
            return;
        }

        let Changeset { diffs, .. } = Changeset::new(expected_text, actual_text, "\n");

        for i in 0..diffs.len() {
            match &diffs[i] {
                Difference::Same(x) => println!(" {}", x),
                Difference::Add(x) => println!("+{}", x),
                Difference::Rem(x) => println!("-{}", x),
            }
        }

        panic!("incorrect log results");
    }
}

salsa::query_context_storage! {
    crate struct TestContextImplStorage for TestContextImpl {
        impl memoized_dep_inputs::MemoizedDepInputsContext {
            fn dep_memoized2() for memoized_dep_inputs::Memoized2;
            fn dep_memoized1() for memoized_dep_inputs::Memoized1;
            fn dep_derived1() for memoized_dep_inputs::Derived1;
            fn dep_input1() for memoized_dep_inputs::Input1;
            fn dep_input2() for memoized_dep_inputs::Input2;
        }

        impl memoized_inputs::MemoizedInputsContext {
            fn max() for memoized_inputs::Max;
            fn input1() for memoized_inputs::Input1;
            fn input2() for memoized_inputs::Input2;
        }

        impl memoized_volatile::MemoizedVolatileContext {
            fn memoized2() for memoized_volatile::Memoized2;
            fn memoized1() for memoized_volatile::Memoized1;
            fn volatile() for memoized_volatile::Volatile;
        }
    }
}

impl TestContext for TestContextImpl {
    fn clock(&self) -> &Counter {
        &self.clock
    }

    fn log(&self) -> &Log {
        &self.log
    }
}

impl salsa::QueryContext for TestContextImpl {
    fn salsa_runtime(&self) -> &salsa::runtime::Runtime<TestContextImpl> {
        &self.runtime
    }
}
