use crate::state::{ApplyError, ApplyStatus};
use crate::{Executor, State, Target};

pub trait Compose: State + Sized {
    fn compose<R: State + Sized>(self, other: R) -> ComposedState<Self, R> {
        ComposedState {
            left: self,
            right: other,
        }
    }
}

pub struct ComposedState<L: State, R: State> {
    left: L,
    right: R,
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
