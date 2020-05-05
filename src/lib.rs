#![allow(dead_code)]

mod executor;
mod state;
mod target;

pub use executor::Executor;
pub use state::condition::CondState;
pub use state::graph::GraphState;
pub use state::State;

pub use target::{Os, OsType, Target};

#[cfg(test)]
mod tests {

    use super::*;

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

        /* // just try some constructions
        let root = GraphState::new("root", |b| {
            b.add(state::NOOP);
            b.add(
                CondState::new()
                    .apply_if(state::NOOP, |t| {
                        t.os().vendor() == "ubuntu" && t.os().version() == "16.04"
                    })
                    .apply_if(state::NOOP, |t| {
                        t.os().vendor() == "ubuntu" && t.os().version() == "18.04"
                    }),
            );
            let some_config_file = b.add(state::NOOP);

            b.add(state::NOOP).depends_on(some_config_file);
        });*/
    }
}
