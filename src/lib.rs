#![allow(dead_code)]

mod executor;
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
        let target = TestTarget;
        let executor = TestExecutor;

        let composed = NOOP.compose(NOOP);

        let depends_of_composed = NOOP.depends_on(&composed);
        let also_depends_of_composed = NOOP.depends_on(&composed);

        let root = composed
            .compose(depends_of_composed)
            .compose(also_depends_of_composed);
    }
}
