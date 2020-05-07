#![allow(dead_code)]

mod executor;
mod graph;
mod state;
mod target;

pub use executor::Executor;
pub use state::State;
pub use state::StateExt;

pub use target::{Os, OsType, Target};

#[cfg(test)]
mod tests {

    use super::*;
    use crate::state::NOOP;

    struct TestTarget;
    impl Target for TestTarget {
        fn hostname(&self) -> &str {
            "foobar"
        }

        fn os(&self) -> &Os {
            // okay this is test code...
            Box::leak(Box::new(Os::from((OsType::Linux, "ubuntu", "16.04"))))
        }
    }

    struct TestExecutor;
    impl Executor for TestExecutor {}

    #[test]
    fn it_works() {
        let _target = TestTarget;
        let _executor = TestExecutor;

        let composed = NOOP.compose(NOOP);

        let onlyif = NOOP.only_if(|t| t.os().version() == "16.04");

        let _superset = composed.compose(onlyif);
    }
}
