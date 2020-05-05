use crate::state::{ApplyError, ApplyStatus};
use crate::{Executor, State, Target};

pub struct OnlyIfState<S: State, F: Fn(&dyn Target) -> bool> {
    inner: S,
    cond: F,
}
impl<S: State, F: Fn(&dyn Target) -> bool> OnlyIfState<S, F> {
    pub(crate) fn new(inner: S, cond: F) -> Self {
        Self { inner, cond }
    }
}

impl<S: State, F: Fn(&dyn Target) -> bool> State for OnlyIfState<S, F> {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        if (self.cond)(target) {
            self.inner.apply(executor, target)
        } else {
            Ok(ApplyStatus::NotChanged)
        }
    }

    fn name(&self) -> String {
        self.inner.name()
    }
}
