use crate::state::{ApplyError, ApplyStatus};
use crate::{Executor, State, Target};

pub struct ComposedState<L: State, R: State> {
    left: L,
    right: R,
}

impl<L: State, R: State> ComposedState<L, R> {
    pub(crate) fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L: State, R: State> State for ComposedState<L, R> {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        let left = self.left.apply(executor, target);
        let right = self.right.apply(executor, target);
        // let's unwrap the errors
        Ok(left? + right?)
    }

    fn name(&self) -> String {
        format!("{} + {}", self.left.name(), self.right.name())
    }
}
